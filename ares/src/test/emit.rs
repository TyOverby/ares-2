use compiler::CompileContext;
use ares_syntax::SymbolIntern;
use typed_arena::Arena;
use super::do_emitting;
use compiler::emit::EmitBuffer;
use vm::Modules;
use super::TestResult;

pub fn test_emit<'ast, 'bound>(
    program: &str,
    output: &str,
    interner: &mut SymbolIntern,
    modules: Option<&Modules>) -> TestResult {

    let parse_arena = Arena::new();
    let bind_arena = Arena::new();
    let mut cc = CompileContext::new();
    let mut buffer = EmitBuffer::new();

    if let Err(e) = do_emitting(program, &parse_arena, &bind_arena, interner, modules, &mut cc, &mut buffer) {
        return TestResult::Error(e);
    }

    let instrs = buffer.into_instructions();
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
            return TestResult::Bad(
                format!("at {}, {} was expected, but {} was produced", i, expected, actual));
        }
    }

    TestResult::Good
}
