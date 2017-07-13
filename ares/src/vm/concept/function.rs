use std::any::TypeId;

use gc::{Gc, GcCell};

use ::host::{State, EphemeralContext};
use ::vm::Value;

#[derive(Trace, Finalize)]
pub struct UserFunction<S: State + ?Sized> {
    name: Option<String>,
    #[unsafe_ignore_trace]
    f: Box<FnMut(Vec<Value>, &mut S, &mut EphemeralContext<S>) -> Value + 'static>,
    #[unsafe_ignore_trace]
    state_typeid: TypeId,
}

pub fn user_function<S: ?Sized, F>(name: Option<String>, f: F) -> Value where
S: State, F: FnMut(Vec<Value>, &mut S, &mut EphemeralContext<S>)->Value + 'static {
    let user_function = UserFunction {
        name: name,
        f: Box::new(f),
        state_typeid: TypeId::of::<S>()
    };
    Value::UserFn(Gc::new(GcCell::new(user_function.erase())))
}

impl <S: State + ?Sized> UserFunction<S> {
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| &s[..])
    }

    pub fn call(&mut self, state: &mut S, args: Vec<Value>, ctx: &mut EphemeralContext<S>) -> Value {
        (self.f)(args, state, ctx)
    }

    pub fn erase(self) -> UserFunction<()> {
        use std::mem::transmute;
        unsafe { transmute(self) }
    }
}

impl UserFunction<()> {
    pub fn correct<S: State + ?Sized>(&mut self) ->
    Result<&mut UserFunction<S>, &mut UserFunction<()>> {
        use std::mem::transmute;
        if TypeId::of::<S>() == self.state_typeid {
            Ok(unsafe { transmute(self) })
        } else {
            Err(self)
        }
    }
}
