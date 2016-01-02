mod intern;
mod lambda;
mod value;
mod stack;

use std::collections::HashMap;
use gc::Trace;

use compiler::CompileContext;

pub use vm::intern::*;
pub use vm::value::*;
pub use vm::lambda::*;
pub use vm::stack::*;

#[derive(Debug)]
pub enum InterpError {
    InternalInterpError(String),
    MismatchedType {
        value: Value,
        expected: ValueKind,
    },
    VariableNotFound(String),
    NoUpvars,
    StackOverflow,
    StackUnderflow,
    StackOutOfBounds,
    BadArity {
        got: u32,
        expected: u32
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceMap {
    values: HashMap<Symbol, Value>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Return {
    code_pos: u32,
    stack_frame: u32
}

#[derive(Debug)]
pub struct Vm {
    pub stack: Stack,
    return_stack: Vec<Return>,
    pub interner: intern::SymbolIntern,
    globals: ReferenceMap,
    code: Vec<Instr>,
    pub compile_context: CompileContext,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instr {
    /// Stop execution
    Halt,
    Nop,
    /// Prints the value at the top of the stack.
    ///
    /// (debug use only)
    Print,
    Dbg,
    /// Copy a value from N positions up the
    /// stack frame into the top of the stack.
    Dup(u32),
    DupTop,
    /// Push a value onto the stack
    /// Pop a value off of the stack
    Pop,
    /// Swap the top two values on the stack
    Swap,

    BoolLit(bool),
    SymbolLit(Symbol),
    IntLit(i32),
    LoadConstant(u32),

    /// Pop the top value and set it in the cell located
    /// at this position from the top of the stack
    SetCell(u32),
    /// Pop the top value and overwrite this position in the
    /// stack with that value.
    Assign(u32),

    /// Calls a function at the specified location.
    /// Pops the first argument off the top of the stack to
    /// be used for the argument list.
    Call(u32),
    /// Move the instruction pointer to a specified location
    Jump(u32),
    /// Clear the current frame (except for the item on the very top),
    /// move the instruction pointer back to where it was before the
    /// lambda was called.
    Ret,

    /// Creates a closure with the given class
    CreateClosure(u32),

    /// Adds two integers by popping two values off of
    /// the stack, adding them, and pushing the result
    /// back on the stack.
    AddInt,
    SubInt,
    DivInt,
    MulInt,

    And,
    Or,

    Eq,

    /// Execute a lambda on the top of the stack with
    /// a specified number of arguments
    ExecuteClosure(u32),
    /// Execute a lambda on the top of the stack with
    /// an amount of arguments equal to the *next*
    /// thing on the stack.
    ExecuteClosureN,

    /// Read a bool off the stack, if true, continue executing,
    /// else, skip the next instruction.
    If,

    /// Read a bool off the stack, if false, continue executing,
    /// else skip the next instruction.
    Ifn,

    /// Returns a symbol from the global scope
    FetchGlobal(Symbol),

    /// Returns a symbol that has been closed over
    FetchUpvar(Symbol),
}

unsafe impl Trace for ReferenceMap {
    custom_trace!(this, {
        for (_, v) in this.values.iter() {
            mark(v);
        }
    });
}

impl ReferenceMap {
    pub fn new() -> ReferenceMap {
        ReferenceMap { values: HashMap::new() }
    }

    pub fn put(&mut self, name: Symbol, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Symbol, interner: &mut SymbolIntern) -> Result<Value, InterpError> {
        self.values
            .get(&name)
            .map(Clone::clone)
            .ok_or_else(|| InterpError::VariableNotFound(interner.lookup_or_anon(name)))
    }
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            stack: Stack::new(),
            interner: intern::SymbolIntern::new(),
            globals: ReferenceMap::new(),
            return_stack: Vec::new(),
            code: Vec::new(),
            compile_context: ::compiler::CompileContext::new(),
        }
    }

    pub fn load_and_execute(&mut self, code: &[Instr], arg_count: u32) -> Result<(), InterpError> {
        let start = self.code.len() as u32;
        let stackframe = self.stack.len() - arg_count;
        self.code.extend(code.iter().cloned());
        self.execute(start, stackframe, None)
    }

    pub fn execute(&mut self,
                   start_at: u32,
                   mut stack_frame: u32,
                   _upvars: Option<&ReferenceMap>)
                   -> Result<(), InterpError> {
        let &mut Vm {
            ref mut stack,
            ref mut return_stack,
            ref mut interner,
            ref mut globals,
            ref mut code,
            ref compile_context,
        } = self;

        let mut i = start_at;
        while i < code.len() as u32 {
            let current_instruction = &code[i as usize];
            let after_current = code.get(i as usize + 1);
            // Here lay some optimizations
            if let Some(after) = after_current {
                let mut optimized = true;
                match (current_instruction, after) {
                    (&Instr::IntLit(_), &Instr::Pop) |
                    (&Instr::BoolLit(_), &Instr::Pop) |
                    (&Instr::SymbolLit(_), &Instr::Pop) |
                    (&Instr::LoadConstant(_), &Instr::Pop) => { }

                    (&Instr::IntLit(added_with), &Instr::AddInt) => {
                        let cur = try!(try!(stack.peek()).expect_int_ref_mut());
                        *cur = *cur + added_with as i64;
                    }
                    (&Instr::IntLit(subtract_by), &Instr::SubInt) => {
                        let cur = try!(try!(stack.peek()).expect_int_ref_mut());
                        *cur = *cur - subtract_by as i64;
                    }
                    (&Instr::IntLit(multiply_by), &Instr::MulInt) => {
                        let cur = try!(try!(stack.peek()).expect_int_ref_mut());
                        *cur = *cur * multiply_by as i64;
                    }
                    (&Instr::IntLit(divide_by), &Instr::DivInt) => {
                        let cur = try!(try!(stack.peek()).expect_int_ref_mut());
                        *cur = *cur / divide_by as i64;
                    }
                    (&Instr::IntLit(value), &Instr::Eq) => {
                        let cur = try!(stack.peek());
                        *cur = Value::Bool(*cur == Value::Int(value as i64));
                    }
                    (&Instr::BoolLit(value), &Instr::Eq) => {
                        let cur = try!(stack.peek());
                        *cur = Value::Bool(*cur == Value::Bool(value));
                    }
                    (&Instr::SymbolLit(value), &Instr::Eq) => {
                        let cur = try!(stack.peek());
                        *cur = Value::Bool(*cur == Value::Symbol(value));
                    }
                    (&Instr::Or, &Instr::If) => {
                        let a = try!(try!(stack.pop()).expect_bool());
                        let b = try!(try!(stack.pop()).expect_bool());
                        if !(a || b) {
                            i += 1;
                        }
                    }
                    (&Instr::Or, &Instr::Ifn) => {
                        let a = try!(try!(stack.pop()).expect_bool());
                        let b = try!(try!(stack.pop()).expect_bool());
                        if a || b {
                            i += 1;
                        }
                    }
                    (&Instr::And, &Instr::If) => {
                        let a = try!(try!(stack.pop()).expect_bool());
                        let b = try!(try!(stack.pop()).expect_bool());
                        if !(a && b) {
                            i += 1;
                        }
                    }
                    (&Instr::And, &Instr::Ifn) => {
                        let a = try!(try!(stack.pop()).expect_bool());
                        let b = try!(try!(stack.pop()).expect_bool());
                        if a && b {
                            i += 1;
                        }
                    }
                    _ => optimized = false,
                }

                if optimized {
                    i = i.wrapping_add(2);
                    continue;
                }
            }

            match current_instruction {
                &Instr::Halt => {
                    break;
                }
                &Instr::Nop => {}
                &Instr::Print => {
                    println!("{:?}", try!(stack.peek()));
                }
                &Instr::Dbg => {
                    print!("{:?} - ", &stack.as_slice()[.. stack_frame as usize]);
                    println!("{:?}", &stack.as_slice()[stack_frame as usize ..]);
                }
                &Instr::Dup(stack_pos) => {
                    let value = try!(stack.peek_n_up(stack_frame as usize + stack_pos as usize)).clone();
                    try!(stack.push(value));
                }
                &Instr::DupTop => {
                    let value = try!(stack.peek()).clone();
                    try!(stack.push(value));
                }
                &Instr::SetCell(frame_pos) => {
                    let value = try!(stack.pop());
                    let cell = try!(stack.peek_n_up(stack_frame as usize + frame_pos as usize));
                    let cell = try!(cell.expect_cell_ref());
                    let mut borrow = cell.borrow_mut();
                    *borrow = value;
                }
                &Instr::Assign(frame_pos) => {
                    {
                        let value = try!(stack.pop());
                        let cell = try!(stack.peek_n_up(stack_frame as usize + frame_pos as usize));
                        *cell = value;
                    }
                    println!("after assign");
                    print!("{:?} - ", &stack.as_slice()[.. stack_frame as usize]);
                    println!("{:?}", &stack.as_slice()[stack_frame as usize ..]);
                }
                &Instr::Pop => {
                    try!(stack.pop());
                }
                &Instr::Swap => {
                    let len = stack.len();
                    try!(stack.swap(len - 1, len - 2));
                }
                &Instr::BoolLit(b) => {
                    try!(stack.push(Value::Bool(b)));
                }
                &Instr::SymbolLit(symbol) => {
                    try!(stack.push(Value::Symbol(symbol)));
                }
                &Instr::IntLit(int) => {
                    try!(stack.push(Value::Int(int as i64)));
                }
                &Instr::LoadConstant(c_id)=> {
                    try!(stack.push(compile_context.get_constant(c_id)));
                }
                &Instr::Jump(location) => {
                    // subtract one because we'll be bumping
                    // it after the match is done.
                    i = location.wrapping_sub(1);
                }
                &Instr::AddInt => {
                    try!(stack.binop_int(|a, b| a + b));
                }
                &Instr::SubInt => {
                    try!(stack.binop_int(|a, b| a - b));
                }
                &Instr::MulInt => {
                    try!(stack.binop_int(|a, b| a * b));
                }
                &Instr::DivInt => {
                    try!(stack.binop_int(|a, b| a / b));
                }
                &Instr::And => {
                    let a = try!(try!(stack.pop()).expect_bool());
                    let b = try!(try!(stack.peek()).expect_bool_ref_mut());
                    *b = a && *b;
                }
                &Instr::Or => {
                    let a = try!(try!(stack.pop()).expect_bool());
                    let b = try!(try!(stack.peek()).expect_bool_ref_mut());
                    *b = a || *b;
                }
                &Instr::Eq => {
                    let a = try!(stack.pop());
                    let b = try!(stack.peek());
                    *b = Value::Bool(&a == b);
                }
                &Instr::ExecuteClosure(arg_count) => {
                    let closure = try!(try!(stack.pop()).expect_closure());
                    let code_pos = closure.class.code_offset;
                    let expected_arg_count = closure.class.arg_count;
                    let local_defines_count = closure.class.local_defines_count;
                    if closure.class.has_rest_params {
                        unimplemented!();
                    }

                    if arg_count != expected_arg_count {
                        return Err(InterpError::BadArity {
                            got: arg_count,
                            expected: expected_arg_count
                        });
                    }


                    return_stack.push(Return {
                        code_pos: i,
                        stack_frame: stack_frame
                    });

                    i = code_pos.wrapping_sub(1);
                    stack_frame = stack.len() as u32 - arg_count as u32;

                    for _ in 0 .. local_defines_count {
                        try!(stack.push(Value::Int(255)));
                    }
                }
                &Instr::CreateClosure(class_id) => {
                    let class = compile_context.get_lambda_class(class_id);
                    let upvar_nums = class.upvars_count;
                    let values = try!(stack.pop_n(upvar_nums as usize));
                    let instance = Closure { class: class, upvars: values};
                    try!(stack.push(instance.into()));
                }
                &Instr::ExecuteClosureN => {
                    /*
                    let lambda = try!(stack.pop().expect_lambda());
                    let arg_count = try!(stack.pop().expect_int());
                    let offset = lambda.code_offset;
                    return_stack.push(Return {
                        code_pos: i,
                        stack_frame: stack_frame
                    });
                    i = offset.wrapping_sub(1);
                    stack_frame = stack.len() as u32 - arg_count as u32;
                    */
                    unimplemented!();
                }
                &Instr::Call(position) => {
                    let arg_count = try!(try!(stack.pop()).expect_int());
                    let offset = position;
                    return_stack.push(Return {
                        code_pos: i,
                        stack_frame: stack_frame
                    });
                    i = offset.wrapping_sub(1);
                    stack_frame = stack.len() as u32 - arg_count as u32;
                }
                &Instr::Ret => {
                    let return_info = match return_stack.pop() {
                        Some(ri) => ri,
                        None => return Ok(())
                    };

                    i = return_info.code_pos as u32;
                    let return_value = try!(stack.pop());
                    try!(stack.truncate(stack_frame as usize));
                    try!(stack.push(return_value));
                    stack_frame = return_info.stack_frame;
                }
                &Instr::If => {
                    let b = try!(try!(stack.pop()).expect_bool());
                    if !b {
                        i += 1
                    }
                }
                &Instr::Ifn => {
                    let b = try!(try!(stack.pop()).expect_bool());
                    if b {
                        i += 1
                    }
                }
                &Instr::FetchGlobal(symbol) => {
                    let value = try!(globals.get(symbol, interner));
                    try!(stack.push(value));
                }
                &Instr::FetchUpvar(_symbol) => {
                    unimplemented!();
                }
            }
            i = i.wrapping_add(1);
        }

        assert!(stack_frame + 1 == stack.len() as u32,
                "'execute' left the stack at the wrong size: actual: {} vs expected: {}",
                stack.len(),
                stack_frame + 1);

        Ok(())
    }

    fn debug(&self) {
        println!("{:?}", self.stack);
    }
}

#[test]
fn assumptions() {
    use std::mem::size_of;
    assert_eq!(4, size_of::<Symbol>());
    assert_eq!(8, size_of::<Instr>());
    assert_eq!(32, size_of::<Value>());
}

#[test]
fn test_addint() {
    let mut vm = Vm::new();
    vm.stack.push(Value::Int(5)).unwrap();
    vm.stack.push(Value::Int(10)).unwrap();
    vm.load_and_execute(&[Instr::AddInt], 2).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, Value::Int(15));
}

#[test]
fn get_global() {
    let mut vm = Vm::new();
    let symbol = vm.interner.intern("hi");
    vm.globals.put(symbol, Value::Int(20));
    vm.load_and_execute(&[Instr::FetchGlobal(symbol)], 0).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, Value::Int(20));
}

#[test]
fn load_literals() {
    {
        let mut vm = Vm::new();
        let dummy = vm.interner.intern("dummy");
        vm.load_and_execute(&[Instr::SymbolLit(dummy)], 0).unwrap();

        let result = vm.stack.pop().unwrap();
        assert_eq!(result, Value::Symbol(dummy));
    }

    {
        let mut vm = Vm::new();
        vm.load_and_execute(&[Instr::BoolLit(true)], 0).unwrap();

        let result = vm.stack.pop().unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    {
        let mut vm = Vm::new();
        vm.load_and_execute(&[Instr::IntLit(5)], 0).unwrap();

        let result = vm.stack.pop().unwrap();
        assert_eq!(result, Value::Int(5));
    }
}

#[test]
fn test_jmp() {
    let mut vm = Vm::new();
    vm.load_and_execute(&[Instr::Jump(2), Instr::IntLit(5), Instr::IntLit(10)], 0)
      .unwrap();
    assert_eq!(vm.stack.pop().unwrap(), Value::Int(10));
    assert!(vm.stack.len() == 0);

}

#[test]
fn test_if() {
    {
        let mut vm = Vm::new();
        vm.stack.push(Value::Int(10)).unwrap();
        vm.stack.push(Value::Int(5)).unwrap();
        vm.stack.push(Value::Bool(true)).unwrap();

        vm.load_and_execute(&[Instr::If,
                     Instr::Jump(3),
                     Instr::Jump(5),
                     Instr::AddInt,
                     Instr::Halt,
                     Instr::SubInt],
                   3)
          .unwrap();
        assert_eq!(vm.stack.pop().unwrap(), Value::Int(15));
    }
    {
        let mut vm = Vm::new();
        vm.stack.push(Value::Int(10)).unwrap();
        vm.stack.push(Value::Int(5)).unwrap();
        vm.stack.push(Value::Bool(false)).unwrap();

        vm.load_and_execute(&[Instr::If,
                     Instr::Jump(3),
                     Instr::Jump(5),
                     Instr::AddInt,
                     Instr::Halt,
                     Instr::SubInt],
                   3)
          .unwrap();
        assert_eq!(vm.stack.pop().unwrap(), Value::Int(5));
    }
    {
        let mut vm = Vm::new();
        vm.stack.push(Value::Int(10)).unwrap();
        vm.stack.push(Value::Int(5)).unwrap();
        vm.stack.push(Value::Bool(true)).unwrap();

        vm.load_and_execute(&[Instr::Ifn,
                     Instr::Jump(3),
                     Instr::Jump(5),
                     Instr::AddInt,
                     Instr::Halt,
                     Instr::SubInt],
                   3)
          .unwrap();
        assert_eq!(vm.stack.pop().unwrap(), Value::Int(5));
    }
    {
        let mut vm = Vm::new();
        vm.stack.push(Value::Int(10)).unwrap();
        vm.stack.push(Value::Int(5)).unwrap();
        vm.stack.push(Value::Bool(false)).unwrap();

        vm.load_and_execute(&[Instr::Ifn,
                     Instr::Jump(3),
                     Instr::Jump(5),
                     Instr::AddInt,
                     Instr::Halt,
                     Instr::SubInt],
                   3)
          .unwrap();
        assert_eq!(vm.stack.pop().unwrap(), Value::Int(15));
    }
}

#[test]
fn test_swap() {
    let mut vm = Vm::new();
    vm.stack.push(Value::Int(10)).unwrap();
    vm.stack.push(Value::Int(5)).unwrap();
    vm.load_and_execute(&[Instr::Swap], 1).unwrap();
    assert_eq!(vm.stack.pop().unwrap(), Value::Int(10));
    assert_eq!(vm.stack.pop().unwrap(), Value::Int(5));
}

#[test]
fn test_and() {
    let mut vm = Vm::new();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.load_and_execute(&[Instr::And], 2).unwrap();
    assert_eq!(vm.stack.pop().unwrap(), Value::Bool(true));

    let mut vm = Vm::new();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.stack.push(Value::Bool(false)).unwrap();
    vm.load_and_execute(&[Instr::And], 2).unwrap();
    assert_eq!(vm.stack.pop().unwrap(), Value::Bool(false));

    let mut vm = Vm::new();
    vm.stack.push(Value::Bool(false)).unwrap();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.load_and_execute(&[Instr::And], 2).unwrap();
    assert_eq!(vm.stack.pop().unwrap(), Value::Bool(false));

    let mut vm = Vm::new();
    vm.stack.push(Value::Bool(false)).unwrap();
    vm.stack.push(Value::Bool(false)).unwrap();
    vm.load_and_execute(&[Instr::And], 2).unwrap();
    assert_eq!(vm.stack.pop().unwrap(), Value::Bool(false));
}

#[test]
fn test_or() {
    let mut vm = Vm::new();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.load_and_execute(&[Instr::Or], 2).unwrap();
    assert_eq!(vm.stack.pop().unwrap(), Value::Bool(true));

    let mut vm = Vm::new();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.stack.push(Value::Bool(false)).unwrap();
    vm.load_and_execute(&[Instr::Or], 2).unwrap();
    assert_eq!(vm.stack.pop().unwrap(), Value::Bool(true));

    let mut vm = Vm::new();
    vm.stack.push(Value::Bool(false)).unwrap();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.load_and_execute(&[Instr::Or], 2).unwrap();
    assert_eq!(vm.stack.pop().unwrap(), Value::Bool(true));

    let mut vm = Vm::new();
    vm.stack.push(Value::Bool(false)).unwrap();
    vm.stack.push(Value::Bool(false)).unwrap();
    vm.load_and_execute(&[Instr::Or], 2).unwrap();
    assert_eq!(vm.stack.pop().unwrap(), Value::Bool(false));
}

#[test]
fn test_lone_ret() {
    let mut vm = Vm::new();
    vm.stack.push(Value::Int(5)).unwrap();
    vm.load_and_execute(&[Instr::Ret], 0).unwrap();
    assert_eq!(vm.stack.pop().unwrap(), Value::Int(5));
}

#[test]
fn test_call() {
    let mut vm = Vm::new();
    vm.stack.push(Value::Int(5)).unwrap();
    vm.stack.push(Value::Int(10)).unwrap();

    vm.load_and_execute(
        &[
            Instr::IntLit(2), // 2 args
            Instr::Call(3),   // call the adding function
            Instr::Ret,       // return from main
            Instr::AddInt,    // add the numbers
            Instr::Ret,       // return from adding

        ], 2).unwrap();
    assert_eq!(vm.stack.pop().unwrap(), Value::Int(15));
}

#[test]
fn naive_fib() {
    let mut vm = Vm::new();
    vm.stack.push(Value::Int(15)).unwrap();
    vm.load_and_execute(
               &[
               Instr::Dbg,       //  0 [6]
               Instr::Dup(0),    //  1 [6, 6]
               Instr::Dup(0),    //  2 [6, 6, 6]
               Instr::IntLit(0), //  3 [6, 6, 6, 0]
               Instr::Eq,        //  4 [6, 6, false]
               Instr::Swap,      //  5 [6, false, 6]
               Instr::IntLit(1), //  6 [6, false, 6, 1]
               Instr::Eq,        //  7 [6, false, false]
               Instr::Or,        //  8 [6, false]
               Instr::Dbg,
               Instr::If,        //  9 [6]
               Instr::Ret,       //  10 done
               Instr::Dup(0),    // 11 [6, 6]
               Instr::IntLit(1), // 12 [6, 6, 1]
               Instr::SubInt,    // 13 [6, 5]
               Instr::IntLit(1), // 14 [6, 5, 1]
               Instr::Call(0),   // 15 [6, 5]
               Instr::Swap,      // 16 [5, 6]
               Instr::IntLit(2), // 16 [5, 6, 2]
               Instr::SubInt,    // 17 [5, 4]
               Instr::IntLit(1), // 18 [5, 4, 1]
               Instr::Call(0),   // 19 [5, 3]
               Instr::AddInt,    // 20 [8]
               Instr::Ret,       // 21 finish this execution
                 ], 1).unwrap();
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
    //
    // IntLit - AddInt
    //
    let mut vm = Vm::new();
    vm.stack.push(Value::Int(5)).unwrap();
    vm.load_and_execute(&[Instr::IntLit(10), Instr::AddInt], 1).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, Value::Int(15));
    assert_eq!(vm.stack.pop_count(), 1);  // This pop is done in the test
    assert_eq!(vm.stack.push_count(), 1); // this push is done in the test

    //
    // IntLit - SubInt
    //
    let mut vm = Vm::new();
    vm.stack.push(Value::Int(10)).unwrap();
    vm.load_and_execute(&[Instr::IntLit(5), Instr::SubInt], 1).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, Value::Int(5));
    assert_eq!(vm.stack.pop_count(), 1);  // This pop is done in the test
    assert_eq!(vm.stack.push_count(), 1); // this push is done in the test

    //
    // IntLit - MulInt
    //
    let mut vm = Vm::new();
    vm.stack.push(Value::Int(5)).unwrap();
    vm.load_and_execute(&[Instr::IntLit(10), Instr::MulInt], 1).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, Value::Int(50));
    assert_eq!(vm.stack.pop_count(), 1);  // This pop is done in the test
    assert_eq!(vm.stack.push_count(), 1); // this push is done in the test

    //
    // IntLit - DivInt
    //
    let mut vm = Vm::new();
    vm.stack.push(Value::Int(10)).unwrap();
    vm.load_and_execute(&[Instr::IntLit(5), Instr::DivInt], 1).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, Value::Int(2));
    assert_eq!(vm.stack.pop_count(), 1);  // This pop is done in the test
    assert_eq!(vm.stack.push_count(), 1); // this push is done in the test

    //
    // IntLit - Eq
    //
    let mut vm = Vm::new();
    vm.stack.push(Value::Int(10)).unwrap();
    vm.load_and_execute(&[Instr::IntLit(10), Instr::Eq], 1).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, Value::Bool(true));
    assert_eq!(vm.stack.pop_count(), 1);  // This pop is done in the test
    assert_eq!(vm.stack.push_count(), 1); // this push is done in the test

    //
    // BoolLit - Eq
    //
    let mut vm = Vm::new();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.load_and_execute(&[Instr::BoolLit(true), Instr::Eq], 1).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, Value::Bool(true));
    assert_eq!(vm.stack.pop_count(), 1);  // This pop is done in the test
    assert_eq!(vm.stack.push_count(), 1); // this push is done in the test

    //
    // SymbolLit - Eq
    let mut vm = Vm::new();
    let s = vm.interner.gen_sym();
    vm.stack.push(Value::Symbol(s)).unwrap();
    vm.load_and_execute(&[Instr::SymbolLit(s), Instr::Eq], 1).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, Value::Bool(true));
    assert_eq!(vm.stack.pop_count(), 1);  // This pop is done in the test
    assert_eq!(vm.stack.push_count(), 1); // this push is done in the test

    //
    // Or - If
    //
    let mut vm = Vm::new();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.stack.push(Value::Bool(false)).unwrap();
    vm.load_and_execute(&[Instr::Or, Instr::If, Instr::IntLit(5)], 2).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, Value::Int(5));
    assert_eq!(vm.stack.pop_count(), 3);  // 1 from the test, two from popping the bools
    assert_eq!(vm.stack.push_count(), 3); // 2 from the test, one for the int-lit

    //
    // Or - If
    //
    let mut vm = Vm::new();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.stack.push(Value::Bool(false)).unwrap();
    vm.load_and_execute(&[Instr::Or, Instr::Ifn, Instr::Nop, Instr::IntLit(5)], 2).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, Value::Int(5));
    assert_eq!(vm.stack.pop_count(), 3);  // 1 from the test, two from popping the bools
    assert_eq!(vm.stack.push_count(), 3); // 2 from the test, one for the int-lit

    //
    // And - If
    //
    let mut vm = Vm::new();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.load_and_execute(&[Instr::And, Instr::If, Instr::IntLit(5)], 2).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, Value::Int(5));
    assert_eq!(vm.stack.pop_count(), 3);  // 1 from the test, two from popping the bools
    assert_eq!(vm.stack.push_count(), 3); // 2 from the test, one for the int-lit

    //
    // Or - If
    //
    let mut vm = Vm::new();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.stack.push(Value::Bool(true)).unwrap();
    vm.load_and_execute(&[Instr::And, Instr::Ifn, Instr::Nop, Instr::IntLit(5)], 2).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, Value::Int(5));
    assert_eq!(vm.stack.pop_count(), 3);  // 1 from the test, two from popping the bools
    assert_eq!(vm.stack.push_count(), 3); // 2 from the test, one for the int-lit
}

#[test]
fn load_constant() {
    let mut vm = Vm::new();
    let instr = vm.compile_context.add_constant("hello world".into());
    assert_eq!(instr, Instr::LoadConstant(0));
    vm.load_and_execute(&[instr], 0).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, "hello world".into());

    let instr = vm.compile_context.add_constant("bye world".into());
    assert_eq!(instr, Instr::LoadConstant(1));
    vm.load_and_execute(&[instr], 0).unwrap();
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, "bye world".into());
}

#[test]
fn basic_lambdas() {
    let mut vm = Vm::new();
    let closure_class_id = vm.compile_context.add_closure_class(ClosureClass {
        code_offset: 3,
        arg_count: 0,
        local_defines_count: 0,
        upvars_count: 0,
        has_rest_params: false,
    });

    vm.load_and_execute(&[
        Instr::CreateClosure(closure_class_id as u32),
        Instr::ExecuteClosure(0), // 0 arguments
        Instr::Jump(5),
        Instr::IntLit(30),
        Instr::Ret
    ], 0).unwrap();
    println!("{:?}", vm.stack);
    let result = vm.stack.pop().unwrap();
    assert_eq!(result, 30.into());
}

#[test]
fn one_arg_lambda() {
    let mut vm = Vm::new();
    let closure_class_id = vm.compile_context.add_closure_class(ClosureClass {
        code_offset: 4,
        arg_count: 1,
        local_defines_count: 0,
        upvars_count: 0,
        has_rest_params: false,
    });

    vm.load_and_execute(&[
        Instr::IntLit(10),
        Instr::CreateClosure(closure_class_id as u32),
        Instr::ExecuteClosure(1), // 1 argument
        Instr::Jump(6),
        Instr::Dup(0),
        Instr::MulInt,
        Instr::Ret
    ], 0).unwrap();

    let result = vm.stack.pop().unwrap();
    assert_eq!(result, 100.into());
}
