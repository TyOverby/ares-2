use parse::ParseError;
use emit::EmitError;

#[derive(Debug)]
pub enum CompileError {
    ParseError(ParseError),
    EmitError(EmitError),
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
