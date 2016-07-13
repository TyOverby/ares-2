use super::test_binding;
use super::test_emit;
pub use ares_syntax::{Span, Ast, AstRef};
use super::TestResult;

pub struct TestRunResults {
    pub name: String,
    pub program: String,
    pub binding_test: TestResult,
    pub emit_test: TestResult,
    pub output_test: TestResult,
    pub result_test: TestResult,
    pub any_output: bool,
}

impl TestRunResults {
    pub fn all_results(&self) -> Vec<&TestResult> {
        vec![&self.binding_test, &self.emit_test, &self.output_test, &self.result_test]
    }
}

pub fn assert_compilation_steps(
    name: &str,
    program: &str,
    bound: Option<String>,
    instr: Option<String>,
    output: Option<String>,
    result: Option<String>) -> TestRunResults
{
    use host::*;
    use vm::*;

    let mut test_run_results = TestRunResults {
        name: name.to_string(),
        program: program.to_string(),
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
            match ctx.eval(&program) {
                Ok(value) => value,
                Err(err) => {
                    println!("{}", ctx.format_error(err));

                    let (stack, instrs, i) = ctx.dump_vm_internals();
                    println!("STACK");
                    for value in stack {
                        println!(" {}", ctx.format_value(&value));
                    }

                    println!("INSTRUCTIONS");
                    for (k, instr) in instrs.into_iter().enumerate() {
                        let current = if k == i {">"} else {" "};
                        println!("{}{:?}", current, instr);
                    }

                    panic!();
                }
            }
        };

        assert!(ctx.vm.utility_stack.len() == 0);

        if let Some(expected_output) = output {
            assert_eq!(expected_output.lines().map(String::from).collect::<Vec<_>>(), actual_output);
            // TODO: Do better than assert and panic
            test_run_results.output_test = TestResult::Good;
        }

        if let Some(expected_result) = result {
            // TODO: Do better than assert and panic
            if let Some(actual_result) = actual_result {
                let as_string = ctx.format_value(&actual_result);
                assert!(expected_result == as_string, "The program \n{}\n had a result of {:?} but you thought it was {:?}", program, actual_result, expected_result);
            } else {
                assert!(expected_result.is_empty(), "The program \n{}\n had no return value, but you provieded {}", program, expected_result);
            }
            test_run_results.result_test = TestResult::Good;
        }

        test_run_results.any_output = actual_output.len() != 0;
    }

    return test_run_results;
}
