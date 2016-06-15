extern crate ares;
extern crate colored;
extern crate linenoise;

use ares::host::*;
use ares::vm::Value;
use ares::vm::user_function;
use ares::compiler::CompileError;
use ares::compiler::ParseError;
use colored::*;

fn main() {
    linenoise::set_multiline(3);
    let mut ctx = UnloadedContext::<()>::new();
    ctx.set_global("print", user_function::<(), _>(None,
        |args, _, ctx| {
            println!("{}", ctx.format_value(&args[0]));
            Value::Nil
        }
    ));

    let mut state = ();
    let mut ctx = ctx.load(&mut state);

    let mut buildup = String::new();

    while let Some(input) = linenoise::input(&format!("{}", (if buildup.len() > 0 {"----> "} else {"ares> "}).cyan())) {
        buildup.push_str(&input);
        buildup.push('\n');

        match ctx.eval(&buildup) {
            Ok(None) => {
                linenoise::history_add(&buildup);
                buildup.clear();
            },
            Ok(Some(v)) => {
                linenoise::history_add(&buildup);
                buildup.clear();
                println!("{}", ctx.format_value(&v).green());
            }
            Err(AresError::CompileError(CompileError::ParseError(ParseError::UnrecognizedToken{token: None, ..}))) => {

            }
            Err(e) => {
                linenoise::history_add(&buildup);
                buildup.clear();
                println!("{}", ctx.format_error(e).red());
            }
        }
    }
}
