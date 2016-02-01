use compiler::CompileError;
use vm::InterpError;

pub type AresResult<A> = Result<A, AresError>;

#[derive(Eq, PartialEq, Debug)]
pub enum AresError {
    CompileError(CompileError),
    InterpError(InterpError),
}

impl From<CompileError> for AresError {
    fn from(ce: CompileError) -> AresError {
        AresError::CompileError(ce)
    }
}

impl From<InterpError> for AresError {
    fn from(ie: InterpError) -> AresError {
        AresError::InterpError(ie)
    }
}
