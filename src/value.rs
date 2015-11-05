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
    List,
    String,
    Float,
    Int,
    Bool,
    Symbol,
    Lambda,
}

impl Value {
    pub fn expect_list(self) -> Result<Rc<Vec<Value>>, InterpError> {
        match self {
            Value::List(list) => Ok(list),
            other => Err(InterpError::MismatchedType {
                value: other,
                expected: ValueKind::List,
            }),
        }
    }

    pub fn expect_string(self) -> Result<Rc<String>, InterpError> {
        match self {
            Value::String(string) => Ok(string),
            other => Err(InterpError::MismatchedType {
                value: other,
                expected: ValueKind::String,
            }),
        }
    }

    pub fn expect_float(self) -> Result<f64, InterpError> {
        match self {
            Value::Float(float) => Ok(float),
            other => Err(InterpError::MismatchedType {
                value: other,
                expected: ValueKind::Float,
            }),
        }
    }

    pub fn expect_int(self) -> Result<i64, InterpError> {
        match self {
            Value::Int(int) => Ok(int),
            other => Err(InterpError::MismatchedType {
                value: other,
                expected: ValueKind::Int,
            }),
        }
    }

    pub fn expect_bool(self) -> Result<bool, InterpError> {
        match self {
            Value::Bool(b) => Ok(b),
            other => Err(InterpError::MismatchedType {
                value: other,
                expected: ValueKind::Bool,
            }),
        }
    }

    pub fn expect_symbol(self) -> Result<Symbol, InterpError> {
        match self {
            Value::Symbol(symbol) => Ok(symbol),
            other => Err(InterpError::MismatchedType {
                value: other,
                expected: ValueKind::Symbol,
            }),
        }
    }

    pub fn expect_lambda(self) -> Result<Rc<Lambda>, InterpError> {
        match self {
            Value::Lambda(lambda) => Ok(lambda),
            other => Err(InterpError::MismatchedType {
                value: other,
                expected: ValueKind::Lambda,
            }),
        }
    }

    pub fn expect_list_ref(&self) -> Result<&Rc<Vec<Value>>, InterpError> {
        match self {
            &Value::List(ref list) => Ok(list),
            other => Err(InterpError::MismatchedType {
                value: other.clone(),
                expected: ValueKind::List,
            }),
        }
    }

    pub fn expect_string_ref(&self) -> Result<&Rc<String>, InterpError> {
        match self {
            &Value::String(ref string) => Ok(string),
            other => Err(InterpError::MismatchedType {
                value: other.clone(),
                expected: ValueKind::String,
            }),
        }
    }

    pub fn expect_float_ref(&self) -> Result<&f64, InterpError> {
        match self {
            &Value::Float(ref float) => Ok(float),
            other => Err(InterpError::MismatchedType {
                value: other.clone(),
                expected: ValueKind::Float,
            }),
        }
    }

    pub fn expect_int_ref(&self) -> Result<&i64, InterpError> {
        match self {
            &Value::Int(ref int) => Ok(int),
            other => Err(InterpError::MismatchedType {
                value: other.clone(),
                expected: ValueKind::Int,
            }),
        }
    }

    pub fn expect_bool_ref(&self) -> Result<&bool, InterpError> {
        match self {
            &Value::Bool(ref b) => Ok(b),
            other => Err(InterpError::MismatchedType {
                value: other.clone(),
                expected: ValueKind::Bool,
            }),
        }
    }

    pub fn expect_symbol_ref(&self) -> Result<&Symbol, InterpError> {
        match self {
            &Value::Symbol(ref symbol) => Ok(symbol),
            other => Err(InterpError::MismatchedType {
                value: other.clone(),
                expected: ValueKind::Symbol,
            }),
        }
    }

    pub fn expect_lambda_ref(&self) -> Result<&Rc<Lambda>, InterpError> {
        match self {
            &Value::Lambda(ref lambda) => Ok(lambda),
            other => Err(InterpError::MismatchedType {
                value: other.clone(),
                expected: ValueKind::Lambda,
            }),
        }
    }

    pub fn expect_list_ref_mut(&mut self) -> Result<&mut Rc<Vec<Value>>, InterpError> {
        match self {
            &mut Value::List(ref mut list) => Ok(list),
            other => Err(InterpError::MismatchedType {
                value: other.clone(),
                expected: ValueKind::List,
            }),
        }
    }

    pub fn expect_string_ref_mut(&mut self) -> Result<&mut Rc<String>, InterpError> {
        match self {
            &mut Value::String(ref mut string) => Ok(string),
            other => Err(InterpError::MismatchedType {
                value: other.clone(),
                expected: ValueKind::String,
            }),
        }
    }

    pub fn expect_float_ref_mut(&mut self) -> Result<&mut f64, InterpError> {
        match self {
            &mut Value::Float(ref mut float) => Ok(float),
            other => Err(InterpError::MismatchedType {
                value: other.clone(),
                expected: ValueKind::Float,
            }),
        }
    }

    pub fn expect_int_ref_mut(&mut self) -> Result<&mut i64, InterpError> {
        match self {
            &mut Value::Int(ref mut int) => Ok(int),
            other => Err(InterpError::MismatchedType {
                value: other.clone(),
                expected: ValueKind::Int,
            }),
        }
    }

    pub fn expect_bool_ref_mut(&mut self) -> Result<&mut bool, InterpError> {
        match self {
            &mut Value::Bool(ref mut b) => Ok(b),
            other => Err(InterpError::MismatchedType {
                value: other.clone(),
                expected: ValueKind::Bool,
            }),
        }
    }

    pub fn expect_symbol_ref_mut(&mut self) -> Result<&mut Symbol, InterpError> {
        match self {
            &mut Value::Symbol(ref mut symbol) => Ok(symbol),
            other => Err(InterpError::MismatchedType {
                value: other.clone(),
                expected: ValueKind::Symbol,
            }),
        }
    }

    pub fn expect_lambda_ref_mut(&mut self) -> Result<&mut Rc<Lambda>, InterpError> {
        match self {
            &mut Value::Lambda(ref mut lambda) => Ok(lambda),
            other => Err(InterpError::MismatchedType {
                value: other.clone(),
                expected: ValueKind::Lambda,
            }),
        }
    }
}
