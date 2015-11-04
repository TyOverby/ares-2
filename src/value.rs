use std::rc::Rc;

use intern::Symbol;
use {InterpError, Lambda};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    List(Rc<Vec<Value>>),
    String(Rc<String>),
    Float(f64),
    Int(i64),
    Bool(bool),
    Symbol(Symbol),
    Lambda(Rc<Lambda>),
}

#[derive(Debug)]
pub enum ValueKind {
    List, String, Float, Int, Bool, Symbol, Lambda
}

impl Value {
    pub fn expect_list(self) -> Result<Rc<Vec<Value>>, InterpError> {
        match self {
            Value::List(list) => Ok(list),
            other => Err(InterpError:: MismatchedType {
                value: other,
                expected: ValueKind::List
            })
        }
    }

    pub fn expect_string(self) -> Result<Rc<String>, InterpError> {
        match self {
            Value::String(string) => Ok(string),
            other => Err(InterpError:: MismatchedType {
                value: other,
                expected: ValueKind::String
            })
        }
    }

    pub fn expect_float(self) -> Result<f64, InterpError> {
        match self {
            Value::Float(float) => Ok(float),
            other => Err(InterpError:: MismatchedType {
                value: other,
                expected: ValueKind::Float
            })
        }
    }

    pub fn expect_int(self) -> Result<i64, InterpError> {
        match self {
            Value::Int(int) => Ok(int),
            other => Err(InterpError:: MismatchedType {
                value: other,
                expected: ValueKind::Int
            })
        }
    }

    pub fn expect_bool(self) -> Result<bool, InterpError> {
        match self {
            Value::Bool(b) => Ok(b),
            other => Err(InterpError:: MismatchedType {
                value: other,
                expected: ValueKind::Bool
            })
        }
    }

    pub fn expect_symbol(self) -> Result<Symbol, InterpError> {
        match self {
            Value::Symbol(symbol) => Ok(symbol),
            other => Err(InterpError:: MismatchedType {
                value: other,
                expected: ValueKind::Symbol
            })
        }
    }

    pub fn expect_lambda(self) -> Result<Rc<Lambda>, InterpError> {
        match self {
            Value::Lambda(lambda) => Ok(lambda),
            other => Err(InterpError:: MismatchedType {
                value: other,
                expected: ValueKind::Lambda
            })
        }
    }
}
