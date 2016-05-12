extern crate latin;

use std::path::{Path, PathBuf};

const TEST_SIGNIFIER: &'static str = "#test";
const BIND_SIGNIFIER: &'static str = "#bind";
const EMIT_SIGNIFIER: &'static str = "#emit";
const OUTPUT_SIGNIFIER: &'static str = "#output";
const RESULT_SIGNIFIER: &'static str = "#result";

struct Checks {
    name: String,
    binding: Option<Result<(), String>>,
    emit: Option<Result<(), String>>,
    output: Option<Result<(), String>>,
    result: Option<Result<(), String>>,
}

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

fn run_these(name: String, program: String, phases: Vec<Phase>) -> Checks {
    unimplemented!();
}

fn run_test<F, I: Iterator<Item=String>>(lines: I, corrector: F) -> Vec<Checks>
where I: Iterator<Item=String>, F: Fn(String, String, Vec<Phase>) -> Checks {
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

            current_name = Some(line[TEST_SIGNIFIER.len() ..].trim().to_string());
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
                (None, None) => panic!("not inside a valid program of phase.")
            }
        }
    }

    if let (Some(name), Some(prog), Some(phase)) = (current_name.take(), current_program.take(), current_phase.take()) {
        phases.push(phase);
        out.push(corrector(name, prog, phases));
        phases = vec![];
    }

    out
}

#[test]
fn main() {
    let path_to_me = Path::new(file!());
    let mut path_to_test_dir = path_to_me.parent().unwrap();

    let mut tests = vec![];

    for test in ::latin::directory::children(path_to_test_dir).unwrap() {
        if test.ends_with(".artest") {
            let lines = ::latin::file::read_lines(test).unwrap().map(|l| l.unwrap());
            tests.append(&mut run_test(lines, run_these));
        }
    }
}

#[test]
fn framework_works() {
    let file =
r#"#test foo
a + b
#bind
bindings
#emit
emitings
#output
outputings
#result
resultings
"#;

    let phases = run_test(file.lines().map(String::from), |name, program, phases| {
        assert_eq!(name, "foo");
        assert_eq!(program, "a + b");

        let mut found_binding = false;
        let mut found_emitting = false;
        let mut found_outputting = false;
        let mut found_result = false;

        for phase in phases {
            match phase {
                Phase::Binding(s) => {
                    assert_eq!(s, "bindings");
                    found_binding = true;
                }
                Phase::Emit(s)  => {
                    assert_eq!(s, "emitings");
                    found_emitting = true;
                }
                Phase::Output(s) => {
                    assert_eq!(s, "outputings");
                    found_outputting = true;
                }
                Phase::Result(s) =>  {
                    assert_eq!(s, "resultings");
                    found_result = true;
                }
            }
        }

        assert!(found_binding, "did not find a binding");
        assert!(found_emitting, "did not find a emitting");
        assert!(found_outputting, "did not find a outputting");
        assert!(found_result, "did not find a result");

        Checks {
            name: name,
            binding: None,
            emit: None,
            output: None,
            result: None,
        }
    });
}
