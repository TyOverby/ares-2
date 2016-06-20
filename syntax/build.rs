extern crate lalrpop;
use std::process::Command;

fn main() {
    let s = Command::new("lalrpop").arg("src/syntax.lalrpop").status();
    if s.is_err() || !s.unwrap().success() {
        lalrpop::process_root().unwrap();
    }
}
