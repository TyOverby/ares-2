use std::any::TypeId;

use gc::{Trace, Gc, GcCell};

use ::host::State;
use ::vm::Value;

pub struct UserFunction<S: State + ?Sized> {
    name: Option<String>,
    f: Box<FnMut(&mut S, Vec<Value>) -> Value + 'static>,
    state_typeid: TypeId,
}

pub fn user_function<S: ?Sized, F>(name: Option<String>, f: F) -> Value where
S: State, F: FnMut(&mut S, Vec<Value>)->Value + 'static {
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

    pub fn call(&mut self, state: &mut S, args: Vec<Value>) -> Value {
        (self.f)(state, args)
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


unsafe impl Trace for UserFunction<()> {
    unsafe_empty_trace!();
}
