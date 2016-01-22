extern crate ares;
use ares::vm::*;

fn main() {

    let mut vm = Vm::<()>::new();
    vm.stack.push(Value::Int(30)).unwrap();
    vm.load_and_execute(&[Instr::Dup(0), // 1 [6, 6]
                          Instr::Dup(0), // 2 [6, 6, 6]
                          Instr::IntLit(0), // 3 [6, 6, 6, 0]
                          Instr::Eq, // 4 [6, 6, false]
                          Instr::Swap, // 5 [6, false, 6]
                          Instr::IntLit(1), // 6 [6, false, 6, 1]
                          Instr::Eq, // 7 [6, false, false]
                          Instr::Or, // 8 [6, false]
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
    println!("{:?}", vm.stack.pop());
    println!("pushes: {}, pops: {}",
             vm.stack.push_count(),
             vm.stack.pop_count());
}
