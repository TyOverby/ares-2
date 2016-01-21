use super::vm::{Vm, Value};

mod error;

pub use self::error::*;

pub struct Context<S> {
    vm: Vm<S>,
}

pub struct LoadedContext<'a, S: 'a> {
    context: &'a mut Context<S>,
    state: &'a mut S
}

impl <S> Context<S> {
    pub fn new() -> Context<S> {
        Context {
            vm: Vm::new(),
        }
    }

    pub fn load<'a>(&'a mut self, state: &'a mut S) -> LoadedContext<'a, S> {
        LoadedContext::new(self, state)
    }
}

impl <'a, S> LoadedContext<'a, S> {
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
        try!(self.context.vm.load_and_execute(&instrs[..], 0));
        let result = try!(self.context.vm.stack.pop());
        assert!(stack_size == self.context.vm.stack.len());
        Ok(result)
    }
}

#[test]
fn basic_context() {
    let mut state = ();
    let mut ctx = Context::new();
    let mut lctx = ctx.load(&mut state);
    assert_eq!(lctx.eval("(+ 1 2 3)"), Ok(6.into()));
}
