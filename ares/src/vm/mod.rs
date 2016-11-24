mod value;
mod util;
mod concept;
#[cfg(test)]
mod test;

use std::marker::PhantomData;
use std::cell::RefCell;

use compiler::{CompileContext, ShiftMeta};
use host::{State, EphemeralContext};

pub use vm::value::*;
pub use vm::concept::lambda::*;
pub use vm::util::stack::*;
pub use vm::util::module::*;
pub use vm::concept::function::*;
pub use vm::concept::continuation::*;
pub use gc::Gc;

use ares_syntax::*;

const SHOULD_PRINT: bool = false;

#[derive(Debug, Eq, PartialEq)]
pub enum InterpError {
    InternalInterpError(String),
    MismatchedType {
        value: Value,
        expected: ValueKind,
    },
    IncomparableValues(Value, Value),
    VariableNotFound(String),
    StackOverflow,
    StackUnderflow,
    StackOutOfBounds,
    BadArity {
        got: u32,
        expected: u32,
    },
    UserFnWithWrongStateType,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Frame {
    resume_code_pos: usize,
    stack_frame: u32,
    namespace: Symbol,
    reset_symbols: Option<Vec<Symbol>>
}

#[derive(Debug)]
pub struct Vm<S: State = ()> {
    pub(crate) stack: Stack,
    pub(crate) frames: Vec<Frame>,
    pub(crate) interner: SymbolIntern,
    pub(crate) globals: Modules,
    pub(crate) code: Vec<Instr>,
    pub(crate) compile_context: CompileContext,
    pub(crate) last_code_position: usize,
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

    /// Pops n symbols off the stack and pushes it
    /// into the reset stack.
    Reset(u32),
    /// Pops a symbol off the stack and shifts on
    /// that symbol.
    Shift(u32),

    /// Push nil on to the stack.
    NilLit,
    /// Push a boolean literal on to the stack.
    BoolLit(bool),
    /// Push a symbol on to the stack.
    SymbolLit(Symbol),
    /// Push an integer literal on to the stack.
    ///
    /// Note that not all possible int literals
    /// are able to be represented as an i32.
    /// The rest must be loaded from LoadConstant.
    IntLit(i32),
    /// Loads a constant from the pool of constants.
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
    /// Pop the value at the top of the stack, and assuming that
    /// it is a Cell, take the value out of the cell and place it
    /// back on the stack
    UnwrapCell,
    /// Pops a value at the top of the stack, wraps it with a cell,
    /// and puts it back on the stack.
    WrapCell,

    /// Calls a function at the specified location.
    /// Pops the first argument off the top of the stack to
    /// be used for the argument list.
    Call(u32),
    /// Move the instruction pointer to a specified location
    Jump(u32),
    /// Pops an int off the stack and jumps to that location in code.
    JumpTo,
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
    /// See AddInt, but with subtraction
    SubInt,
    /// See AddInt, but with division
    DivInt,
    /// See AddInt, but with multiplication
    MulInt,

    And,
    Or,

    Lt,
    Lte,
    Gt,
    Gte,
    Eq,
    Neq,

    /// Create a list of the given size by popping
    /// that many elements off the stack
    ConstructList(u32),

    /// Pops a number off the stack.  Pops a list off the stack.
    /// Uses the number to index the list.
    ListIndex,

    /// Execute a lambda on the top of the stack with
    /// a specified number of arguments
    Execute(u32),

    /// Execute a lambda on the top of the stack with
    /// an amount of arguments equal to the *top* value
    /// on the stack.
    ExecuteN,

    /// Read a bool off the stack, if true, continue executing,
    /// else, skip the next instruction.
    If,

    /// Read a bool off the stack, if false, continue executing,
    /// else skip the next instruction.
    Ifn,
}

impl Instr {
	fn smart_print(&self, interner: &SymbolIntern) -> String {
        match self {
            &Instr::SymbolLit(s) => format!("SymbolLit({})", interner.lookup_or_anon(s)),
            &Instr::GetGlobal(s) => format!("GetGlobal({})", interner.lookup_or_anon(s)),
            &Instr::PutGlobal(s) => format!("PutGlobal({})", interner.lookup_or_anon(s)),
            other => format!("{:?}", other),
        }
	}
}

fn compare<I, F>(a: &Value, b: Value, i: I, f: F) -> Result<bool, InterpError>
where I: FnOnce(i64, i64) -> bool,
      F: FnOnce(f64, f64) -> bool {
    Ok(match (a, b) {
        (&Value::Int(ai), Value::Int(bi)) => i(ai, bi),
        (&Value::Float(af), Value::Float(bf)) => f(af, bf),
        (&Value::Int(ai), Value::Float(bf)) => f(ai as f64, bf),
        (&Value::Float(af), Value::Int(bi)) => f(af, bi as f64),
        (a, b) => return Err(InterpError::IncomparableValues(a.clone(), b)),
    })
}

impl <S: State> Vm<S> {
    pub fn new() -> Vm<S> {
        Vm {
            stack: Stack::new(),
            frames: vec![],
            globals: Modules::new(),
            code: Vec::new(),
            interner: SymbolIntern::new(),
            compile_context: ::compiler::CompileContext::new(),
            last_code_position: 0,
            _phantom: PhantomData,
        }
    }

    pub fn load_and_execute(&mut self, code: &[Instr], arg_count: u32, state: &mut S) -> Result<(), InterpError> {
        let start = self.code.len() as u32;
        let default_ns = self.interner.precomputed.default_namespace;

        let base_frame = Frame {
            resume_code_pos: 0,
            stack_frame: self.stack.len() - arg_count,
            namespace: default_ns,
            reset_symbols: None,
        };
        self.frames.push(base_frame);

        self.code.extend(code.iter().cloned());
        let r = self.execute(start, state);
        self.frames.pop();
        r
    }

    fn execute(&mut self,
                   start_at: u32,
                   state: &mut S)
                   -> Result<(), InterpError> {

        struct ExecCtx<'a, S: State + 'a> {
            i: &'a mut usize,
            code: &'a [Instr],
            stack: &'a mut Stack,
            globals: &'a mut Modules,
            interner: &'a mut SymbolIntern,
            compile_context: &'a CompileContext,
            frames: &'a mut Vec<Frame>,
            state: &'a mut S,
        }

        #[inline(always)]
        fn step<'a, S: State>(ctx: &mut ExecCtx<'a, S>) -> Result<bool, InterpError> {
            let &mut ExecCtx {
                ref mut i,
                ref code,
                ref mut stack,
                ref mut globals,
                ref mut interner,
                ref compile_context,
                ref mut frames,
                ref mut state
            } = ctx;
            let i: &mut usize = *i;

            let current_instruction = &code[*i];
            let after_current = code.get(*i + 1);
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
                            *i += 1;
                        }
                    }
                    (&Instr::Or, &Instr::Ifn) => {
                        let a = try!(try!(stack.pop()).expect_bool());
                        let b = try!(try!(stack.pop()).expect_bool());
                        if a || b {
                            *i += 1;
                        }
                    }
                    (&Instr::And, &Instr::If) => {
                        let a = try!(try!(stack.pop()).expect_bool());
                        let b = try!(try!(stack.pop()).expect_bool());
                        if !(a && b) {
                            *i += 1;
                        }
                    }
                    (&Instr::And, &Instr::Ifn) => {
                        let a = try!(try!(stack.pop()).expect_bool());
                        let b = try!(try!(stack.pop()).expect_bool());
                        if a && b {
                            *i += 1;
                        }
                    }
                    _ => optimized = false,
                }

                if optimized {
                    *i = i.wrapping_add(2);
                    return Ok(true);
                }
            }

            match current_instruction {
                &Instr::Halt => {
                    return Ok(false)
                }
                &Instr::Reset(n) => {
                    let n = n as usize;
                    let mut symbols = Vec::with_capacity(n);
                    for value in try!(stack.pop_n(n)) {
                        symbols.push(try!(value.expect_symbol()));
                    }

                    let closure = try!(try!(stack.peek()).expect_closure_mut());
                    *closure.reset_symbols.borrow_mut() = Some(symbols);
                }
                &Instr::Shift(shift_id) => {
                    fn symbols_intersect(xs: &[Symbol], ys: &[Symbol]) -> bool {
                        for &x in xs {
                            if ys.iter().any(|&y| x == y) {
                                return true;
                            }
                        }
                        return false;
                    }

                    let ShiftMeta{ num_symbols, return_pos } = compile_context.get_shift_meta(shift_id);

                    // The top item on the stack will be a closure, and we want it to stay on the
                    // top, so we'll pop it, then pop our symbols, *then* put it back on at the
                    // very end.
                    let closure = try!(stack.pop());

                    let mut shifting_symbols = Vec::with_capacity(num_symbols as usize);
                    for value in try!(stack.pop_n(num_symbols as usize)) {
                        shifting_symbols.push(try!(value.expect_symbol()));
                    }

                    let mut saved_frames = Vec::new();
                    let saved_stack_len: u32;

                    loop {
                        let next = frames.pop().expect("ran out of frames");
                        let done = match &next.reset_symbols {
                            &Some(ref s) => symbols_intersect(s, &shifting_symbols),
                            &None => false
                        };

                        if done {
                            saved_stack_len = next.stack_frame;
                            saved_frames.push(next);
                            break;
                        } else {
                            saved_frames.push(next);
                        }
                    }

                    saved_frames.reverse();

                    let saved_stack = try!(stack.keep(saved_stack_len));

                    let cont = Continuation {
                        instruction_pos: return_pos,
                        saved_stack: saved_stack,
                        saved_stack_frames: saved_frames,
                    };

                    try!(stack.push(Value::Continuation(Gc::new(cont))));
                    try!(stack.push(closure));
                }
                &Instr::Nop => {}
                &Instr::Print => {
                    println!("{:?}", try!(stack.peek()));
                }
                &Instr::Dbg => {
                    print!("{:?} - ", &stack.as_slice()[.. frames.last().unwrap().stack_frame as usize]);
                    println!("{:?}", &stack.as_slice()[frames.last().unwrap().stack_frame as usize..]);
                }
                &Instr::Dup(stack_pos) => {
                    let value = try!(stack.peek_n_up(frames.last().unwrap().stack_frame as usize + stack_pos as usize))
                                    .clone();
                    try!(stack.push(value));
                }
                &Instr::DupTop => {
                    let value = try!(stack.peek()).clone();
                    try!(stack.push(value));
                }
                &Instr::SetCell(frame_pos) => {
                    let value = try!(stack.pop());
                    let cell = try!(stack.peek_n_up(frames.last().unwrap().stack_frame as usize + frame_pos as usize));
                    let cell = try!(cell.expect_cell_ref());
                    let mut borrow = cell.borrow_mut();
                    *borrow = value;
                }
                &Instr::WrapCell => {
                    let value = try!(stack.peek());
                    let mut swap = Value::Nil;
                    ::std::mem::swap(value, &mut swap);
                    swap = swap.cellify();
                    ::std::mem::swap(value, &mut swap);
                    ::std::mem::forget(swap);
                }
                &Instr::UnwrapCell => {
                    let slot = try!(stack.peek());
                    let value = {
                        let cell = try!(slot.expect_cell_ref());
                        let value: &Value = &*cell.borrow();
                        value.clone()
                    };
                    *slot = value;
                }
                &Instr::ConstructList(n) => {
                    let elements = try!(stack.take_top(n));
                    let list = Gc::new(elements);
                    try!(stack.push(Value::List(list)));
                }
                &Instr::ListIndex => {
                    let idx = try!(try!(stack.pop()).expect_int());
                    let lst = try!(try!(stack.pop()).expect_list());
                    let value = lst[idx as usize].clone();
                    try!(stack.push(value));
                }
                &Instr::GetGlobal(symbol) => {
                    if let Some(value) = globals.get(frames.last().unwrap().namespace, symbol).cloned() {
                        try!(stack.push(value));
                    } else {
                        return Err(InterpError::VariableNotFound(
                                       interner.lookup_or_anon(symbol)))
                    }
                }
                &Instr::PutGlobal(symbol) => {
                    let value = try!(stack.pop());
                    globals.set(frames.last().unwrap().namespace, symbol, value);
                }
                &Instr::Assign(frame_pos) => {
                    let value = try!(stack.pop());
                    let cell = try!(stack.peek_n_up(frames.last().unwrap().stack_frame as usize + frame_pos as usize));
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
                    *i = location.wrapping_sub(1) as usize;
                }
                &Instr::JumpTo => {
                    let location = try!(try!(stack.pop()).expect_int());
                    *i = location.wrapping_sub(1) as usize;
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
                &Instr::Lt => {
                    let a = try!(stack.pop());
                    let b = try!(stack.peek());
                    *b = Value::Bool(try!(compare(b, a, |a, b| a < b, |a, b| a < b)));
                }
                &Instr::Lte => {
                    let a = try!(stack.pop());
                    let b = try!(stack.peek());
                    *b = Value::Bool(try!(compare(b, a, |a, b| a <= b, |a, b| a <= b)));
                }
                &Instr::Gt => {
                    let a = try!(stack.pop());
                    let b = try!(stack.peek());
                    *b = Value::Bool(try!(compare(b, a, |a, b| a > b, |a, b| a > b)));
                }
                &Instr::Gte => {
                    let a = try!(stack.pop());
                    let b = try!(stack.peek());
                    *b = Value::Bool(try!(compare(b, a, |a, b| a >= b, |a, b| a >= b)));
                }
                &Instr::Eq => {
                    let a = try!(stack.pop());
                    let b = try!(stack.peek());
                    *b = Value::Bool(&a == b);
                }
                &Instr::Neq => {
                    let a = try!(stack.pop());
                    let b = try!(stack.peek());
                    *b = Value::Bool(&a != b);
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
                            let code_pos = closure.class.code_offset as usize;
                            let expected_arg_count = closure.class.arg_count;
                            let local_defines_count = closure.class.local_defines_count;
                            let new_namespace = closure.class.namespace;

                            if closure.class.has_rest_params { unimplemented!(); }

                            if arg_count != expected_arg_count {
                                return Err(InterpError::BadArity {
                                    got: arg_count,
                                    expected: expected_arg_count,
                                });
                            }

                            if !closure.class.is_shifter {
                                let last_item_on_stack = frames.last_mut().unwrap();
                                last_item_on_stack.resume_code_pos = *i;
                            }

                            frames.push(Frame {
                                resume_code_pos: 0,
                                stack_frame: stack.len() as u32 - arg_count as u32,
                                namespace: new_namespace, //
                                reset_symbols: closure.reset_symbols.borrow().clone(),
                            });

                            *i = code_pos.wrapping_sub(1);

                            for v in &closure.upvars {
                                try!(stack.push(v.clone()));
                            }

                            for _ in 0 .. local_defines_count {
                                try!(stack.push(Value::Nil));
                            }
                        }
                        Value::Continuation(ref c) => {
                            let &Continuation{
                                instruction_pos,
                                ref saved_stack,
                                ref saved_stack_frames,
                            } = &**c;

                            // The continuation can be resumed with either 0 args or
                            // 1 argument.  If we have no args passed, resume with
                            // a nil.
                            let arg = if arg_count == 1 {
                                try!(stack.pop())
                            } else {
                                Value::Nil
                            };

                            {
                                let last_item_on_stack = frames.last_mut().unwrap();
                                last_item_on_stack.resume_code_pos = *i;
                            }

                            let current_top = stack.len();
                            let prev_top = saved_stack_frames.first().map(|sf| sf.stack_frame).unwrap_or(0);
                            for v in saved_stack {
                                let v = v.clone();
                                try!(stack.push(v));
                            }

                            try!(stack.push(arg));

                            for r in saved_stack_frames.into_iter() {
                                let mut r = r.clone();
                                r.stack_frame -= prev_top;
                                r.stack_frame += current_top;
                                frames.push(r);
                            }

                            *i = (instruction_pos as usize).wrapping_sub(1);
                        }
                        o => panic!("tried to call value ({:?})", o),
                    }
                }
                &Instr::CreateClosure(class_id) => {
                    let class = compile_context.get_lambda_class(class_id);
                    let upvar_nums = class.upvars_count;
                    let values = try!(stack.pop_n(upvar_nums as usize));
                    let instance = Closure { class: class, upvars: values, reset_symbols: RefCell::new(None)};
                    try!(stack.push(instance.into()));
                }
                &Instr::ExecuteN => {
                    unimplemented!();
                }
                &Instr::Call(position) => {
                    let arg_count = try!(try!(stack.pop()).expect_int());
                    let offset = position as usize;
                    let ns = frames.last().unwrap().namespace;

                    {
                        frames.last_mut().unwrap().resume_code_pos = *i;
                    }

                    frames.push(Frame {
                        resume_code_pos: 0,
                        stack_frame: stack.len() as u32 - arg_count as u32,
                        namespace: ns,
                        reset_symbols: None,
                    });

                    *i = offset.wrapping_sub(1);
                }
                &Instr::Ret => {
                    let cur = frames.pop().unwrap();
                    let truncate_to = cur.stack_frame;
                    if frames.len() == 0 {
                        return Ok(false);
                    }

                    let &Frame { resume_code_pos, .. } = frames.last().unwrap();

                    *i = resume_code_pos;
                    let return_value = try!(stack.pop());

                    try!(stack.truncate(truncate_to as usize));
                    try!(stack.push(return_value));
                }
                &Instr::If => {
                    let b = try!(try!(stack.pop()).expect_bool());
                    if !b {
                        *i += 1
                    }
                }
                &Instr::Ifn => {
                    let b = try!(try!(stack.pop()).expect_bool());
                    if b {
                        *i += 1
                    }
                }
            }

            *i = i.wrapping_add(1);
            Ok(true)
        }

        let mut i = start_at as usize;

        let mut ctx = ExecCtx {
            i: &mut i,
            code: &self.code,
            stack: &mut self.stack,
            globals: &mut self.globals,
            interner: &mut self.interner,
            compile_context: &self.compile_context,
            frames: &mut self.frames,
            state: state,
        };

        while *(ctx.i) < ctx.code.len(){
            if SHOULD_PRINT {
                println!("\n\nSTACK");
                for value in ctx.stack.as_slice() {
                    println!("*  {:?}", value);
                }
                println!("RETURN-STACK");
                for value in ctx.frames.as_slice() {
                    println!("*  {:?}", value);
                }
                println!("INSTRUCTIONS");
                for (k, instr) in ctx.code.iter().enumerate() {
                    let padding = if *ctx.i == k { "> " } else { "  " };
                    println!("{:03}{}{}", k, padding, instr.smart_print(ctx.interner));
                }
            }

            match step(&mut ctx) {
                Ok(true) => {}
                Ok(false) => { break; }
                Err(e) => {
                    self.last_code_position = *ctx.i;
                    return Err(e);
                }
            }
        }

        Ok(())
    }

    fn debug(&self) {
        println!("{:?}", self.stack);
    }
}
