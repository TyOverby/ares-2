use vm::Symbol;
use compiler::parse::Span;

#[derive(Debug, Eq, PartialEq)]
pub enum BindingError {
    CouldNotBind(Symbol, Span),
    AlreadyDefined(Symbol),
}
