use compiler::parse::ParseError;
use compiler::emit::EmitError;
use compiler::binding::BindingError;

#[derive(Debug, Eq, PartialEq)]
pub enum CompileError {
    ParseError(ParseError),
    EmitError(EmitError),
    BindingError(BindingError),
}

impl From<ParseError> for CompileError {
    fn from(pe: ParseError) -> CompileError {
        CompileError::ParseError(pe)
    }
}

impl From<EmitError> for CompileError {
    fn from(ee: EmitError) -> CompileError {
        CompileError::EmitError(ee)
    }
}

impl From<BindingError> for CompileError {
    fn from(ee: BindingError) -> CompileError {
        CompileError::BindingError(ee)
    }
}
