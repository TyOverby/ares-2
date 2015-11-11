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
    assert_eq!(run_this("(+ 1)"), Value::Int(1));
    assert_eq!(run_this("(+)"), Value::Int(0));
    assert_eq!(run_this("(+ 1 2 3)"), Value::Int(6));
    assert_eq!(run_this("(+ 1 (+ 2 3))"), Value::Int(6));
}

#[test]
fn iff() {
    assert_eq!(run_this("(if true 15 20)"), Value::Int(15));
    assert_eq!(run_this("(if false 15 20)"), Value::Int(20));
}
