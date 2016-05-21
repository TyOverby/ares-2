mod lambda;
mod value;
mod stack;
mod module;
mod function;
#[cfg(test)]
mod test;

use std::marker::PhantomData;

use compiler::CompileContext;
use host::{State, EphemeralContext};

pub use vm::value::*;
pub use vm::lambda::*;
pub use vm::stack::*;
pub use vm::module::*;
pub use vm::function::*;

use ares_syntax::*;

#[derive(Debug, Eq, PartialEq)]
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
        expected: u32,
    },
    UserFnWithWrongStateType,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Return {
    code_pos: u32,
    stack_frame: u32,
    namespace: Symbol
}

#[derive(Debug)]
pub struct Vm<S: State = ()> {
    pub stack: Stack,
    return_stack: Vec<Return>,
    pub interner: SymbolIntern,
    pub globals: Globals,
    code: Vec<Instr>,
    pub compile_context: CompileContext,
    _phantom: PhantomData<S>,
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

    NilLit,
    BoolLit(bool),
    SymbolLit(Symbol),
    IntLit(i32),
    LoadConstant(u32),

    /// Searches for the current symbol in the global
    /// namespace and pushes it onto the stack
    GetGlobal(Symbol),
    /// Pops a value off of the stack and places it
    /// in the global namespace.
    PutGlobal(Symbol),

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

    /// Returns a symbol that has been closed over
    FetchUpvar(Symbol),
}

impl <S: State> Vm<S> {
    pub fn new() -> Vm<S> {
        Vm {
            stack: Stack::new(),
            interner: SymbolIntern::new(),
            globals: Globals::new(),
            return_stack: Vec::new(),
            code: Vec::new(),
            compile_context: ::compiler::CompileContext::new(),
            _phantom: PhantomData,
        }
    }

    pub fn load_and_execute(&mut self, code: &[Instr], arg_count: u32, state: &mut S) -> Result<(), InterpError> {
        let start = self.code.len() as u32;
        let stackframe = self.stack.len() - arg_count;
        let default_ns = self.interner.precomputed.default_namespace;
        self.code.extend(code.iter().cloned());
        self.execute(start, stackframe, default_ns, state)
    }

    fn execute(&mut self,
                   start_at: u32,
                   mut stack_frame: u32,
                   mut current_ns: Symbol,
                   state: &mut S)
                   -> Result<(), InterpError> {
        let &mut Vm {
            ref mut stack,
            ref mut return_stack,
            ref mut interner,
            ref mut globals,
            ref mut code,
            ref compile_context,
            ..
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
                    (&Instr::LoadConstant(_), &Instr::Pop) => {}

                    (&Instr::IntLit(added_with), &Instr::AddInt) => {
                        let cur = try!(try!(stack.peek()).expect_int_mut());
                        *cur = *cur + added_with as i64;
                    }
                    (&Instr::IntLit(subtract_by), &Instr::SubInt) => {
                        let cur = try!(try!(stack.peek()).expect_int_mut());
                        *cur = *cur - subtract_by as i64;
                    }
                    (&Instr::IntLit(multiply_by), &Instr::MulInt) => {
                        let cur = try!(try!(stack.peek()).expect_int_mut());
                        *cur = *cur * multiply_by as i64;
                    }
                    (&Instr::IntLit(divide_by), &Instr::DivInt) => {
                        let cur = try!(try!(stack.peek()).expect_int_mut());
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
                    print!("{:?} - ", &stack.as_slice()[..stack_frame as usize]);
                    println!("{:?}", &stack.as_slice()[stack_frame as usize..]);
                }
                &Instr::Dup(stack_pos) => {
                    let value = try!(stack.peek_n_up(stack_frame as usize + stack_pos as usize))
                                    .clone();
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
                &Instr::GetGlobal(symbol) => {
                    if let Some(value) = globals.get(current_ns, symbol).cloned() {
                        try!(stack.push(value));
                    } else {
                        return Err(InterpError::VariableNotFound(
                                       interner.lookup_or_anon(symbol)))
                    }
                }
                &Instr::PutGlobal(symbol) => {
                    let value = try!(stack.pop());
                    let value = value.cellify();
                    globals.set(current_ns, symbol, value);
                }
                &Instr::Assign(frame_pos) => {
                    let value = try!(stack.pop());
                    let cell = try!(stack.peek_n_up(stack_frame as usize + frame_pos as usize));
                    *cell = value;
                }
                &Instr::Pop => {
                    try!(stack.pop());
                }
                &Instr::Swap => {
                    let len = stack.len();
                    try!(stack.swap(len - 1, len - 2));
                }
                &Instr::NilLit => {
                    try!(stack.push(Value::Nil));
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
                &Instr::LoadConstant(c_id) => {
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
                    let b = try!(try!(stack.peek()).expect_bool_mut());
                    *b = a && *b;
                }
                &Instr::Or => {
                    let a = try!(try!(stack.pop()).expect_bool());
                    let b = try!(try!(stack.peek()).expect_bool_mut());
                    *b = a || *b;
                }
                &Instr::Eq => {
                    let a = try!(stack.pop());
                    let b = try!(stack.peek());
                    *b = Value::Bool(&a == b);
                }
                &Instr::Execute(arg_count) => {
                    let callable = try!(stack.pop());
                    let callable = callable.decell();
                    match callable {
                        Value::UserFn(ref gccell) => {
                            let args = try!(stack.take_top(arg_count));
                            let mut user_fn = gccell.borrow_mut();
                            let user_fn = user_fn.correct::<S>();
                            let mut user_fn = try!(user_fn.or(Err(
                                InterpError::UserFnWithWrongStateType)));
                            let mut ctx = EphemeralContext::new(globals, interner);
                            let result = user_fn.call(state, args, &mut ctx);
                            try!(stack.push(result));
                        }
                        Value::Closure(ref closure) => {
                            let code_pos = closure.class.code_offset;
                            let expected_arg_count = closure.class.arg_count;
                            let local_defines_count = closure.class.local_defines_count;
                            if closure.class.has_rest_params {
                                unimplemented!();
                            }

                            if arg_count != expected_arg_count {
                                return Err(InterpError::BadArity {
                                    got: arg_count,
                                    expected: expected_arg_count,
                                });
                            }

                            return_stack.push(Return {
                                code_pos: i,
                                stack_frame: stack_frame,
                                namespace: current_ns,
                            });

                            i = code_pos.wrapping_sub(1);
                            stack_frame = stack.len() as u32 - arg_count as u32;

                            for _ in 0 .. local_defines_count {
                                try!(stack.push(Value::Int(255)));
                            }
                        }
                        _ => panic!()
                    }
                }
                &Instr::CreateClosure(class_id) => {
                    let class = compile_context.get_lambda_class(class_id);
                    let upvar_nums = class.upvars_count;
                    let values = try!(stack.pop_n(upvar_nums as usize));
                    let instance = Closure { class: class, upvars: values};
                    try!(stack.push(instance.into()));
                }
                &Instr::ExecuteN => {
                    // let lambda = try!(stack.pop().expect_lambda());
                    // let arg_count = try!(stack.pop().expect_int());
                    // let offset = lambda.code_offset;
                    // return_stack.push(Return {
                    // code_pos: i,
                    // stack_frame: stack_frame
                    // });
                    // i = offset.wrapping_sub(1);
                    // stack_frame = stack.len() as u32 - arg_count as u32;
                    //
                    unimplemented!();
                }
                &Instr::Call(position) => {
                    let arg_count = try!(try!(stack.pop()).expect_int());
                    let offset = position;
                    return_stack.push(Return {
                        code_pos: i,
                        stack_frame: stack_frame,
                        namespace: current_ns,
                    });
                    i = offset.wrapping_sub(1);
                    stack_frame = stack.len() as u32 - arg_count as u32;
                }
                &Instr::Ret => {
                    let return_info = match return_stack.pop() {
                        Some(ri) => ri,
                        None => return Ok(()),
                    };
                    let Return { 
                        code_pos,
                        stack_frame: sf,
                        namespace
                    } = return_info;

                    i = code_pos as u32;
                    let return_value = try!(stack.pop());
                    try!(stack.truncate(stack_frame as usize));
                    try!(stack.push(return_value));
                    stack_frame = sf;
                    current_ns = namespace;
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

