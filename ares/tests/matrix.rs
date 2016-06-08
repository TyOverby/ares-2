extern crate latin;
extern crate ares;

use ares::test::{TestRunResults, TestResult};

const TEST_SIGNIFIER: &'static str = "#test";
const BIND_SIGNIFIER: &'static str = "#bind";
const EMIT_SIGNIFIER: &'static str = "#emit";
const OUTPUT_SIGNIFIER: &'static str = "#output";
const RESULT_SIGNIFIER: &'static str = "#result";

enum Phase {
    Binding(String),
    Emit(String),
    Output(String),
    Result(String),
}

impl Phase {
    fn get_lines(&mut self) -> &mut String {
        match *self {
            Phase::Binding(ref mut s) |
            Phase::Emit(ref mut s) |
            Phase::Output(ref mut s) |
            Phase::Result(ref mut s) => s
        }

    }

    fn append_line(&mut self, line: String) {
        let s = self.get_lines();
        if s.len() != 0 {
            s.push('\n');
        }
        s.push_str(&line);
    }
}

fn run_these(name: String, program: String, phases: Vec<Phase>) -> TestRunResults {
    let mut binding = None;
    let mut emitting = None;
    let mut output = None;
    let mut result = None;
    println!("- running {}", name);

    for phase in phases { match phase {
        Phase::Binding(b) => {
            assert!(binding.is_none(), "two binding blocks for test {}", name);
            binding = Some(b);
        }
        Phase::Emit(e) => {
            assert!(emitting.is_none(), "two emit blocks for test {}", name);
            emitting = Some(e);
        }
        Phase::Output(o) => {
            assert!(output.is_none(), "two output blocks for test {}", name);
            output = Some(o);
        }
        Phase::Result(r) => {
            assert!(result.is_none(), "two result blocks for test {}", name);
            result = Some(r)
        }
    }}

    ares::test::assert_compilation_steps(&name, &program, binding, emitting, output, result)
}

fn run_test<F, I: Iterator<Item=String>>(file_name: String, lines: I, corrector: F) -> Vec<TestRunResults>
where I: Iterator<Item=String>, F: Fn(String, String, Vec<Phase>) -> TestRunResults {
    let mut out = vec![];
    let mut phases = vec![];
    let mut current_name = None;
    let mut current_program = None;
    let mut current_phase = None;

    for line in lines {
        let line = line.trim_right().to_string();
        if line.starts_with(TEST_SIGNIFIER) {
            if let (Some(name), Some(prog), Some(phase)) = (current_name.take(), current_program.take(), current_phase.take()) {
                phases.push(phase);
                out.push(corrector(name, prog, phases));
                phases = vec![];
            }

            current_name = Some(format!("{}/{}", file_name, line[TEST_SIGNIFIER.len() ..].trim().to_string()));
            current_program = Some(String::new());
        } else if current_phase.is_some() && line.chars().all(char::is_whitespace) {
            continue;
        } else if line.starts_with(BIND_SIGNIFIER) {
            phases.extend(current_phase.take());
            current_phase = Some(Phase::Binding(String::new()));
        } else if line.starts_with(EMIT_SIGNIFIER) {
            phases.extend(current_phase.take());
            current_phase = Some(Phase::Emit(String::new()));
        } else if line.starts_with(OUTPUT_SIGNIFIER) {
            phases.extend(current_phase.take());
            current_phase = Some(Phase::Output(String::new()));
        } else if line.starts_with(RESULT_SIGNIFIER) {
            phases.extend(current_phase.take());
            current_phase = Some(Phase::Result(String::new()));
        } else {
            match (current_program.as_mut(), current_phase.as_mut()) {
                (_, Some(phase)) => phase.append_line(line),
                (Some(program), _) => program.push_str(&line),
                (None, None) => panic!("not inside a valid program")
            }
        }
    }

    if let (Some(name), Some(prog), Some(phase)) = (current_name.take(), current_program.take(), current_phase.take()) {
        phases.push(phase);
        out.push(corrector(name, prog, phases));
    }

    out
}

#[test]
fn main() {
    use std::io::Write;
    fn check(b: TestResult) -> String {
        match b {
            TestResult::Good =>   ":heavy_check_mark:".to_string(),
            TestResult::NotRan => "                  ".to_string(),
            TestResult::Bad(s) => s,
            _ => panic!()
        }
    }

    let mut tests = vec![];

    for test in ::latin::directory::children("./tests/").unwrap() {
        if ::latin::file::has_extension(&test, "artest") {
            let lines = ::latin::file::read_lines(&test).unwrap().map(|l| l.unwrap());
            let file_name = format!("{}", test.display());
            println!("+ {:?}", file_name);
            tests.append(&mut run_test(file_name, lines, run_these));
        }
    }

    let longest_name = tests.iter().map(|test| test.name.len()).max().unwrap_or(0);

    let mut buffer: Vec<u8> = Vec::new();

    writeln!(&mut buffer, "| {2:<0$} | {3:<1$} | {4:<1$} | {5:<1$} | {6:<1$} |",
             longest_name, check(TestResult::Good).len(),
             "name", "binding", "emit", "output", "result").unwrap();

    writeln!(&mut buffer, "|---|---|---|---|---|").unwrap();

    let mut any_failed = false;
    for mut test in tests {
        for result in test.all_results() {
            match result {
                &TestResult::Good | &TestResult::NotRan => {}
                &TestResult::Bad(ref s) => {
                    any_failed = true;
                    println!("test {} with program \n\t{}\n failed with:\n\t{}", test.name, test.program, s);
                }
                &TestResult::Panic(_) => unimplemented!(),
                &TestResult::Error(ref e) => {
                    any_failed = true;
                    println!("\n\ntest {} with program \n\t{}\n didn't compile with \n\t{:?}", test.name, test.program, e);
                }
            }
        }
        // No output and not run means insta-pass
        test.output_test = match (test.output_test, test.any_output) {
            (TestResult::NotRan, false) => TestResult::Good,
            (otherwise, _) => otherwise,
        };

        writeln!(&mut buffer, "| {0:<1$} | {2} | {3} | {4} | {5} |", test.name, longest_name,
               check(test.binding_test),
               check(test.emit_test),
               check(test.output_test),
               check(test.result_test)).unwrap();
    }
    ::latin::file::write("./tests/readme.md", &buffer).unwrap();
    if any_failed {
        panic!("tests failed!");
    }
}

#[test]
fn matrix_framework_works() {
    let file =
r#"#test foo
a + b
#bind
bindings foo
#emit
emitings foo
#output
outputings foo
#result
resultings foo
"#;

    let _ = run_test("test_file".to_string(), file.lines().map(String::from), |name, program, phases| {
        assert_eq!(name, "test_file/foo");
        assert_eq!(program, "a + b");

        let mut found_binding = false;
        let mut found_emitting = false;
        let mut found_outputting = false;
        let mut found_result = false;

        for phase in phases {
            match phase {
                Phase::Binding(s) => {
                    assert_eq!(s, "bindings foo");
                    found_binding = true;
                }
                Phase::Emit(s)  => {
                    assert_eq!(s, "emitings foo");
                    found_emitting = true;
                }
                Phase::Output(s) => {
                    assert_eq!(s, "outputings foo");
                    found_outputting = true;
                }
                Phase::Result(s) =>  {
                    assert_eq!(s, "resultings foo");
                    found_result = true;
                }
            }
        }

        assert!(found_binding, "did not find a binding");
        assert!(found_emitting, "did not find a emitting");
        assert!(found_outputting, "did not find a outputting");
        assert!(found_result, "did not find a result");

        TestRunResults {
            name: "test".to_string(),
            program: "test".to_string(),
            binding_test: TestResult::Good,
            emit_test: TestResult::Good,
            output_test: TestResult::Good,
            result_test: TestResult::Good,
            any_output: true,
        }
    });
}
