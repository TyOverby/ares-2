use super::*;
use ares_syntax::*;

#[test]
fn assumptions() {
    use std::mem::size_of;
    assert_eq!(4, size_of::<Symbol>());
    assert_eq!(8, size_of::<Instr>());
    assert_eq!(24, size_of::<Value>());
}

#[test]
fn test_addint() {
    let mut vm = Vm::<()>::new();
    vm.stack.push(Value::Int(5)).unwrap();
    vm.stack.push(Value::Int(10)).unwrap();
    vm.load_and_execute(&[Instr::AddInt], 2, &mut ()).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, Value::Int(15));
}

#[test]
fn get_global() {
    let mut vm = Vm::<()>::new();
    let symbol = vm.interner.intern("hi");
    vm.stack.push(Value::Int(20)).unwrap();
    vm.load_and_execute(&[Instr::PutGlobal(symbol),
                          Instr::GetGlobal(symbol)], 1, &mut ()).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, Value::Int(20));
}

#[test]
fn load_literals() {
    {
        let mut vm = Vm::<()>::new();
        let dummy = vm.interner.intern("dummy");
        vm.load_and_execute(&[Instr::SymbolLit(dummy)], 0, &mut ()).unwrap();

        let result = vm.stack.pop().unwrap();
        assert_eq!(result, Value::Symbol(dummy));
    }

    {
        let mut vm = Vm::<()>::new();
        vm.load_and_execute(&[Instr::BoolLit(true)], 0, &mut ()).unwrap();

        let result = vm.stack.pop().unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    {
        let mut vm = Vm::<()>::new();
        vm.load_and_execute(&[Instr::IntLit(5)], 0, &mut ()).unwrap();

        let result = vm.stack.pop().unwrap();
        assert_eq!(result, Value::Int(5));
    }
}

#[test]
fn test_jmp() {
    let mut vm = Vm::<()>::new();
    vm.load_and_execute(&[Instr::Jump(2), Instr::IntLit(5), Instr::IntLit(10)], 0, &mut ())
      .unwrap();
    assert_eq!(vm.stack.pop().unwrap(), Value::Int(10));
    assert!(vm.stack.len() == 0);

}

#[test]
fn test_if() {
    {
        let mut vm = Vm::<()>::new();
        vm.stack.push(Value::Int(10)).unwrap();
        vm.stack.push(Value::Int(5)).unwrap();
        vm.stack.push(Value::Bool(true)).unwrap();

        vm.load_and_execute(&[Instr::If,
                              Instr::Jump(3),
                              Instr::Jump(5),
                              Instr::AddInt,
                              Instr::Halt,
                              Instr::SubInt],
                            3, &mut ())
          .unwrap();
        assert_eq!(vm.stack.pop().unwrap(), Value::Int(15));
    }
    {
        let mut vm = Vm::<()>::new();
        vm.stack.push(Value::Int(10)).unwrap();
        vm.stack.push(Value::Int(5)).unwrap();
        vm.stack.push(Value::Bool(false)).unwrap();

        vm.load_and_execute(&[Instr::If,
                              Instr::Jump(3),
                              Instr::Jump(5),
                              Instr::AddInt,
                              Instr::Halt,
                              Instr::SubInt],
                            3, &mut ())
          .unwrap();
        assert_eq!(vm.stack.pop().unwrap(), Value::Int(5));
    }
    {
        let mut vm = Vm::<()>::new();
        vm.stack.push(Value::Int(10)).unwrap();
        vm.stack.push(Value::Int(5)).unwrap();
        vm.stack.push(Value::Bool(true)).unwrap();

        vm.load_and_execute(&[Instr::Ifn,
                              Instr::Jump(3),
                              Instr::Jump(5),
                              Instr::AddInt,
                              Instr::Halt,
                              Instr::SubInt],
                            3, &mut ())
          .unwrap();
        assert_eq!(vm.stack.pop().unwrap(), Value::Int(5));
    }
    {
        let mut vm = Vm::<()>::new();
        vm.stack.push(Value::Int(10)).unwrap();
        vm.stack.push(Value::Int(5)).unwrap();
        vm.stack.push(Value::Bool(false)).unwrap();

        vm.load_and_execute(&[Instr::Ifn,
                              Instr::Jump(3),
                              Instr::Jump(5),
                              Instr::AddInt,
                              Instr::Halt,
                              Instr::SubInt],
                            3, &mut ())
          .unwrap();
        assert_eq!(vm.stack.pop().unwrap(), Value::Int(15));
    }
}

#[test]
fn test_swap() {
    let mut vm = Vm::<()>::new();
    vm.stack.push(Value::Int(10)).unwrap();
    vm.stack.push(Value::Int(5)).unwrap();
    vm.load_and_execute(&[Instr::Swap], 1, &mut ()).unwrap();
    assert_eq!(vm.stack.pop().unwrap(), Value::Int(10));
    assert_eq!(vm.stack.pop().unwrap(), Value::Int(5));
}

#[test]
fn test_and() {
    let mut vm = Vm::<()>::new();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.load_and_execute(&[Instr::And], 2, &mut ()).unwrap();
    assert_eq!(vm.stack.pop().unwrap(), Value::Bool(true));

    let mut vm = Vm::<()>::new();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.stack.push(Value::Bool(false)).unwrap();
    vm.load_and_execute(&[Instr::And], 2, &mut ()).unwrap();
    assert_eq!(vm.stack.pop().unwrap(), Value::Bool(false));

    let mut vm = Vm::<()>::new();
    vm.stack.push(Value::Bool(false)).unwrap();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.load_and_execute(&[Instr::And], 2, &mut ()).unwrap();
    assert_eq!(vm.stack.pop().unwrap(), Value::Bool(false));

    let mut vm = Vm::<()>::new();
    vm.stack.push(Value::Bool(false)).unwrap();
    vm.stack.push(Value::Bool(false)).unwrap();
    vm.load_and_execute(&[Instr::And], 2, &mut ()).unwrap();
    assert_eq!(vm.stack.pop().unwrap(), Value::Bool(false));
}

#[test]
fn test_or() {
    let mut vm = Vm::<()>::new();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.load_and_execute(&[Instr::Or], 2, &mut ()).unwrap();
    assert_eq!(vm.stack.pop().unwrap(), Value::Bool(true));

    let mut vm = Vm::<()>::new();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.stack.push(Value::Bool(false)).unwrap();
    vm.load_and_execute(&[Instr::Or], 2, &mut ()).unwrap();
    assert_eq!(vm.stack.pop().unwrap(), Value::Bool(true));

    let mut vm = Vm::<()>::new();
    vm.stack.push(Value::Bool(false)).unwrap();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.load_and_execute(&[Instr::Or], 2, &mut ()).unwrap();
    assert_eq!(vm.stack.pop().unwrap(), Value::Bool(true));

    let mut vm = Vm::<()>::new();
    vm.stack.push(Value::Bool(false)).unwrap();
    vm.stack.push(Value::Bool(false)).unwrap();
    vm.load_and_execute(&[Instr::Or], 2, &mut ()).unwrap();
    assert_eq!(vm.stack.pop().unwrap(), Value::Bool(false));
}

#[test]
fn test_lone_ret() {
    let mut vm = Vm::<()>::new();
    vm.stack.push(Value::Int(5)).unwrap();
    vm.load_and_execute(&[Instr::Ret], 0, &mut ()).unwrap();
    assert_eq!(vm.stack.pop().unwrap(), Value::Int(5));
}

#[test]
fn test_call() {
    let mut vm = Vm::<()>::new();
    vm.stack.push(Value::Int(5)).unwrap();
    vm.stack.push(Value::Int(10)).unwrap();

    vm.load_and_execute(&[Instr::IntLit(2), // 2 args
                          Instr::Call(3), // call the adding function
                          Instr::Ret, // return from main
                          Instr::AddInt, // add the numbers
                          Instr::Ret /* return from adding */],
                        2, &mut ())
      .unwrap();
    assert_eq!(vm.stack.pop().unwrap(), Value::Int(15));
}

#[test]
fn naive_fib() {
    let mut vm = Vm::<()>::new();
    vm.stack.push(Value::Int(15)).unwrap();
    vm.load_and_execute(&[Instr::Dbg, // 0 [6]
                          Instr::Dup(0), // 1 [6, 6]
                          Instr::Dup(0), // 2 [6, 6, 6]
                          Instr::IntLit(0), // 3 [6, 6, 6, 0]
                          Instr::Eq, // 4 [6, 6, false]
                          Instr::Swap, // 5 [6, false, 6]
                          Instr::IntLit(1), // 6 [6, false, 6, 1]
                          Instr::Eq, // 7 [6, false, false]
                          Instr::Or, // 8 [6, false]
                          Instr::Dbg,
                          Instr::If, // 9 [6]
                          Instr::Ret, // 10 done
                          Instr::Dup(0), // 11 [6, 6]
                          Instr::IntLit(1), // 12 [6, 6, 1]
                          Instr::SubInt, // 13 [6, 5]
                          Instr::IntLit(1), // 14 [6, 5, 1]
                          Instr::Call(0), // 15 [6, 5]
                          Instr::Swap, // 16 [5, 6]
                          Instr::IntLit(2), // 16 [5, 6, 2]
                          Instr::SubInt, // 17 [5, 4]
                          Instr::IntLit(1), // 18 [5, 4, 1]
                          Instr::Call(0), // 19 [5, 3]
                          Instr::AddInt, // 20 [8]
                          Instr::Ret /* 21 finish this execution */],
                        1, &mut ())
      .unwrap();
    assert!(vm.stack.len() == 1);
    assert_eq!(vm.stack.pop().unwrap(), Value::Int(610));
}

#[test]
fn test_stack() {
    let mut stack = Stack::new();
    stack.push(Value::Int(5)).unwrap();
    stack.push(Value::Int(10)).unwrap();
    assert_eq!(stack.pop().unwrap(), Value::Int(10));
    assert_eq!(stack.pop().unwrap(), Value::Int(5));
    stack.push(Value::Int(15)).unwrap();
    assert_eq!(stack.pop().unwrap(), Value::Int(15));

    let mut stack = Stack::new();
    stack.push(Value::Int(5)).unwrap();
    assert_eq!(stack.peek().unwrap(), &mut Value::Int(5));
}

#[test]
fn test_optimizations() {
    // IntLit - AddInt
    //
    let mut vm = Vm::<()>::new();
    vm.stack.push(Value::Int(5)).unwrap();
    vm.load_and_execute(&[Instr::IntLit(10), Instr::AddInt], 1, &mut ()).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, Value::Int(15));
    assert_eq!(vm.stack.pop_count(), 1);  // This pop is done in the test
    assert_eq!(vm.stack.push_count(), 1); // this push is done in the test

    // IntLit - SubInt
    //
    let mut vm = Vm::<()>::new();
    vm.stack.push(Value::Int(10)).unwrap();
    vm.load_and_execute(&[Instr::IntLit(5), Instr::SubInt], 1, &mut ()).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, Value::Int(5));
    assert_eq!(vm.stack.pop_count(), 1);  // This pop is done in the test
    assert_eq!(vm.stack.push_count(), 1); // this push is done in the test

    // IntLit - MulInt
    //
    let mut vm = Vm::<()>::new();
    vm.stack.push(Value::Int(5)).unwrap();
    vm.load_and_execute(&[Instr::IntLit(10), Instr::MulInt], 1, &mut ()).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, Value::Int(50));
    assert_eq!(vm.stack.pop_count(), 1);  // This pop is done in the test
    assert_eq!(vm.stack.push_count(), 1); // this push is done in the test

    // IntLit - DivInt
    //
    let mut vm = Vm::<()>::new();
    vm.stack.push(Value::Int(10)).unwrap();
    vm.load_and_execute(&[Instr::IntLit(5), Instr::DivInt], 1, &mut ()).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, Value::Int(2));
    assert_eq!(vm.stack.pop_count(), 1);  // This pop is done in the test
    assert_eq!(vm.stack.push_count(), 1); // this push is done in the test

    // IntLit - Eq
    //
    let mut vm = Vm::<()>::new();
    vm.stack.push(Value::Int(10)).unwrap();
    vm.load_and_execute(&[Instr::IntLit(10), Instr::Eq], 1, &mut ()).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, Value::Bool(true));
    assert_eq!(vm.stack.pop_count(), 1);  // This pop is done in the test
    assert_eq!(vm.stack.push_count(), 1); // this push is done in the test

    // BoolLit - Eq
    //
    let mut vm = Vm::<()>::new();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.load_and_execute(&[Instr::BoolLit(true), Instr::Eq], 1, &mut ()).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, Value::Bool(true));
    assert_eq!(vm.stack.pop_count(), 1);  // This pop is done in the test
    assert_eq!(vm.stack.push_count(), 1); // this push is done in the test

    // SymbolLit - Eq
    let mut vm = Vm::<()>::new();
    let s = vm.interner.gensym();
    vm.stack.push(Value::Symbol(s)).unwrap();
    vm.load_and_execute(&[Instr::SymbolLit(s), Instr::Eq], 1, &mut ()).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, Value::Bool(true));
    assert_eq!(vm.stack.pop_count(), 1);  // This pop is done in the test
    assert_eq!(vm.stack.push_count(), 1); // this push is done in the test

    // Or - If
    //
    let mut vm = Vm::<()>::new();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.stack.push(Value::Bool(false)).unwrap();
    vm.load_and_execute(&[Instr::Or, Instr::If, Instr::IntLit(5)], 2, &mut ()).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, Value::Int(5));
    assert_eq!(vm.stack.pop_count(), 3);  // 1 from the test, two from popping the bools
    assert_eq!(vm.stack.push_count(), 3); // 2 from the test, one for the int-lit

    // Or - If
    //
    let mut vm = Vm::<()>::new();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.stack.push(Value::Bool(false)).unwrap();
    vm.load_and_execute(&[Instr::Or, Instr::Ifn, Instr::Nop, Instr::IntLit(5)], 2, &mut ()).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, Value::Int(5));
    assert_eq!(vm.stack.pop_count(), 3);  // 1 from the test, two from popping the bools
    assert_eq!(vm.stack.push_count(), 3); // 2 from the test, one for the int-lit

    // And - If
    //
    let mut vm = Vm::<()>::new();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.load_and_execute(&[Instr::And, Instr::If, Instr::IntLit(5)], 2, &mut ()).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, Value::Int(5));
    assert_eq!(vm.stack.pop_count(), 3);  // 1 from the test, two from popping the bools
    assert_eq!(vm.stack.push_count(), 3); // 2 from the test, one for the int-lit

    // Or - If
    //
    let mut vm = Vm::<()>::new();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.load_and_execute(&[Instr::And, Instr::Ifn, Instr::Nop, Instr::IntLit(5)], 2, &mut ()).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, Value::Int(5));
    assert_eq!(vm.stack.pop_count(), 3);  // 1 from the test, two from popping the bools
    assert_eq!(vm.stack.push_count(), 3); // 2 from the test, one for the int-lit
}

#[test]
fn load_constant() {
    let mut vm = Vm::<()>::new();
    let instr = vm.compile_context.add_constant("hello world".into());
    assert_eq!(instr, Instr::LoadConstant(0));
    vm.load_and_execute(&[instr], 0, &mut ()).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, "hello world".into());

    let instr = vm.compile_context.add_constant("bye world".into());
    assert_eq!(instr, Instr::LoadConstant(1));
    vm.load_and_execute(&[instr], 0, &mut ()).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, "bye world".into());
}

#[test]
fn basic_lambdas() {
    let mut vm = Vm::<()>::new();
    let closure_class_id = vm.compile_context.add_closure_class(ClosureClass {
        code_offset: 3,
        arg_count: 0,
        local_defines_count: 0,
        upvars_count: 0,
        has_rest_params: false,
        namespace: Default::default(),
    });

    vm.load_and_execute(&[
        Instr::CreateClosure(closure_class_id as u32),
        Instr::Execute(0), // 0 arguments
        Instr::Jump(5),
        Instr::IntLit(30),
        Instr::Ret
    ], 0, &mut ()).unwrap();
    println!("{:?}", vm.stack);
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, 30.into());
}

#[test]
fn one_arg_lambda() {
    let mut vm = Vm::<()>::new();
    let closure_class_id = vm.compile_context.add_closure_class(ClosureClass {
        code_offset: 4,
        arg_count: 1,
        local_defines_count: 0,
        upvars_count: 0,
        has_rest_params: false,
        namespace: Default::default(),
    });

    vm.load_and_execute(&[
        Instr::IntLit(10),
        Instr::CreateClosure(closure_class_id as u32),
        Instr::Execute(1), // 1 argument
        Instr::Jump(6),
        Instr::Dup(0),
        Instr::MulInt,
        Instr::Ret
    ], 0, &mut ()).unwrap();

    let result = vm.stack.pop().unwrap();
    assert_eq!(result, 100.into());
}
