use vm::{Vm, Value, Modules};
use ares_syntax::{Symbol, SymbolIntern};
use std::marker::PhantomData;

mod error;
mod state;

pub use self::error::*;
pub use self::state::State;

pub struct UnloadedContext<S: State> {
    pub(crate) vm: Vm<S>,
}

pub struct LoadedContext<'a, S: State + 'a> {
    pub(crate) context: &'a mut UnloadedContext<S>,
    pub(crate) state: &'a mut S
}

pub struct EphemeralContext<'a, S: ?Sized + State + 'a> {
    pub(crate) globals: &'a mut Modules,
    pub(crate) interner: &'a mut SymbolIntern,
    _phantom: PhantomData<S>
}

pub trait GlobalPath {
    fn into(self, default_symbol: Symbol, interner: &mut SymbolIntern) -> (Symbol, Symbol);
}

pub trait Context<S: State> {
    fn modules(&self) -> &Modules;
    fn modules_mut(&mut self) -> &mut Modules;
    fn interner(&self) -> &SymbolIntern;
    fn interner_mut(&mut self) -> &mut SymbolIntern;

    fn has_global<P: GlobalPath>(&mut self, path: P) -> bool {
        let default_namespace = self.interner_mut().precomputed.default_namespace;
        let (namespace, name) = path.into(default_namespace, self.interner_mut());
        self.modules().is_defined(namespace, name)
    }

    fn set_global<P: GlobalPath>(&mut self, path: P, value: Value) -> Option<Value> {
        let default_namespace = self.interner_mut().precomputed.default_namespace;
        let (namespace, name) = path.into(default_namespace, self.interner_mut());
        self.modules_mut().set(namespace, name, value)
    }

    fn get_global<P: GlobalPath>(&mut self, path: P) -> Option<&Value> {
        let default_namespace = self.interner_mut().precomputed.default_namespace;
        let (namespace, name) = path.into(default_namespace, self.interner_mut());
        self.modules_mut().get(namespace, name)
    }

    fn get_global_mut<P: GlobalPath>(&mut self, path: P) -> Option<&mut Value> {
        let default_namespace = self.interner_mut().precomputed.default_namespace;
        let (namespace, name) = path.into(default_namespace, self.interner_mut());
        self.modules_mut().get_mut(namespace, name)
    }

    fn format_value(&self, value: &Value) -> String {
        ::vm::to_string_helper(&value, self.interner())
    }

    fn format_error(&self, error: AresError) -> String {
        use ::vm::InterpError;
        use ::compiler::CompileError;
        use compiler::binding::BindingError;

        match error {
            AresError::CompileError(CompileError::ParseError(pe)) => format!("{:?}", pe),
            AresError::CompileError(CompileError::BindingError(BindingError::CouldNotBind(s, sp))) =>
                format!("CouldNotBind({}) at {:?}", self.interner().lookup_or_anon(s), sp),
            AresError::CompileError(CompileError::BindingError(BindingError::Multiple(es))) => {
                let mut s = String::new();
                for e in es {
                    s.push_str(&self.format_error(AresError::CompileError(CompileError::BindingError(e))));
                    s.push('\n')
                }
                s
            }
            AresError::CompileError(CompileError::BindingError(BindingError::AlreadyDefined(s))) =>
                format!("AlreadyDefined({})", self.interner().lookup_or_anon(s)),
            AresError::CompileError(CompileError::EmitError(_)) => unreachable!(),

            AresError::InterpError(InterpError::InternalInterpError(s)) =>
                format!("InternalInterpError({})", s),
            AresError::InterpError(InterpError::MismatchedType{value, expected}) =>
                format!("MismatchedType{{value: {}, expected: {:?}}}", self.format_value(&value), expected),
            AresError::InterpError(InterpError::VariableNotFound(s)) => format!("VariableNotFound({})", s),
            AresError::InterpError(InterpError::StackOverflow) => "StackOverflow".to_string(),
            AresError::InterpError(InterpError::StackUnderflow) => "StackUnderflow".to_string(),
            AresError::InterpError(InterpError::StackOutOfBounds) => "StackOutOfBounds".to_string(),
            AresError::InterpError(InterpError::IncomparableValues(a, b)) =>
                format!("Could not compare {} with {}", self.format_value(&a), self.format_value(&b)),
            AresError::InterpError(InterpError::BadArity{got, expected}) =>
                format!("BadArity{{got: {}, expected: {}}}", got, expected),
            AresError::InterpError(InterpError::UserFnWithWrongStateType) => "UserFnWithWrongStateType".to_string()
        }
    }
}

impl <'a, S: State> EphemeralContext<'a, S> {
    pub fn new(globals: &'a mut Modules, interner: &'a mut SymbolIntern) -> EphemeralContext<'a, S> {
        EphemeralContext {
            globals: globals,
            interner: interner,
            _phantom: PhantomData,
        }
    }
}

impl <S: State> UnloadedContext<S> {
    pub fn new() -> UnloadedContext<S> {
        UnloadedContext {
            vm: Vm::new(),
        }
    }

    pub fn load<'a>(&'a mut self, state: &'a mut S) -> LoadedContext<'a, S> {
        LoadedContext::new(self, state)
    }

    pub(crate) fn dump_vm_internals(&self) -> (Vec<Value>, Vec<::vm::Instr>, usize) {
        let stack = self.vm.stack.as_slice().iter().cloned().collect();
        let instructions = self.vm.code.clone();
        let instruction_pointer = self.vm.last_code_position;

        (stack, instructions, instruction_pointer)
    }
}

impl <S: State> Context<S> for UnloadedContext<S> {
    fn modules(&self) -> &Modules {
        &self.vm.globals
    }
    fn modules_mut(&mut self) -> &mut Modules {
        &mut self.vm.globals
    }
    fn interner(&self) -> &SymbolIntern {
        &self.vm.interner
    }
    fn interner_mut(&mut self) -> &mut SymbolIntern {
        &mut self.vm.interner
    }
}

impl <'a, S: State> Context<S> for EphemeralContext<'a, S> {
    fn modules(&self) -> &Modules {
        &self.globals
    }
    fn modules_mut(&mut self) -> &mut Modules {
        &mut self.globals
    }
    fn interner(&self) -> &SymbolIntern {
        &self.interner
    }
    fn interner_mut(&mut self) -> &mut SymbolIntern {
        &mut self.interner
    }
}

impl <'a, S: State> LoadedContext<'a, S> {
    fn new(ctx: &'a mut UnloadedContext<S>, state: &'a mut S) -> LoadedContext<'a, S> {
        LoadedContext {
            context: ctx,
            state: state
        }
    }

    pub(crate) fn dump_vm_internals(&self) -> (Vec<Value>, Vec<::vm::Instr>, usize) {
        self.context.dump_vm_internals()
    }

    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut S {
        &mut self.state
    }

    pub fn eval(&mut self, program: &str) -> AresResult<Option<Value>> {
        let emitted_code_size = self.context.vm.code.len();

        let instrs = {
            let &mut Vm{ ref mut compile_context, ref mut interner, ref globals, .. }
                = &mut self.context.vm;
            try!(::compiler::compile(program, compile_context, Some(globals), interner, emitted_code_size))
        };

        let previous_stack_size = self.context.vm.stack.len();
        try!(self.context.vm.load_and_execute(&instrs[..], 0, self.state));
        let new_stack_size = self.context.vm.stack.len();
        assert!(new_stack_size == previous_stack_size ||
                new_stack_size == previous_stack_size + 1);

        if new_stack_size == previous_stack_size + 1 {
            let result = try!(self.context.vm.stack.pop());
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }
}

impl <'a, S: State> Context<S> for LoadedContext<'a, S> {
    fn modules(&self) -> &Modules {
        &self.context.vm.globals
    }
    fn modules_mut(&mut self) -> &mut Modules {
        &mut self.context.vm.globals
    }
    fn interner(&self) -> &SymbolIntern {
        &self.context.vm.interner
    }
    fn interner_mut(&mut self) -> &mut SymbolIntern {
        &mut self.context.vm.interner
    }
}

impl GlobalPath for (Symbol, Symbol) {
    fn into(self, _default: Symbol, _interner: &mut SymbolIntern) -> (Symbol, Symbol) {
        self
    }
}

impl GlobalPath for Symbol {
    fn into(self, default: Symbol, _interner: &mut SymbolIntern) -> (Symbol, Symbol) {
        (default, self)
    }
}

impl <'a> GlobalPath for (&'a str, &'a str) {
    fn into(self, _default: Symbol, interner: &mut SymbolIntern) -> (Symbol, Symbol) {
        (interner.intern(self.0), interner.intern(self.1))
    }
}

impl <'a> GlobalPath for &'a str {
    fn into(self, default: Symbol, interner: &mut SymbolIntern) -> (Symbol, Symbol) {
        (default, interner.intern(self))
    }
}

#[test]
fn basic_context() {
    let mut state = ();
    let mut ctx = UnloadedContext::new();
    let mut lctx = ctx.load(&mut state);
    assert_eq!(lctx.eval("1 + 2 + 3"), Ok(Some(6.into())));
}

#[test]
fn test_globals() {
    let mut ctx = UnloadedContext::<()>::new();

    ctx.set_global("foo", Value::Int(5));
    assert_eq!(ctx.get_global("foo").cloned().unwrap(), 5.into());
    assert!(ctx.get_global(("ns", "foo")).is_none());

    ctx.set_global(("ns", "foo"), Value::Int(10));
    assert_eq!(ctx.get_global("foo").cloned().unwrap(), 5.into());
    assert_eq!(ctx.get_global(("ns", "foo")).cloned().unwrap(), 10.into());
}

#[test]
fn context_with_user_fn() {
    use vm::user_function;
    let mut state = 0i64;
    let mut ctx = UnloadedContext::new();
    ctx.set_global("foo", user_function(None,
        |_, state: &mut i64, _| {
            *state += 1;
            Value::Int(*state)
        }
    ));

    {
        let mut lctx = ctx.load(&mut state);
        let res = lctx.eval("foo()");
        assert_eq!(res.unwrap(), Some(1.into()));
    }
    assert_eq!(state, 1);
    {
        let mut lctx = ctx.load(&mut state);
        let res = lctx.eval("foo()");
        assert_eq!(res.unwrap(), Some(2.into()));
    }
    assert_eq!(state, 2);
}
