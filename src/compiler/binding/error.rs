use vm::Symbol;
use compiler::parse::Span;

#[derive(Debug)]
pub enum BindingError {
    CouldNotBind(Symbol, Span)
}
