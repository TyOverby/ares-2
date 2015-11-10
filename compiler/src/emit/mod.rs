mod error;

use parse::Ast;
use ares_vm::Instr;

pub use self::error::EmitError;

pub fn emit(ast: &Ast, out: &mut Vec<Instr>) {
    match ast {
        &Ast::Add(ref operands, _) => {
            for operand in &operands[..] {
                emit(operand, out);
            }
            for _ in 0 .. operands.len() - 1 {
                out.push(Instr::AddInt);
            }
        }
        &Ast::IntLit(i, _) => {
            out.push(Instr::IntLit(i as i32))
        }
        _ => unimplemented!()
    }
}

#[test]
fn test_int_lit_emit() {
    use parse::Span;

    let mut out = vec![];
    let ast = Ast::IntLit(5, Span::dummy());
    emit(&ast, &mut out);
    assert!(out.len() == 1);
    assert_eq!(Instr::IntLit(5), out[0]);
}

#[test]
fn test_add_emit() {
    use parse::Span;

    // Add a single item
    let mut out = vec![];
    let ast = Ast::Add(vec![Ast::IntLit(5, Span::dummy())], Span::dummy());
    emit(&ast, &mut out);
    assert!(out.len() == 1);
    assert_eq!(Instr::IntLit(5), out[0]);

    // Add two things
    let mut out = vec![];
    let ast = Ast::Add(vec![
                       Ast::IntLit(5, Span::dummy()),
                       Ast::IntLit(10, Span::dummy())
                       ], Span::dummy());
    emit(&ast, &mut out);
    assert_eq!(vec![Instr::IntLit(5), Instr::IntLit(10), Instr::AddInt], out);
}
