use vm::{Instr, Value};
use compiler::CompileContext;
use ares_syntax::{SymbolIntern};
use typed_arena::Arena;

fn compile_this(program: &str) -> (Vec<Instr>, CompileContext) {
    use compiler::emit::emit;
    use compiler::emit::EmitBuffer;
    use compiler::binding::Bound;

    let mut out = EmitBuffer::new();
    let mut compile_context = CompileContext::new();
    let mut bound_arena = Arena::new();
    let mut interner = SymbolIntern::new();
    let ast = ::compiler::parse::test::ok_parse_1_full(program, &mut interner);
    let bound = Bound::bind_top(ast, &mut bound_arena, &mut interner).unwrap();
    emit(&bound, &mut compile_context, &mut out, None).unwrap();
    (out.into_instructions(), compile_context)
}

pub fn assert_instrs(program: &str, output: &str) {
    let (instrs, _) = compile_this(program);
    let instrs_lines: Vec<String> = instrs.iter().map(|a| format!("{:?}", a)).collect();
    let expected_lines: Vec<String> = output.lines().map(String::from).collect();
    if instrs_lines.len() != expected_lines.len() {
        println!("{} instructions were expected, but only {} were actually produced", expected_lines.len(), instrs_lines.len());
        println!("EXPECTED: \n{}", expected_lines.join("\n"));
        println!("ACTUAL: \n{}", instrs_lines.join("\n"));
        panic!();
    }

    for (i, (expected, actual)) in expected_lines.into_iter().zip(instrs_lines.into_iter()).enumerate() {
        if expected != actual {
            panic!("at {}, {} was expected, but {} was produced", i, expected, actual);
        }
    }
}

#[test]
fn test_literal_emit() {
    {
        let (out, _) = compile_this("5");
        assert_eq!(out, vec![Instr::IntLit(5)]);
    }
    {
        let (out, cc) = compile_this("8589934592");
        assert_eq!(cc.get_constant(0), Value::Int(8589934592));
        assert_eq!(out, vec![Instr::LoadConstant(0)]);

    }
    {
        let (out, cc) = compile_this("3.14");
        assert_eq!(cc.get_constant(0), Value::Float(3.14));
        assert_eq!(out, vec![Instr::LoadConstant(0)]);
    }
    {
        let (out, cc) = compile_this("\"hello world\"");
        assert_eq!(cc.get_constant(0), "hello world".into());
        assert_eq!(out, vec![Instr::LoadConstant(0)]);
    }
}

#[test]
fn test_add_emit() {
    {
        // Add two things
        let (out, _) = compile_this("5 + 10");
        assert_eq!(out,
                   vec![Instr::IntLit(5), Instr::IntLit(10), Instr::AddInt]);
    }
    {
        let (out, _) = compile_this("(5 + 10) + (15 + 20)");
        assert_eq!(out,
                   vec![Instr::IntLit(5),
                        Instr::IntLit(10),
                        Instr::AddInt,
                        Instr::IntLit(15),
                        Instr::IntLit(20),
                        Instr::AddInt,
                        Instr::AddInt]);
    }
}

#[test]
fn test_sub_emit() {
    let (out, _) = compile_this("5 - 10");
    assert_eq!(out,
               vec![Instr::IntLit(5), Instr::IntLit(10), Instr::SubInt]
               )
}

#[test]
fn test_basic_if() {
    let (out, _) = compile_this("if true then 15 else 20");

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
    let (out, _) = compile_this("fn() { 10; 5 }");

    assert_eq!(out, vec![
               Instr::CreateClosure(0),
               Instr::Jump(6),
               Instr::IntLit(10),
               Instr::Pop,
               Instr::IntLit(5),
               Instr::Ret]);
}

#[test]
fn emit_fn_call() {
    let (out, _) = compile_this("1(2, 3)");
    assert_eq!(out,
               vec![Instr::IntLit(2),
                    Instr::IntLit(3),
                    Instr::IntLit(1),
                    Instr::Execute(2)]);
}

#[test]
fn emit_one_arg_lambda() {
    let (out, _) = compile_this("fn(a) { a }");

    assert_eq!(out, vec![
               Instr::CreateClosure(0),
               Instr::Jump(4),
               Instr::Dup(0),
               Instr::Ret]);
}

#[test]
fn emit_if_statement_no_else() {
    let (out, _) = compile_this("if true then { 1(); }");

    assert_eq!(out, vec![
               Instr::BoolLit(true),
               Instr::Ifn,
               Instr::Jump(6),
               Instr::IntLit(1),
               Instr::Execute(0),
               Instr::Pop,
    ]);
}

#[test]
fn emit_if_statement() {
    let (out, _) = compile_this("if true then { 1(); } else { 2(); }");

    assert_eq!(out, vec![
               Instr::BoolLit(true),
               Instr::Ifn,
               Instr::Jump(7),
               Instr::IntLit(1),
               Instr::Execute(0),
               Instr::Pop,
               Instr::Jump(10),
               Instr::IntLit(2),
               Instr::Execute(0),
               Instr::Pop,
    ]);
}
