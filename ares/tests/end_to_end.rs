extern crate ares;

use std::io::{Read, BufRead};
use std::fs::File;

use ares::host::{Context, UnloadedContext};
use ares::vm::user_function;

#[test]
pub fn literals() {
    run_test("literals");
}

#[test]
pub fn hard_parse() {
    run_test("hard_parse");
}

fn get_lines(contents: String) -> (String, Vec<String>) {
    let mut program = vec![];
    let mut expected = vec![];
    let mut seen_sep = false;
    for line in contents.lines() {
        if line.len() > 4 && line.chars().all(|c| c == '=') {
            seen_sep = true;
        } else if seen_sep {
            expected.push(line.to_owned());
        } else {
            program.push(line.to_owned());
        }
    }
    (program.join("\n"), expected)
}

fn read_file(test_name: &str) -> String {
    let file = File::open(&format!("./tests/{}.artest", test_name));
    let mut file = file.unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    buffer
}

fn run_test(test_name: &str) {
    let contents = read_file(test_name);
    let (program, expected) = get_lines(contents);

    let mut ctx: UnloadedContext<Vec<String>> = UnloadedContext::new();

    ctx.set_global("print", user_function(None, |args, state: &mut Vec<String>, ctx| {
        assert!(args.len() == 1);
        let formatted: String = ctx.format_value(&args[0]);
        state.push(formatted);
        0.into()
    }));

    let mut output = vec![];
    {
        let mut ctx = ctx.load(&mut output);
        ctx.eval(&program).unwrap();
    }

    assert_eq!(output, expected);
}
