extern crate ares_compiler;
extern crate ares_vm;

use ares_vm::{Vm, Value};
use ares_compiler::compile;

fn run_this(src: &str) -> Value {
    let mut vm = Vm::new();
    let compiled = compile(src, &mut vm.interner).unwrap();
    vm.load_and_execute(&compiled[..], 0).unwrap();
    vm.stack.pop().unwrap()
}

#[test]
fn addition() {
    assert_eq!(run_this("(+ 1 2 3)"), Value::Int(6));
}
