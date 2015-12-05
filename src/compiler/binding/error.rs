use vm::Symbol;
use compiler::parse::Span;

pub enum BindingError {
    CouldNotBind(Symbol, Span)
}
