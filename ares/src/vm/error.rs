use ::vm::{Value, ValueKind, Instr, Stack};

pub(crate) trait InterpErrorExt {
    type Out;
    fn with_details(self, stack: &Stack, code: &[Instr], code_ptr: usize) -> Self::Out;
}

pub struct InterpErrorDetails {
    stack: Vec<Value>,
    code: Vec<Instr>,
    code_ptr: usize,
}

pub struct InterpError {
    kind: InterpErrorKind,
    details: InterpErrorDetails,
}

#[derive(Debug, Eq, PartialEq)]
pub enum InterpErrorKind {
    InternalInterpError(String),
    MismatchedType {
        value: Value,
        expected: ValueKind,
    },
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

impl InterpErrorKind {
    pub(crate) fn with_details(self, stack: &Stack, code: &[Instr], code_ptr: usize) -> InterpError {
        InterpError {
            kind: self,
            details: InterpErrorDetails {
                stack: stack.as_slice().iter().cloned().collect(),
                code: code.iter().cloned().collect(),
                code_ptr: code_ptr,
            }
        }
    }
}

impl <T> InterpErrorExt for Result<T, InterpErrorKind> {
    type Out = Result<T, InterpError>;
    #[inline(always)]
    fn with_details(self, stack: &Stack, code: &[Instr], code_ptr: usize) -> Self::Out {
        self.map_err(|e| e.with_details(stack, code, code_ptr))
    }
}
