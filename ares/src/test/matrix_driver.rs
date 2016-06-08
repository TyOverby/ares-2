use super::super::compiler;
use super::test_binding;
use super::test_emit;
use vm::{Instr, Modules};
use compiler::CompileContext;
use ares_syntax::{SymbolIntern};
use typed_arena::Arena;
pub use ares_syntax::{Span, Ast, AstRef};
use super::TestResult;
use std::panic::catch_unwind;

pub struct TestRunResults {
    binding_test: TestResult,
    emit_test: TestResult,
    output_test: TestResult,
    result_test: TestResult,
    any_output: bool,
}

pub fn assert_compilation_steps(
    program: &str,
    bound: Option<String>,
    instr: Option<String>,
    output: Option<String>,
    result: Option<String>) -> TestRunResults
{
    use host::*;
    use vm::*;

    let mut test_run_results = TestRunResults {
        binding_test: TestResult::NotRan,
        emit_test: TestResult::NotRan,
        output_test: TestResult::NotRan,
        result_test: TestResult::NotRan,
        any_output: true,
    };

    fn get_vm() -> UnloadedContext<Vec<String>> {
        let mut ctx: UnloadedContext<Vec<String>> = UnloadedContext::new();

        ctx.set_global("print", user_function(None, |args, state: &mut Vec<String>, ctx| {
            assert!(args.len() == 1);
            let formatted: String = ctx.format_value(&args[0]);
            state.push(formatted);
            0.into()
        }));
        ctx
    }


    // Binding
    if let Some(bound) = bound {
        let UnloadedContext{vm: Vm {ref mut interner, ref mut globals, ..}} = get_vm();
        catch_unwind(|| test_binding(program, &bound, Some(globals), interner));
        test_run_results.binding_test = test_binding(program, &bound, Some(globals), interner);
    }

    if let Some(instr) = instr {
        let UnloadedContext{vm: Vm {ref mut interner, ref mut globals, ..}} = get_vm();
        test_run_results.emit_test = test_emit(program, &instr, interner, Some(globals));
    }

    if output.is_some() || result.is_some() {
        let mut ctx = get_vm();

        let mut actual_output = vec![];
        let actual_result = {
            let mut ctx = ctx.load(&mut actual_output);
            ctx.eval(&program).unwrap()
        };

        if let Some(expected_output) = output {
            assert_eq!(expected_output.lines().map(String::from).collect::<Vec<_>>(), actual_output)
        }

        if let Some(expected_result) = result {
            if let Some(actual_result) = actual_result {
                let as_string = ctx.format_value(&actual_result);
                assert!(expected_result == as_string, "The program \n{}\n had a result of {:?} but you thought it was {:?}", program, actual_result, expected_result);
            } else {
                assert!(expected_result.is_empty(), "The program \n{}\n had no return value, but you provieded {}", program, expected_result);
            }
        }

        test_run_results.any_output = actual_output.len() != 0;
    }

    return test_run_results;
}
