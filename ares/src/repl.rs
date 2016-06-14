extern crate ares;
extern crate colored;
extern crate linenoise;

use ares::host::*;
use ares::vm::Value;
use ares::vm::user_function;
use colored::*;

fn main() {
    let mut ctx = UnloadedContext::<()>::new();
    ctx.set_global("print", user_function::<(), _>(None,
        |args, _, ctx| {
            println!("{}", ctx.format_value(&args[0]));
            Value::Nil
        }
    ));

    let mut state = ();
    let mut ctx = ctx.load(&mut state);

    while let Some(input) = linenoise::input(&format!("{}", "ares> ".cyan())) {
        linenoise::history_add(&input);
        match ctx.eval(&input) {
            Ok(None) => {},
            Ok(Some(v)) => {
                println!("{}", ctx.format_value(&v).green());
            }
            Err(e) => {
                println!("{}", ctx.format_error(e).red());
            }
        }
    }
}
