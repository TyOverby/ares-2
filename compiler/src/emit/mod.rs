use parse::Ast;
use ares_vm::Instr;

fn emit(ast: &Ast, out: &mut Vec<Instr>) {
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

/*
#[test]
fn test_int_lit_emit() {
    let mut out = vec![];
    let ast = Ast::IntLit(5, Token);
    emit(&ast, &mut out);
    assert!(out.len() == 1);
    assert_eq!(Instr::AddInt, out[0]);
}*/
