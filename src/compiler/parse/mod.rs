mod errors;
mod tokens;
mod parse;
mod util;

use vm::{Symbol, SymbolIntern};
use compiler::parse::tokens::Position;

pub use self::errors::ParseError;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Span {
    start: Position,
    end: Position,
}

impl Span {
    pub fn dummy() -> Span {
        Span {
            start: Position(0, 0),
            end: Position(0, 0),
        }
    }
    fn from_pos(p1: Position, p2: Position) -> Span {
        if p1 < p2 {
            Span {
                start: p1,
                end: p2,
            }
        } else {
            Span {
                start: p2,
                end: p1,
            }
        }
    }

    fn join(self, other: Span) -> Span {
        use std::cmp::{min, max};
        let lowest = min(self.start, other.start);
        let highest = max(self.end, other.end);
        Span {
            start: lowest,
            end: highest
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Ast {
    BoolLit(bool, Span),
    StringLit(String, Span),
    IntLit(i64, Span),
    FloatLit(f64, Span),
    ListLit(Vec<Ast>, Span),
    MapLit(Vec<(Ast, Ast)>, Span),
    SymbolLit(Symbol, Span),
    Add(Vec<Ast>, Span),
    Quote(Box<Ast>, Span),
    List(Vec<Ast>, Span),
    If(Box<Ast>, Box<Ast>, Box<Ast>, Span),
}

pub fn parse(s: &str, interner: &mut SymbolIntern) -> Result<Vec<Ast>, errors::ParseError> {
    parse::parse(s, interner)
}

impl Ast {
    pub fn is_symbol_lit_with(&self, symbol: &Symbol) -> bool {
        if let &Ast::SymbolLit(ref s, _) = self {
            s == symbol
        } else { false }
    }
}
