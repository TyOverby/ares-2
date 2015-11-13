extern crate ares;

use ares::vm::{Vm, Value};
use ares::compiler::compile;

fn run_this(src: &str) -> Value {
    let mut vm = Vm::new();
    let compiled = compile(src, &mut vm.compile_context, &mut vm.interner).unwrap();
    vm.load_and_execute(&compiled[..], 0).unwrap();
    vm.stack.pop().unwrap()
}

#[test]
fn test_add() {
    assert_eq!(run_this("(+ 1)"), Value::Int(1));
    assert_eq!(run_this("(+)"), Value::Int(0));
    assert_eq!(run_this("(+ 1 2 3)"), Value::Int(6));
    assert_eq!(run_this("(+ 1 (+ 2 3))"), Value::Int(6));
}

#[test]
fn test_iff() {
    assert_eq!(run_this("(if true 15 20)"), Value::Int(15));
    assert_eq!(run_this("(if false 15 20)"), Value::Int(20));
}

#[test]
fn test_literals() {
    assert_eq!(run_this("123"), Value::Int(123));
    assert_eq!(run_this("3.14"), Value::Float(3.14));
    assert_eq!(run_this("\"hello world\""), "hello world".into());
    assert_eq!(run_this("8589934592"), 8589934592i64.into());
}
