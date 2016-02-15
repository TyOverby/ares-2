mod error;
mod emit_buffer;

use compiler::parse::Ast;
use compiler::binding::{Bound, SymbolBindSource, LambdaBindings};
use compiler::CompileContext;
use vm::{Instr, ClosureClass};

pub use self::error::EmitError;
pub use self::emit_buffer::EmitBuffer;

#[allow(unused_variables)]
pub fn emit<'bound, 'ast: 'bound>(ast: &'bound Bound<'bound, 'ast>,
                    compile_context: &mut CompileContext,
                    out: &mut EmitBuffer,
                    inside_lambda: Option<&LambdaBindings>)
                    -> Result<(), EmitError> {
    match ast {
        &Bound::Block(ref bound_bodies, _) => {
            for body in &bound_bodies[..bound_bodies.len() - 1] {
                try!(emit(body, compile_context, out, inside_lambda));
                out.push(Instr::Pop);
            }
            if let Some(last_body) = bound_bodies.last() {
                try!(emit(last_body, compile_context, out, inside_lambda));
            }
        }
        &Bound::Add(ref l, ref r, _) => {
            try!(emit(l, compile_context, out, inside_lambda));
            try!(emit(r, compile_context, out, inside_lambda));
            out.push(Instr::AddInt);
        }
        &Bound::Sub(ref l, ref r, _) => {
            try!(emit(l, compile_context, out, inside_lambda));
            try!(emit(r, compile_context, out, inside_lambda));
            out.push(Instr::SubInt);
        }
        &Bound::Mul(ref l, ref r, _) => {
            try!(emit(l, compile_context, out, inside_lambda));
            try!(emit(r, compile_context, out, inside_lambda));
            out.push(Instr::MulInt);
        }
        &Bound::Div(ref l, ref r, _) => {
            try!(emit(l, compile_context, out, inside_lambda));
            try!(emit(r, compile_context, out, inside_lambda));
            out.push(Instr::DivInt);
        }
        &Bound::Literal(ast) => {
            match ast {
                &Ast::IntLit(i, _) => {
                    use std::i32::{MIN, MAX};
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
                _ => panic!("non-literal ast found in Bound::Literal"),
            }
        }
        &Bound::IfExpression(ref cond, ref tru, ref fals, _) => {
            let mut true_code = EmitBuffer::new();
            let mut false_code = EmitBuffer::new();

            try!(emit(&**cond, compile_context, out, inside_lambda));

            out.push(Instr::Ifn);
            let (false_pos, fulfill_false) = out.standin();
            out.push_standin(false_pos);

            try!(emit(&**tru, compile_context, &mut true_code, inside_lambda));
            try!(emit(&**fals, compile_context, &mut false_code, inside_lambda));

            // The true branch needs to jump past the end
            // of the false branch.
            let end = out.len() + (true_code.len() + 1) + false_code.len();
            true_code.push(Instr::Jump(end as u32));

            out.merge(true_code);
            let len_with_true_code = out.len();
            out.fulfill(fulfill_false, Instr::Jump(len_with_true_code as u32));
            out.merge(false_code);
        }
        &Bound::IfStatement(_, _, _, _) => unimplemented!(),
        &Bound::Lambda { ref arg_symbols, ref body, ref bindings, ..} => {
            const INSTRS_BEFORE_LAMBDA_CODE: u32 = 2;
            let prior_code_len = out.len();

            let closure_class = ClosureClass {
                    code_offset: out.len() as u32 + INSTRS_BEFORE_LAMBDA_CODE,
                    // TODO: take varargs into account
                    arg_count: arg_symbols.len() as u32,
                    local_defines_count: bindings.num_declarations,
                    upvars_count: bindings.num_upvars,
                    has_rest_params: false,
            };

            let cc_id = compile_context.add_closure_class(closure_class);

            out.push(Instr::CreateClosure(cc_id));
            // TODO: load closure with upvars

            // Standin for the end of the lambda code.
            let (eol_standin, eol_fulfill) = out.standin();
            out.push_standin(eol_standin);

            debug_assert_eq!(prior_code_len + INSTRS_BEFORE_LAMBDA_CODE as usize,
                             out.len());

            try!(emit(body, compile_context, out, Some(bindings)));
            out.push(Instr::Ret);

            let next = out.len() as u32;
            out.fulfill(eol_fulfill, Instr::Jump(next));
        }
        &Bound::FnCall(ref funclike, ref args, _) => {
            for arg in args {
                try!(emit(arg, compile_context, out, inside_lambda));
            }
            try!(emit(funclike, compile_context, out, inside_lambda));
            out.push(Instr::Execute(args.len() as u32));
        }
        &Bound::Symbol { symbol, ast, source, } => {
            if let SymbolBindSource::Global(symbol) = source {
                out.push(Instr::GetGlobal(symbol));
            } else {
                let binder = inside_lambda.unwrap();
                out.push(Instr::Dup(binder.compute_stack_offset(source)));
            }
        }
        &Bound::Define(_, source, value, _) => {
            if let SymbolBindSource::Global(_) = source {
                unimplemented!();
            } else {
                let binder = inside_lambda.unwrap();
                try!(emit(value, compile_context, out, inside_lambda));
                out.push(Instr::DupTop);
                out.push(Instr::Assign(binder.compute_stack_offset(source)));
            }
        }
        &Bound::ListLit(_, _) |
        &Bound::MapLit(_, _) => unimplemented!(),
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use vm::{Instr, Value};
    use compiler::parse::{Ast, Span};
    use compiler::CompileContext;
    use ares_syntax::{SymbolIntern};
    use typed_arena::Arena;

    fn compile_this(ast: &Ast, interner: Option<SymbolIntern>) -> (Vec<Instr>, CompileContext) {
        use compiler::emit::emit;
        use compiler::emit::EmitBuffer;
        use compiler::binding::Bound;

        let mut out = EmitBuffer::new();
        let mut compile_context = CompileContext::new();
        let mut bound_arena = Arena::new();
        let mut interner = interner.unwrap_or_else(|| SymbolIntern::new());

        let bound = Bound::bind_top(ast, &mut bound_arena, &mut interner).unwrap();
        emit(&bound, &mut compile_context, &mut out, None).unwrap();
        (out.into_instructions(), compile_context)
    }

    #[test]
    fn test_literal_emit() {
        {
            let ast = Ast::IntLit(5, Span::dummy());
            let (out, _) = compile_this(&ast, None);
            assert_eq!(out, vec![Instr::IntLit(5)]);
        }
        {
            let ast = Ast::IntLit(8589934592, Span::dummy());
            let (out, cc) = compile_this(&ast, None);
            assert_eq!(cc.get_constant(0), Value::Int(8589934592));
            assert_eq!(out, vec![Instr::LoadConstant(0)]);

        }
        {
            let ast = Ast::FloatLit(3.14, Span::dummy());
            let (out, cc) = compile_this(&ast, None);
            assert_eq!(cc.get_constant(0), Value::Float(3.14));
            assert_eq!(out, vec![Instr::LoadConstant(0)]);
        }
        {
            let ast = Ast::StringLit("hello world".into(), Span::dummy());
            let (out, cc) = compile_this(&ast, None);
            assert_eq!(cc.get_constant(0), "hello world".into());
            assert_eq!(out, vec![Instr::LoadConstant(0)]);
        }
    }

    #[test]
    fn test_add_emit() {
        let a = Arena::new();
        {
            // Add one thing
            let ast = Ast::Add(vec![a.alloc(Ast::IntLit(5, Span::dummy()))], Span::dummy());
            let (out, _) = compile_this(&ast, None);
            assert_eq!(out, vec![Instr::IntLit(5)]);
        }
        {
            // Add two things
            let ast = Ast::Add(vec![a.alloc(Ast::IntLit(5, Span::dummy())),
                                    a.alloc(Ast::IntLit(10, Span::dummy()))],
                               Span::dummy());
            let (out, _) = compile_this(&ast, None);
            assert_eq!(out,
                       vec![Instr::IntLit(5), Instr::IntLit(10), Instr::AddInt]);
        }
        {
            // Add some addition
            let ast = Ast::Add(vec![a.alloc(Ast::IntLit(5, Span::dummy())),
                                    a.alloc(Ast::IntLit(10, Span::dummy())),
                                    a.alloc(Ast::Add(vec![
                                        a.alloc(Ast::IntLit(15, Span::dummy())),
                                        a.alloc(Ast::IntLit(20, Span::dummy())),
                                        ],
                                                     Span::dummy()))],
                               Span::dummy());
            let (out, _) = compile_this(&ast, None);
            assert_eq!(out,
                       vec![Instr::IntLit(5),
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
        let a = Arena::new();
        let ast = Ast::If(a.alloc(Ast::BoolLit(true, Span::dummy())),
                          a.alloc(Ast::IntLit(15, Span::dummy())),
                          a.alloc(Ast::IntLit(20, Span::dummy())),
                          Span::dummy());

        let (out, _) = compile_this(&ast, None);

        assert_eq!(out,
                   vec![Instr::BoolLit(true),
                        Instr::Ifn,
                        Instr::Jump(5),
                        Instr::IntLit(15),
                        Instr::Jump(6),
                        Instr::IntLit(20)]);
    }

    #[test]
    fn emit_no_arg_lambda() {
        let a = Arena::new();
        let ast = Ast::Lambda(vec![],
                              a.alloc(Ast::Block(vec![a.alloc(Ast::IntLit(10, Span::dummy())),
                                   a.alloc(Ast::IntLit(5, Span::dummy()))], Span::dummy())),
                              Span::dummy());

        let (out, _) = compile_this(&ast, None);

        assert_eq!(out, vec![
                   Instr::CreateClosure(0),
                   Instr::Jump(6),
                   Instr::IntLit(10),
                   Instr::Pop,
                   Instr::IntLit(5),
                   Instr::Ret]);
    }

    #[test]
    fn emit_list() {
        let a = Arena::new();
        let ast = Ast::List(vec![
                a.alloc(Ast::IntLit(1, Span::dummy())),
                a.alloc(Ast::IntLit(2, Span::dummy())),
                a.alloc(Ast::IntLit(3, Span::dummy())),
            ],
                            Span::dummy());

        let (out, _) = compile_this(&ast, None);

        assert_eq!(out,
                   vec![Instr::IntLit(2),
                        Instr::IntLit(3),
                        Instr::IntLit(1),
                        Instr::Execute(2)]);
    }

    #[test]
    fn emit_one_arg_lambda() {
        let a = Arena::new();
        let mut interner = SymbolIntern::new();

        let arg1 = interner.intern("test");
        let ast = Ast::Lambda(vec![arg1],
                              a.alloc(Ast::Block(vec![a.alloc(Ast::Symbol(arg1, Span::dummy()))], Span::dummy())),
                              Span::dummy());

        let (out, _) = compile_this(&ast, Some(interner));

        assert_eq!(out, vec![
                   Instr::CreateClosure(0),
                   Instr::Jump(4),
                   Instr::Dup(0),
                   Instr::Ret]);
    }
}
