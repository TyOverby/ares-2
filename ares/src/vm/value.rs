use gc::{Gc, Trace, GcCell};
use std::ops::Deref;
use std::collections::HashMap;
use ares_syntax::{Symbol, SymbolIntern};
use vm::{InterpError, Closure, Continuation};
use vm::concept::function::UserFunction;

macro_rules! gen_expect {
    ($self_fn: ident, $ref_fn: ident, $mut_fn: ident, $selector: path, $out: ty, $expected: expr) => {
        pub fn $self_fn(self) -> Result<$out, InterpError> {
            if let $selector(v) = self {
                Ok(v)
            } else {
                Err(InterpError::MismatchedType {
                    value: self,
                    expected: $expected,
                })
            }
        }

        pub fn $ref_fn(&self) -> Result<&$out, InterpError> {
            if let &$selector(ref v) = self {
                Ok(v)
            } else {
                Err(InterpError::MismatchedType {
                    value: self.clone(),
                    expected: $expected,
                })
            }
        }
        pub fn $mut_fn(&mut self) -> Result<&mut $out, InterpError> {
            if let &mut $selector(ref mut v) = self {
                Ok(v)
            } else {
                Err(InterpError::MismatchedType {
                    value: self.clone(),
                    expected: $expected,
                })
            }
        }
    };
}

macro_rules! gen_from {
    ($inx: ty, $out: path) => {
        gen_from!($inx, $out, |i| i);
    };
    ($inx: ty, $out: path, $tr: expr) => {
        impl From<$inx> for Value {
            fn from(i: $inx) -> Value {
                $out($tr(i))
            }
        }
    }
}


#[derive(Clone)]
pub enum Value {
    Nil,
    List(Gc<Vec<Value>>),
    Map(Gc<MapWrapper>),
    String(Gc<String>),
    Float(f64),
    Int(i64),
    Bool(bool),
    Symbol(Symbol),
    Closure(Gc<Closure>),
    UserFn(Gc<GcCell<UserFunction<()>>>),
    Cell(Gc<GcCell<Value>>),
    Continuation(Gc<Continuation>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum ValueKind {
    Nil,
    List,
    Map,
    String,
    Float,
    Int,
    Bool,
    Symbol,
    Closure,
    UserFn,
    Cell,
    Continuation,
}

#[derive(Debug, PartialEq)]
pub struct MapWrapper(HashMap<Value, Value>);

impl Deref for MapWrapper {
    type Target = HashMap<Value, Value>;
    fn deref(&self) -> &HashMap<Value, Value> {
        &self.0
    }
}

unsafe impl Trace for Value {
    custom_trace!(this, {
        match this {
            &Value::List(ref gc) => mark(gc),
            &Value::Map(ref gc) => mark(gc),
            &Value::String(ref gc) => mark(gc),
            &Value::Closure(ref gc) => mark(gc),
            _ => {}
        }
    });
}

unsafe impl Trace for MapWrapper {
    custom_trace!(this, {
        for (k, v) in &this.0 {
            mark(k);
            mark(v);
        }
    });
}

impl ::std::fmt::Debug for Value {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        let empty_interner = SymbolIntern::new();
        let s = to_string_helper(self, &empty_interner);
        formatter.write_str(&s[..])
    }
}

impl Eq for Value {}
impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        use vm::Value::*;

        match (self, other) {
            (&List(ref gc1), &List(ref gc2)) => {
                gc_to_usize(gc1) == gc_to_usize(gc2) || &**gc1 == &**gc2
            }
            (&Map(ref gc1), &Map(ref gc2)) => {
                gc_to_usize(gc1) == gc_to_usize(gc2) || &**gc1 == &**gc2
            }
            (&String(ref gc1), &String(ref gc2)) => &**gc1 == &**gc2,
            (&Float(f1), &Float(f2)) => f1 == f2,
            (&Int(i1), &Int(i2)) => i1 == i2,
            (&Bool(b1), &Bool(b2)) => b1 == b2,
            (&Symbol(ref id1), &Symbol(ref id2)) => id1 == id2,
            // (&Closure(ref l1, b1), &Closure(ref l2, b2)) => l1 == l2 && b1 == b2,
            (&Cell(ref c1), &Cell(ref c2)) => &*c1.borrow() == &*c2.borrow(),
            _ => false,
        }
    }
}

impl Value {
    pub fn decell(self) -> Value {
        match self {
            Value::Cell(i) => i.borrow().clone(),
            other => other
        }
    }

    pub fn cellify(self) -> Value {
        match self {
            c@Value::Cell(_) => c,
            other => Value::Cell(Gc::new(GcCell::new(other))),
        }
    }

    /*
    Cell(Gc<GcCell<Value>>),
    Continuation(Gc<Continuation>),
     */
    gen_expect!(expect_string, expect_string_ref, expect_string_mut, Value::String, Gc<String>, ValueKind::String);
    gen_expect!(expect_list, expect_list_ref, expect_list_mut, Value::List, Gc<Vec<Value>>, ValueKind::List);
    gen_expect!(expect_map, expect_map_ref, expect_map_mut, Value::Map, Gc<MapWrapper>, ValueKind::Map);
    gen_expect!(expect_float, expect_float_ref, expect_float_mut, Value::Float, f64, ValueKind::Float);
    gen_expect!(expect_int, expect_int_ref, expect_int_mut, Value::Int, i64, ValueKind::Int);
    gen_expect!(expect_bool, expect_bool_ref, expect_bool_mut, Value::Bool, bool, ValueKind::Bool);
    gen_expect!(expect_symbol, expect_symbol_ref, expect_symbol_mut, Value::Symbol, Symbol, ValueKind::Symbol);
    gen_expect!(expect_closure, expect_closure_ref, expect_closure_mut, Value::Closure, Gc<Closure>, ValueKind::Closure);
    gen_expect!(expect_user_fn, expect_user_fn_ref, expect_user_fn_mut, Value::UserFn, Gc<GcCell<UserFunction<()>>>, ValueKind::UserFn);
    gen_expect!(expect_cell, expect_cell_ref, expect_cell_mut, Value::Cell, Gc<GcCell<Value>>, ValueKind::Cell);
    gen_expect!(expect_continuation, expect_continuation_ref, expect_continuation_mut, Value::Continuation, Gc<Continuation>, ValueKind::Continuation);

    pub fn expect_nil(self) -> Result<(), InterpError> {
        if let Value::Nil = self { Ok(()) }
        else {
            Err(InterpError::MismatchedType {
                value: self,
                expected: ValueKind::Nil,
            })
        }
    }

    pub fn expect_nil_ref(&self) -> Result<(), InterpError> {
        if let &Value::Nil = self { Ok(()) }
        else {
            Err(InterpError::MismatchedType {
                value: self.clone(),
                expected: ValueKind::Nil,
            })
        }
    }

    pub fn expect_nil_mut(&mut self) -> Result<(), InterpError> {
        if let &mut Value::Nil = self { Ok(()) }
        else {
            Err(InterpError::MismatchedType {
                value: self.clone(),
                expected: ValueKind::Nil,
            })
        }
    }
}

fn gc_to_usize<T: Trace>(gc: &Gc<T>) -> usize {
    use std::mem::transmute;
    let ptr_t: &T = &**gc;
    unsafe { transmute(ptr_t) }
}

pub fn to_string_helper(value: &Value, interner: &SymbolIntern) -> String {
    use std::collections::HashSet;
    match value {
        &Value::Nil => "nil".to_string(),
        &Value::Int(i) => format!("{}", i),
        &Value::Float(f) => format!("{}", f),
        &Value::String(ref s) => (&**s).clone(),
        &Value::Bool(b) => format!("{}", b),
        &Value::Symbol(s) => format!("'{}", interner.lookup_or_anon(s)),
        &Value::Closure(ref c) => format!("<Closure {}>", c.class.code_offset),
        &Value::Continuation(ref c) => format!("<Continuation {:?}>", c),
        &Value::UserFn(ref f) => {
            let f = f.borrow();
            let name = f.name().unwrap_or("{anon}");
            format!("<UserFn {}>", name)
        }
        &Value::Cell(ref t) => format!("c {}", to_string_helper(&*t.borrow(), interner)),

        &ref l@Value::List(_) | &ref l@Value::Map(_) => {
            fn format_singles(vec: &Gc<Vec<Value>>,
                              buf: &mut String,
                              seen: &mut HashSet<usize>,
                              interner: &SymbolIntern) {
                let ptr = gc_to_usize(vec);
                if seen.contains(&ptr) {
                    buf.push_str("[ ... ]")
                } else {
                    seen.insert(ptr);
                    buf.push_str("[");
                    for v in vec.iter() {
                        build_buf(v, buf, seen, interner);
                        buf.push_str(", ");
                    }
                    // remove trailing comma and space
                    if vec.len() >= 1 {
                        buf.pop();
                        buf.pop();
                    }
                    buf.push_str("]");
                    seen.remove(&ptr);
                }
            }
            fn format_pairs(m: &Gc<MapWrapper>,
                            buf: &mut String,
                            seen: &mut HashSet<usize>,
                            interner: &SymbolIntern) {
                let ptr = gc_to_usize(m);
                if seen.contains(&ptr) {
                    buf.push_str("{ ... }")
                } else {
                    seen.insert(ptr);
                    buf.push_str("{");
                    for (k, v) in m.iter() {
                        build_buf(k, buf, seen, interner);
                        buf.push_str(", ");
                        build_buf(v, buf, seen, interner);
                    }
                    buf.push_str("}");
                    seen.remove(&ptr);
                }
            }
            fn build_buf(cur: &Value,
                         buf: &mut String,
                         seen: &mut HashSet<usize>,
                         interner: &SymbolIntern) {
                match cur {
                    &Value::List(ref v) => format_singles(v, buf, seen, interner),
                    &Value::Map(ref m) => format_pairs(m, buf, seen, interner),
                    other => buf.push_str(&to_string_helper(&other, interner)),
                }
            }
            let mut inner = String::new();
            let mut seen = HashSet::new();
            build_buf(&l, &mut inner, &mut seen, interner);
            inner
        }
    }
}

impl ::std::hash::Hash for Value {
    fn hash<H>(&self, state: &mut H)
        where H: ::std::hash::Hasher
    {
        use std::mem::transmute;
        match self {
            &Value::Nil => state.write_u8(0),
            &Value::List(ref rc) => rc.hash(state),
            &Value::Map(ref rc) => {
                for (k, v) in rc.iter() {
                    k.hash(state);
                    v.hash(state);
                }
            }
            &Value::String(ref rc) => rc.hash(state),
            &Value::Float(f) => unsafe { state.write(&transmute::<_, [u8; 8]>(f)) },
            &Value::Int(i) => state.write_i64(i),
            &Value::Bool(b) => {
                let byte = if b {
                    1
                } else {
                    0
                };
                state.write_u8(byte);
            }
            &Value::Symbol(ref rc) => rc.hash(state),
            &Value::Cell(ref t) => t.borrow().hash(state),
            &Value::Continuation(ref c) => {
                c.instruction_pos.hash(state);
                c.saved_stack.len().hash(state);
                c.saved_stack_frames.len().hash(state);
            }
            &Value::Closure(ref c) => state.write_usize(unsafe {transmute(&*c)}),
            &Value::UserFn(ref f) => {
                state.write_usize(unsafe {transmute(&*f.borrow())})
            }
        }
    }
}


gen_from!(Symbol, Value::Symbol, |a| a);
gen_from!(u8, Value::Int, |a| a as i64);
gen_from!(i8, Value::Int, |a| a as i64);
gen_from!(u16, Value::Int, |a| a as i64);
gen_from!(i16, Value::Int, |a| a as i64);
gen_from!(u32, Value::Int, |a| a as i64);
gen_from!(i32, Value::Int, |a| a as i64);
gen_from!(u64, Value::Int, |a| a as i64);
gen_from!(i64, Value::Int);

gen_from!(f32, Value::Float, |a| a as f64);
gen_from!(f64, Value::Float);

gen_from!(bool, Value::Bool);

gen_from!(String, Value::String, Gc::new);
gen_from!(Closure, Value::Closure, |a| Gc::new(a));

impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(x: Vec<T>) -> Value {
        Value::List(Gc::new(x.into_iter().map(|a| a.into()).collect()))
    }
}

impl<T: Into<Value> + ::std::hash::Hash + Eq> From<HashMap<T, T>> for Value {
    fn from(x: HashMap<T, T>) -> Value {
        Value::Map(Gc::new(MapWrapper(x.into_iter().map(|(k, v)| (k.into(), v.into())).collect())))
    }
}

impl<'a> From<&'a str> for Value {
    fn from(x: &'a str) -> Value {
        let s: String = x.into();
        let v: Value = s.into();
        v
    }
}
