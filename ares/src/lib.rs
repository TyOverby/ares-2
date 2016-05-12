#![feature(pub_restricted)]

#![allow(dead_code)]

#[macro_use]
extern crate gc;
#[macro_use(ast)]
extern crate ares_syntax;
extern crate libc;
extern crate typed_arena;
extern crate itertools;
extern crate lalrpop_util;

#[cfg(test)]
extern crate latin;

macro_rules! matches {
    ($e: expr, $p: pat) => {
        if let $p = $e { true } else { false }
    }
}

pub mod compiler;
pub mod vm;
pub mod host;
mod util;


#[cfg(test)]
fn assert_compilation_steps(program: &str, bound: Option<String>, instr: Option<String>, output: Option<String>, result: Option<String>) {
    use host::*;
    use vm::*;

    // Binding
    if let Some(bound) = bound {
        compiler::binding::test::assert_bound_form(program, &bound);
    }

    if let Some(instr) = instr {
        compiler::emit::test::assert_instrs(program, &instr);
    }

    if output.is_some() || result.is_some() {
        let mut ctx: UnloadedContext<Vec<String>> = UnloadedContext::new();

        ctx.set_global("print", user_function(None, |args, state: &mut Vec<String>, ctx| {
            assert!(args.len() == 1);
            let formatted: String = ctx.format_value(&args[0]);
            state.push(formatted);
            0.into()
        }));

        let mut actual_output = vec![];
        let actual_result = {
            let mut ctx = ctx.load(&mut actual_output);
            ctx.eval(&program).unwrap()
        };

        if let Some(expected_output) = output {
            assert_eq!(expected_output.lines().map(String::from).collect::<Vec<_>>(), actual_output)
        }
        if let Some(expected_result) = result {
            let as_string = ctx.format_value(&actual_result);
            assert_eq!(expected_result, as_string);
        }
    }
}
