#![allow(dead_code)]

mod intern;
mod lambda;
mod value;

use std::collections::HashMap;

pub use intern::*;
pub use value::*;
pub use lambda::*;

#[derive(Debug)]
pub enum InterpError {
    MismatchedType {
        value: Value,
        expected: ValueKind
    },
    VariableNotFound(String),
    NoUpvars,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceMap {
    values: HashMap<Symbol, Value>
}

#[derive(Debug)]
pub struct Vm {
    stack: Vec<Value>,
    interner: intern::SymbolIntern,
    globals: ReferenceMap
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instr {
    /// Stop execution
    Halt,
    /// Copy a value from N positions down the stack
    /// into the top of the stack
    Dup(u32),
    /// Push a value onto the stack
    /// Pop a value off of the stack
    Pop,

    LoadBoolLit(bool),
    LoadSymbolLit(Symbol),
    LoadIntLit(i32),

    Jump(u32),

    /// Adds two integers by popping two values off of
    /// the stack, adding them, and pushing the result
    /// back on the stack.
    AddInt,
    SubInt,
    DivInt,
    MulInt,

    /// Execute a lambda on the top of the stack with
    /// a specified number of arguments
    Execute(u32),
    /// Execute a lambda on the top of the stack with
    /// an amount of arguments equal to the *next*
    /// thing on the stack.
    ExecuteN,

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

impl ReferenceMap {
    pub fn new() -> ReferenceMap {
        ReferenceMap {
            values: HashMap::new()
        }
    }

    pub fn put(&mut self, name: Symbol, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Symbol, interner: &mut SymbolIntern) -> Result<Value, InterpError> {
        self.values.get(&name)
                   .map(Clone::clone)
                   .ok_or_else(|| InterpError::VariableNotFound(interner.lookup_or_anon(name)))
    }
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            stack: Vec::new(),
            interner: intern::SymbolIntern::new(),
            globals: ReferenceMap::new(),
        }
    }

    pub fn get(&self, i: u32) -> Value {
        self.stack.get(self.stack.len() - i as usize - 1)
                  .expect("Stack had no value for get")
                  .clone()
    }

    pub fn pop(&mut self) -> Value {
        self.stack.pop().expect("Stack had no value to pop")
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn execute(&mut self, instructions: &[Instr], arg_offset: u32, upvars: Option<&ReferenceMap>) -> Result<(), InterpError> {
        let prior_stack_size = self.stack.len() - arg_offset as usize;

        let mut i = 0u32;
        while i < instructions.len() as u32 {
            let current_instruction = &instructions[i as usize];
            match current_instruction {
                &Instr::Halt => {
                    break;
                }
                &Instr::Dup(stack_pos) => {
                    let value = self.get(stack_pos);
                    self.push(value);
                }
                &Instr::Pop => {
                    self.pop();
                }
                &Instr::LoadBoolLit(b) => {
                    self.push(Value::Bool(b));
                },
                &Instr::LoadSymbolLit(symbol) => {
                    self.push(Value::Symbol(symbol));
                },
                &Instr::LoadIntLit(int) => {
                    self.push(Value::Int(int as i64));
                },
                &Instr::Jump(location) => {
                    // subtract one because we'll be bumping
                    // it after the match is done.
                    i = location.wrapping_sub(1);
                }
                &Instr::AddInt => {
                    try!(self.binop_int(|a, b| a + b));
                },
                &Instr::SubInt => {
                    try!(self.binop_int(|a, b| a - b));
                },
                &Instr::MulInt => {
                    try!(self.binop_int(|a, b| a * b));
                },
                &Instr::DivInt => {
                    try!(self.binop_int(|a, b| a / b));
                },
                &Instr::Execute(arg_count) => {
                    let lambda = try!(self.pop().expect_lambda());
                    try!(self.execute(&lambda.code[..], arg_count, Some(&lambda.upvars)));
                }
                &Instr::ExecuteN => {
                    let lambda = try!(self.pop().expect_lambda());
                    let arg_count = try!(self.pop().expect_int());
                    try!(self.execute(&lambda.code[..], arg_count as u32, Some(&lambda.upvars)));
                }
                &Instr::If => {
                    let b = try!(self.pop().expect_bool());
                    if !b { i += 1 }
                }
                &Instr::Ifn => {
                    let b = try!(self.pop().expect_bool());
                    if b { i += 1 }
                }
                &Instr::FetchGlobal(symbol) => {
                    let value = try!(self.globals.get(symbol, &mut self.interner));
                    self.push(value);
                }
                &Instr::FetchUpvar(symbol) => {
                    if let Some(upvars) = upvars {
                        let value = try!(upvars.get(symbol, &mut self.interner));
                        self.push(value);
                    } else {
                        return Err(InterpError::NoUpvars)
                    }
                }
            };
            i += 1;
        }

        let current_stack_size = self.stack.len();
        assert!(current_stack_size == prior_stack_size + 1,
                "execute left the stack at the wrong size: actual: {} vs expected: {}",
                current_stack_size, prior_stack_size + 1);

        Ok(())
    }

    fn binop_int<F: FnOnce(i64, i64) -> i64>(&mut self, f: F) -> Result<(), InterpError> {
        let a = try!(self.pop().expect_int());
        let b = try!(self.pop().expect_int());
        self.push(Value::Int(f(a, b)));
        Ok(())
    }

    fn debug(&self) {
        println!("{:?}", self.stack);
    }
}

#[test]
fn assumptions() {
    use std::mem::size_of;
    assert_eq!(size_of::<Symbol>(), 4);
    assert_eq!(size_of::<Instr>(), 8);
    assert_eq!(size_of::<Value>(), 16);
}

#[test]
fn basic_addint() {
    let mut vm = Vm::new();
    vm.push(Value::Int(5));
    vm.push(Value::Int(10));
    vm.execute(&[Instr::AddInt], 2, None).unwrap();
    let result = vm.pop();
    assert_eq!(result, Value::Int(15));
}

#[test]
fn static_arg_count_basic_lambda() {
    use std::rc::Rc;

    let mut vm = Vm::new();
    vm.push(Value::Int(5));  // Number to add
    vm.push(Value::Int(10)); // Number to add

    let lambda = Lambda::new(ReferenceMap::new(), vec![Instr::AddInt]);

    vm.push(Value::Lambda(Rc::new(lambda)));

    vm.execute(&[Instr::Execute(2)], 3, None).unwrap();
    let result = vm.pop();
    assert_eq!(result, Value::Int(15));
}

#[test]
fn variable_arg_count_basic_lambda() {
    use std::rc::Rc;

    let mut vm = Vm::new();
    vm.push(Value::Int(5));  // Number to add
    vm.push(Value::Int(10)); // Number to add
    vm.push(Value::Int(2));  // Argument count

    let lambda = Lambda::new(ReferenceMap::new(), vec![Instr::AddInt]);

    vm.push(Value::Lambda(Rc::new(lambda)));

    vm.execute(&[Instr::ExecuteN], 4, None).unwrap();
    let result = vm.pop();
    assert_eq!(result, Value::Int(15));

}

#[test]
fn get_global() {
    let mut vm = Vm::new();
    let symbol = vm.interner.intern("hi");
    vm.globals.put(symbol, Value::Int(20));
    vm.execute(&[Instr::FetchGlobal(symbol)], 0, None).unwrap();
    let result = vm.pop();
    assert_eq!(result, Value::Int(20));
}

#[test]
fn load_literals() {
    {
        let mut vm = Vm::new();
        let dummy = vm.interner.intern("dummy");
        vm.execute(&[Instr::LoadSymbolLit(dummy)], 0, None).unwrap();

        let result = vm.pop();
        assert_eq!(result, Value::Symbol(dummy));
    }

    {
        let mut vm = Vm::new();
        vm.execute(&[Instr::LoadBoolLit(true)], 0, None).unwrap();

        let result = vm.pop();
        assert_eq!(result, Value::Bool(true));
    }

    {
        let mut vm = Vm::new();
        vm.execute(&[Instr::LoadIntLit(5)], 0, None).unwrap();

        let result = vm.pop();
        assert_eq!(result, Value::Int(5));
    }
}

#[test]
fn test_jmp() {
    let mut vm = Vm::new();
    vm.execute(&[Instr::Jump(2), Instr::LoadIntLit(5), Instr::LoadIntLit(10)], 0, None).unwrap();
    assert_eq!(vm.pop(), Value::Int(10));
    assert!(vm.stack.len() == 0);

}

#[test]
fn test_if() {
    {
        let mut vm = Vm::new();
        vm.push(Value::Int(10));
        vm.push(Value::Int(5));
        vm.push(Value::Bool(true));

        vm.execute(&[Instr::If, Instr::Jump(3), Instr::Jump(5), Instr::AddInt, Instr::Halt, Instr::SubInt], 3, None).unwrap();
        assert_eq!(vm.pop(), Value::Int(15));
    }
    {
        let mut vm = Vm::new();
        vm.push(Value::Int(10));
        vm.push(Value::Int(5));
        vm.push(Value::Bool(false));

        vm.execute(&[Instr::If, Instr::Jump(3), Instr::Jump(5), Instr::AddInt, Instr::Halt, Instr::SubInt], 3, None).unwrap();
        assert_eq!(vm.pop(), Value::Int(-5));
    }
    {
        let mut vm = Vm::new();
        vm.push(Value::Int(10));
        vm.push(Value::Int(5));
        vm.push(Value::Bool(true));

        vm.execute(&[Instr::Ifn, Instr::Jump(3), Instr::Jump(5), Instr::AddInt, Instr::Halt, Instr::SubInt], 3, None).unwrap();
        assert_eq!(vm.pop(), Value::Int(-5));
    }
    {
        let mut vm = Vm::new();
        vm.push(Value::Int(10));
        vm.push(Value::Int(5));
        vm.push(Value::Bool(false));

        vm.execute(&[Instr::Ifn, Instr::Jump(3), Instr::Jump(5), Instr::AddInt, Instr::Halt, Instr::SubInt], 3, None).unwrap();
        assert_eq!(vm.pop(), Value::Int(15));
    }
}
