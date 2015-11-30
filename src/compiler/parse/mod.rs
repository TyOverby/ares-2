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
    Lambda(Vec<Symbol>, Vec<Ast>, Span)
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

    pub fn equals_sans_span(&self, other: &Ast) -> bool {
        use self::Ast::*;
        match (self, other) {
            (&BoolLit(ref a, _), &BoolLit(ref b, _)) => a == b,
            (&StringLit(ref a, _), &StringLit(ref b, _)) => a == b,
            (&IntLit(ref a, _), &IntLit(ref b, _)) => a == b,
            (&FloatLit(ref a, _), &FloatLit(ref b, _)) => a == b,
            (&ListLit(ref a, _), &ListLit(ref b, _)) => {
                if a.len() != b.len() { return false }
                a.iter().zip(b.iter()).all(|(ref a, ref b)| a.equals_sans_span(b))
            },
            (&MapLit(ref a, _), &MapLit(ref b, _)) => {
                if a.len() != b.len() { return false }
                a.iter().zip(b.iter()).all(|(ref a, ref b)|
                    a.0.equals_sans_span(&b.0) && a.1.equals_sans_span(&b.1))
            },
            (&SymbolLit(a, _), &SymbolLit(b, _)) => a == b,
            (&Add(ref a, _), &Add(ref b, _)) => {
                if a.len() != b.len() { return false }
                a.iter().zip(b.iter()).all(|(ref a, ref b)| a.equals_sans_span(b))
            },
            (&Quote(ref a, _), &Quote(ref b, _)) => a.equals_sans_span(&*b),
            (&List(ref a, _), &List(ref b, _)) => {
                if a.len() != b.len() { return false }
                a.iter().zip(b.iter()).all(|(ref a, ref b)| a.equals_sans_span(b))
            },
            (&If(ref ac, ref at, ref af, _), &If(ref bc, ref bt, ref bf, _)) =>
                ac.equals_sans_span(&*bc) &&
                at.equals_sans_span(&*bt) &&
                af.equals_sans_span(&*bf),
            (&Lambda(ref a_args, ref a_bodies, _), &Lambda(ref b_args, ref b_bodies, _)) => {
                if a_args.len() != b_args.len() { return false }
                if !a_args.iter().zip(b_args.iter()).all(|(ref a, ref b)| a == b) {
                    return false
                }

                if a_bodies.len() != b_bodies.len() { return false }
                a_bodies.iter().zip(b_bodies.iter()).all(|(ref a, ref b)| a.equals_sans_span(b))
            }
            _ => false
        }
    }
}
