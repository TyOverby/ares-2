mod error;
mod emit_buffer;

use compiler::parse::Ast;
use compiler::CompileContext;
use vm::Instr;

pub use self::error::EmitError;
pub use self::emit_buffer::EmitBuffer;

pub fn emit(ast: &Ast, compile_context: &mut CompileContext, out: &mut EmitBuffer) {
    match ast {
        &Ast::Add(ref operands, _) => {
            for operand in &operands[..] {
                emit(operand, compile_context, out);
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
        &Ast::SymbolLit(s, _) => {
            out.push(Instr::SymbolLit(s));
        }
        &Ast::FloatLit(f, _) => {
            out.push(compile_context.add_constant(f.into()));
        }

        &Ast::If(ref cond, ref tru, ref fals, _) => {
            let mut true_code = EmitBuffer::new();
            let mut false_code = EmitBuffer::new();

            emit(&**cond, compile_context, out);

            out.push(Instr::Ifn);
            let (false_pos, fulfill_false) = out.standin();
            out.push_standin(false_pos);

            emit(&**tru, compile_context, &mut true_code);
            emit(&**fals, compile_context, &mut false_code);

            // The true branch needs to jump past the end
            // of the false branch.
            let end = out.len() + (true_code.len() + 1) + false_code.len();
            true_code.push(Instr::Jump(end as u32));

            out.merge(true_code);
            let len_with_true_code = out.len();
            out.fulfill(fulfill_false, Instr::Jump(len_with_true_code as u32));
            out.merge(false_code);
        }
        _ => unimplemented!()
    }
}

#[test]
fn test_literal_emit() {
    use compiler::parse::Span;
    use vm::Value;

    {
        // Inlieable int
        let mut out = EmitBuffer::new();
        let mut compile_context = CompileContext::new();
        let ast = Ast::IntLit(5, Span::dummy());
        emit(&ast, &mut compile_context, &mut out);
        let out = out.into_instructions();
        assert!(out.len() == 1);
        assert_eq!(Instr::IntLit(5), out[0]);
    } {
        // constant int
        let mut out = EmitBuffer::new();
        let mut compile_context = CompileContext::new();
        let ast = Ast::IntLit(8589934592, Span::dummy());
        emit(&ast, &mut compile_context, &mut out);
        let out = out.into_instructions();
        assert!(out.len() == 1);
        assert_eq!(Instr::LoadConstant(0), out[0]);
        assert_eq!(compile_context.get_constant(0), Value::Int(8589934592))
    } {
        // constant float
        let mut out = EmitBuffer::new();
        let mut compile_context = CompileContext::new();
        let ast = Ast::FloatLit(3.14, Span::dummy());
        emit(&ast, &mut compile_context, &mut out);
        let out = out.into_instructions();
        assert!(out.len() == 1);
        assert_eq!(Instr::LoadConstant(0), out[0]);
        assert_eq!(compile_context.get_constant(0), Value::Float(3.14))
    } {
        // string float
        let mut out = EmitBuffer::new();
        let mut compile_context = CompileContext::new();
        let ast = Ast::StringLit("hello world".into(), Span::dummy());
        emit(&ast, &mut compile_context, &mut out);
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
        emit(&ast, &mut compile_context, &mut out);
        let out = out.into_instructions();
        assert!(out.len() == 1);
        assert_eq!(Instr::IntLit(5), out[0]);
    } {
        // Add two things
        let mut out = EmitBuffer::new();
        let mut compile_context = CompileContext::new();
        let ast = Ast::Add(vec![
                           Ast::IntLit(5, Span::dummy()),
                           Ast::IntLit(10, Span::dummy())
                           ], Span::dummy());
        emit(&ast, &mut compile_context, &mut out);
        let out = out.into_instructions();
        assert_eq!(vec![Instr::IntLit(5), Instr::IntLit(10), Instr::AddInt], out);
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
        emit(&ast, &mut compile_context, &mut out);
        let out = out.into_instructions();
        assert_eq!(vec![
                   Instr::IntLit(5),
                   Instr::IntLit(10),
                   Instr::IntLit(15),
                   Instr::IntLit(20),
                   Instr::AddInt,
                   Instr::AddInt,
                   Instr::AddInt], out);
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
    emit(&ast, &mut compile_context, &mut out);
    let out = out.into_instructions();
    assert!(out.len() == 6);
    assert_eq!(vec![
               Instr::BoolLit(true),
               Instr::Ifn,
               Instr::Jump(5),
               Instr::IntLit(15),
               Instr::Jump(6),
               Instr::IntLit(20)], out);
}
