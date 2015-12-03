mod error;
mod emit_buffer;

use compiler::parse::Ast;
use compiler::CompileContext;
use vm::{Instr, ClosureClass};

pub use self::error::EmitError;
pub use self::emit_buffer::EmitBuffer;

pub fn emit(ast: &Ast, compile_context: &mut CompileContext, out: &mut EmitBuffer) -> Result<(), EmitError> {
    match ast {
        &Ast::Add(ref operands, _) => {
            for operand in &operands[..] {
                try!(emit(operand, compile_context, out));
            }
            if operands.len() == 0 {
                out.push(Instr::IntLit(0));
            } else {
                for _ in 0 .. operands.len() - 1 {
                    out.push(Instr::AddInt);
                }
            }
        }
        &Ast::IntLit(i, _) => {
            use ::std::i32::{MIN, MAX};
            if i >= MIN as i64 && i <= MAX as i64 {
                out.push(Instr::IntLit(i as i32));
            } else {
                out.push(compile_context.add_constant(i.into()));
            }
        }
        &Ast::BoolLit(b, _) => {
            out.push(Instr::BoolLit(b));
        }
        &Ast::StringLit(ref s, _) => {
            out.push(compile_context.add_constant(s.clone().into()));
        }
        &Ast::FloatLit(f, _) => {
            out.push(compile_context.add_constant(f.into()));
        }

        &Ast::If(ref cond, ref tru, ref fals, _) => {
            let mut true_code = EmitBuffer::new();
            let mut false_code = EmitBuffer::new();

            try!(emit(&**cond, compile_context, out));

            out.push(Instr::Ifn);
            let (false_pos, fulfill_false) = out.standin();
            out.push_standin(false_pos);

            try!(emit(&**tru, compile_context, &mut true_code));
            try!(emit(&**fals, compile_context, &mut false_code));

            // The true branch needs to jump past the end
            // of the false branch.
            let end = out.len() + (true_code.len() + 1) + false_code.len();
            true_code.push(Instr::Jump(end as u32));

            out.merge(true_code);
            let len_with_true_code = out.len();
            out.fulfill(fulfill_false, Instr::Jump(len_with_true_code as u32));
            out.merge(false_code);
        }
        &Ast::Lambda(ref argslist, ref bodies, _) => {
            const INSTRS_BEFORE_LAMBDA_CODE: u32 = 3;
            let prior_code_len = out.len();

            let closure_class = ClosureClass {
                    code_offset: out.len() as u32 + INSTRS_BEFORE_LAMBDA_CODE,
                    // TODO: take varargs into account
                    arg_count: argslist.len() as u32,
                    has_rest_params: false
            };

            let cc_id = compile_context.add_closure_class(closure_class);

            out.push(Instr::CreateClosure(cc_id));
            // TODO: load closure with upvars
            out.push(Instr::LoadClosure(0));

            // Standin for the end of the lambda code.
            let (eol_standin, eol_fulfill) = out.standin();
            out.push_standin(eol_standin);

            debug_assert_eq!(prior_code_len + INSTRS_BEFORE_LAMBDA_CODE as usize, out.len());
            for body in &bodies[.. bodies.len() - 1] {
                try!(emit(body, compile_context, out));
                out.push(Instr::Pop);
            }
            if let Some(last_body) = bodies.last() {
                try!(emit(last_body, compile_context, out));
            }
            out.push(Instr::Ret);

            let next = out.len() as u32;
            out.fulfill(eol_fulfill, Instr::Jump(next));
        }
        &Ast::List(ref elements, _) => {
            if elements.len() == 0 {
                return Err(EmitError::CallWithEmptyList);
            }
            let funclike = &elements[0];
            let args = &elements[1 ..];
            for arg in args {
                try!(emit(arg, compile_context, out));
            }
            try!(emit(funclike, compile_context, out));
            out.push(Instr::ExecuteClosure(args.len() as u32));
        }
        _ => unimplemented!()
    }
    Ok(())
}

/*
#[test]
fn test_literal_emit() {
    use compiler::parse::Span;
    use vm::Value;

    {
        // Inlieable int
        let mut out = EmitBuffer::new();
        let mut compile_context = CompileContext::new();
        let ast = Ast::IntLit(5, Span::dummy());
        emit(&ast, &mut compile_context, &mut out).unwrap();
        let out = out.into_instructions();
        assert!(out.len() == 1);
        assert_eq!(Instr::IntLit(5), out[0]);
    } {
        // constant int
        let mut out = EmitBuffer::new();
        let mut compile_context = CompileContext::new();
        let ast = Ast::IntLit(8589934592, Span::dummy());
        emit(&ast, &mut compile_context, &mut out).unwrap();
        let out = out.into_instructions();
        assert!(out.len() == 1);
        assert_eq!(Instr::LoadConstant(0), out[0]);
        assert_eq!(compile_context.get_constant(0), Value::Int(8589934592))
    } {
        // constant float
        let mut out = EmitBuffer::new();
        let mut compile_context = CompileContext::new();
        let ast = Ast::FloatLit(3.14, Span::dummy());
        emit(&ast, &mut compile_context, &mut out).unwrap();
        let out = out.into_instructions();
        assert!(out.len() == 1);
        assert_eq!(Instr::LoadConstant(0), out[0]);
        assert_eq!(compile_context.get_constant(0), Value::Float(3.14))
    } {
        // string float
        let mut out = EmitBuffer::new();
        let mut compile_context = CompileContext::new();
        let ast = Ast::StringLit("hello world".into(), Span::dummy());
        emit(&ast, &mut compile_context, &mut out).unwrap();
        let out = out.into_instructions();
        assert!(out.len() == 1);
        assert_eq!(Instr::LoadConstant(0), out[0]);
        assert_eq!(compile_context.get_constant(0), "hello world".into())
    }
}

#[test]
fn test_add_emit() {
    use compiler::parse::Span;

    {
        // Add a single item
        let mut out = EmitBuffer::new();
        let mut compile_context = CompileContext::new();
        let ast = Ast::Add(vec![Ast::IntLit(5, Span::dummy())], Span::dummy());
        emit(&ast, &mut compile_context, &mut out).unwrap();
        let out = out.into_instructions();
        assert!(out.len() == 1);
        assert_eq!(out[0], Instr::IntLit(5));
    } {
        // Add two things
        let mut out = EmitBuffer::new();
        let mut compile_context = CompileContext::new();
        let ast = Ast::Add(vec![
                           Ast::IntLit(5, Span::dummy()),
                           Ast::IntLit(10, Span::dummy())
                           ], Span::dummy());
        emit(&ast, &mut compile_context, &mut out).unwrap();
        let out = out.into_instructions();
        assert_eq!(out, vec![Instr::IntLit(5), Instr::IntLit(10), Instr::AddInt]);
    } {
        // Add some addition
        let mut out = EmitBuffer::new();
        let mut compile_context = CompileContext::new();
        let ast = Ast::Add(vec![
                           Ast::IntLit(5, Span::dummy()),
                           Ast::IntLit(10, Span::dummy()),
                           Ast::Add(vec![
                                    Ast::IntLit(15, Span::dummy()),
                                    Ast::IntLit(20, Span::dummy()),
                                    ], Span::dummy())
                           ], Span::dummy());
        emit(&ast, &mut compile_context, &mut out).unwrap();
        let out = out.into_instructions();
        assert_eq!(out, vec![
                   Instr::IntLit(5),
                   Instr::IntLit(10),
                   Instr::IntLit(15),
                   Instr::IntLit(20),
                   Instr::AddInt,
                   Instr::AddInt,
                   Instr::AddInt]);
    }
}

#[test]
fn test_basic_if() {
    use compiler::parse::Span;

    // Add a single item
    let mut out = EmitBuffer::new();
    let mut compile_context = CompileContext::new();
    let ast = Ast::If(
        Box::new(Ast::BoolLit(true, Span::dummy())),
        Box::new(Ast::IntLit(15, Span::dummy())),
        Box::new(Ast::IntLit(20, Span::dummy())),
        Span::dummy());
    emit(&ast, &mut compile_context, &mut out).unwrap();
    let out = out.into_instructions();

    assert_eq!(out, vec![
               Instr::BoolLit(true),
               Instr::Ifn,
               Instr::Jump(5),
               Instr::IntLit(15),
               Instr::Jump(6),
               Instr::IntLit(20)]);
}

#[test]
fn emit_no_arg_lambda() {
    use compiler::parse::Span;

    let mut out = EmitBuffer::new();
    let mut compile_context = CompileContext::new();
    let ast = Ast::Lambda(
        vec![],
        vec![Ast::IntLit(10, Span::dummy()), Ast::IntLit(5, Span::dummy())],
        Span::dummy());
    emit(&ast, &mut compile_context, &mut out).unwrap();
    let out = out.into_instructions();

    assert_eq!(out, vec![
               Instr::CreateClosure(0),
               Instr::LoadClosure(0),
               Instr::Jump(7),
               Instr::IntLit(10),
               Instr::Pop,
               Instr::IntLit(5),
               Instr::Ret]);
}

#[test]
fn emit_list() {
    use compiler::parse::Span;

    let mut out = EmitBuffer::new();
    let mut compile_context = CompileContext::new();
    let ast = Ast::List(
        vec![Ast::IntLit(1, Span::dummy()), Ast::IntLit(2, Span::dummy()), Ast::IntLit(3, Span::dummy())],
        Span::dummy());
    emit(&ast, &mut compile_context, &mut out).unwrap();
    let out = out.into_instructions();

    assert_eq!(out, vec![
               Instr::IntLit(2),
               Instr::IntLit(3),
               Instr::IntLit(1),
               Instr::ExecuteClosure(2)]);
}*/
