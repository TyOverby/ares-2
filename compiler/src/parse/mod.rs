mod errors;
mod tokens;
mod parse;
mod util;

use ares_vm::{Symbol, SymbolIntern};
use parse::tokens::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum Ast {
    BoolLit(bool, Token),
    StringLit(String, Token),
    IntLit(i64, Token),
    FloatLit(f64, Token),
    ListLit(Vec<Ast>, Token, Token),
    MapLit(Vec<(Ast, Ast)>, Token, Token),
    SymbolLit(Symbol, Token),
    Add(Vec<Ast>, Token, Token),
    Quote(Box<Ast>, Token, Token),
    List(Vec<Ast>, Token, Token),
    If(Box<Ast>, Box<Ast>, Box<Ast>, Token, Token),
}

impl Ast {
    pub fn from_st(s: &str, interner: &mut SymbolIntern) -> Result<Vec<Ast>, errors::ParseError> {
        parse::parse(s, interner)
    }
    pub fn is_symbol_lit_with(&self, symbol: &Symbol) -> bool {
        if let &Ast::SymbolLit(ref s, _) = self {
            s == symbol
        } else { false }
    }
}
