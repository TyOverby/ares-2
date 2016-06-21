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

    let mut ctx = UnloadedContext::<bool>::new();

    ctx.set_global("print", user_function::<bool, _>(None,
        |args, _, ctx| {
            println!("{}", ctx.format_value(&args[0]));
            Value::Nil
        }
    ));

    ctx.set_global("exit", user_function::<bool, _>(None,
        |_, state, _| {
            *state = true;
            Value::Nil
        }
    ));

    let mut exit_requested = false;
    let mut ctx = ctx.load(&mut exit_requested);
    let mut expr_iter = 0;

    let mut buildup = String::new();
    while !*ctx.state() {
        while let Some(input) = linenoise::input(&format!("{}", (if buildup.len() > 0 {"----> "} else {"ares> "}).cyan())) {
            buildup.push_str(&input);
            buildup.push('\n');

            match ctx.eval(&buildup) {
                Ok(None) => {
                    linenoise::history_add(&buildup);
                    buildup.clear();
                },
                Ok(Some(v)) => {
                    expr_iter += 1;
                    let expr_id = format!("__{}", expr_iter);
                    let value_str = ctx.format_value(&v);
                    ctx.set_global(&expr_id[..], v);

                    linenoise::history_add(&buildup);
                    buildup.clear();
                    println!("{} = {}", expr_id.yellow(), value_str.green());
                }
                Err(AresError::CompileError(CompileError::ParseError(ParseError::UnrecognizedToken{token: None, ..}))) => {

                }
                Err(e) => {
                    linenoise::history_add(&buildup);
                    buildup.clear();
                    println!("{}", ctx.format_error(e).red());
                }
            }

            if *ctx.state() {
                break;
            }
        }

        if buildup.len() == 0 {
            break;
        }

        buildup.clear();
    }
}
