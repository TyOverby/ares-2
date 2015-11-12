mod error;
mod emit_buffer;

use compiler::parse::Ast;
use vm::Instr;

pub use self::error::EmitError;
pub use self::emit_buffer::EmitBuffer;

pub fn emit(ast: &Ast, out: &mut EmitBuffer) {
    match ast {
        &Ast::Add(ref operands, _) => {
            for operand in &operands[..] {
                emit(operand, out);
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
            out.push(Instr::IntLit(i as i32));
        }
        &Ast::BoolLit(b, _) => {
            out.push(Instr::BoolLit(b));
        }
        &Ast::SymbolLit(s, _) => {
            out.push(Instr::SymbolLit(s));
        }
        &Ast::If(ref cond, ref tru, ref fals, _) => {
            let mut true_code = EmitBuffer::new();
            let mut false_code = EmitBuffer::new();

            emit(&**cond, out);

            out.push(Instr::Ifn);
            let (false_pos, fulfill_false) = out.standin();
            out.push_standin(false_pos);

            emit(&**tru, &mut true_code);
            emit(&**fals, &mut false_code);

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
fn test_int_lit_emit() {
    use compiler::parse::Span;

    let mut out = EmitBuffer::new();
    let ast = Ast::IntLit(5, Span::dummy());
    emit(&ast, &mut out);
    let out = out.into_instructions();
    assert!(out.len() == 1);
    assert_eq!(Instr::IntLit(5), out[0]);
}

#[test]
fn test_add_emit() {
    use compiler::parse::Span;

    // Add a single item
    let mut out = EmitBuffer::new();
    let ast = Ast::Add(vec![Ast::IntLit(5, Span::dummy())], Span::dummy());
    emit(&ast, &mut out);
    let out = out.into_instructions();
    assert!(out.len() == 1);
    assert_eq!(Instr::IntLit(5), out[0]);

    // Add two things
    let mut out = EmitBuffer::new();
    let ast = Ast::Add(vec![
                       Ast::IntLit(5, Span::dummy()),
                       Ast::IntLit(10, Span::dummy())
                       ], Span::dummy());
    emit(&ast, &mut out);
    let out = out.into_instructions();
    assert_eq!(vec![Instr::IntLit(5), Instr::IntLit(10), Instr::AddInt], out);

    // Add some addition
    let mut out = EmitBuffer::new();
    let ast = Ast::Add(vec![
                       Ast::IntLit(5, Span::dummy()),
                       Ast::IntLit(10, Span::dummy()),
                       Ast::Add(vec![
                            Ast::IntLit(15, Span::dummy()),
                            Ast::IntLit(20, Span::dummy()),
                            ], Span::dummy())
                       ], Span::dummy());
    emit(&ast, &mut out);
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

#[test]
fn test_basic_if() {
    use compiler::parse::Span;

    // Add a single item
    let mut out = EmitBuffer::new();
    let ast = Ast::If(
        Box::new(Ast::BoolLit(true, Span::dummy())),
        Box::new(Ast::IntLit(15, Span::dummy())),
        Box::new(Ast::IntLit(20, Span::dummy())),
        Span::dummy());
    emit(&ast, &mut out);
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
