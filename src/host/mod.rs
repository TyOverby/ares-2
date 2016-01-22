use super::vm::{Vm, Value, Symbol, SymbolIntern};

mod error;
mod state;

pub use self::error::*;
pub use self::state::State;

pub struct Context<S: State> {
    vm: Vm<S>,
}

pub struct LoadedContext<'a, S: State + 'a> {
    context: &'a mut Context<S>,
    state: &'a mut S
}

pub trait GlobalPath {
    fn into(self, default_symbol: Symbol, interner: &mut SymbolIntern) -> (Symbol, Symbol);
}

impl <S: State> Context<S> {
    pub fn new() -> Context<S> {
        Context {
            vm: Vm::new(),
        }
    }

    pub fn load<'a>(&'a mut self, state: &'a mut S) -> LoadedContext<'a, S> {
        LoadedContext::new(self, state)
    }

    pub fn set_global<P: GlobalPath>(&mut self, path: P, value: Value) -> Option<Value> {
        let default_namespace = self.vm.interner.precomputed.default_namespace;
        let (namespace, name) = path.into(default_namespace, &mut self.vm.interner);
        self.vm.globals.set(namespace, name, value)
    }

    pub fn get_global<P: GlobalPath>(&mut self, path: P) -> Option<&Value> {
        let default_namespace = self.vm.interner.precomputed.default_namespace;
        let (namespace, name) = path.into(default_namespace, &mut self.vm.interner);
        self.vm.globals.get(namespace, name)
    }

    pub fn get_global_mut<P: GlobalPath>(&mut self, path: P) -> Option<&mut Value> {
        let default_namespace = self.vm.interner.precomputed.default_namespace;
        let (namespace, name) = path.into(default_namespace, &mut self.vm.interner);
        self.vm.globals.get_mut(namespace, name)
    }
}

impl <'a, S: State> LoadedContext<'a, S> {
    fn new(ctx: &'a mut Context<S>, state: &'a mut S) -> LoadedContext<'a, S> {
        LoadedContext {
            context: ctx,
            state: state
        }
    }

    pub fn eval(&mut self, program: &str) -> AresResult<Value> {
        let instrs = {
            let &mut Vm{ ref mut compile_context, ref mut interner, .. }
                = &mut self.context.vm;
            try!(::compiler::compile(program, compile_context, interner))
        };

        let stack_size = self.context.vm.stack.len();
        try!(self.context.vm.load_and_execute(&instrs[..], 0, self.state));
        let result = try!(self.context.vm.stack.pop());
        assert!(stack_size == self.context.vm.stack.len());
        Ok(result)
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
    let mut ctx = Context::new();
    let mut lctx = ctx.load(&mut state);
    assert_eq!(lctx.eval("(+ 1 2 3)"), Ok(6.into()));
}

#[test]
fn test_globals() {
    let mut ctx = Context::<()>::new();

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
    let mut ctx = Context::new();
    ctx.set_global("foo", user_function(None,
        |state: &mut i64, _| {
            *state += 1;
            Value::Int(*state)
        }
    ));

    {
        let mut lctx = ctx.load(&mut state);
        let res = lctx.eval("(foo)");
        assert_eq!(res.unwrap(), 1.into());
    }
    assert_eq!(state, 1);
    {
        let mut lctx = ctx.load(&mut state);
        let res = lctx.eval("(foo)");
        assert_eq!(res.unwrap(), 2.into());
    }
    assert_eq!(state, 2);
}
