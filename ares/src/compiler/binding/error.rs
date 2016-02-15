use ares_syntax::Symbol;
use compiler::parse::Span;

#[derive(Debug, Eq, PartialEq)]
pub enum BindingError {
    CouldNotBind(Symbol, Span),
    AlreadyDefined(Symbol),
    Multiple(Vec<BindingError>)
}
