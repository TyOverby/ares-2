pub use ares_syntax::{Span, Ast, AstRef};
use ares_syntax::SymbolIntern;
use ares_syntax::parse_Program;
use typed_arena::Arena;
use lalrpop_util;

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
   InvalidToken {
        location: usize,
    },
    UnrecognizedToken {
        token: Option<(usize, (usize, String), usize)>,
        expected: Vec<String>,
    },
    ExtraToken {
        token: (usize, (usize, String), usize),
    },
}

pub fn parse<'a>(program: &str, interner: &mut SymbolIntern, arena: &'a Arena<Ast<'a>>)
-> Result<Vec<Ast<'a>>, ParseError> {
    match parse_Program(arena, interner, program) {
        Ok(ast) => Ok(ast),
        Err(e) => {
            Err(match e {
                lalrpop_util::ParseError::InvalidToken{location} =>
                    ParseError::InvalidToken{location: location},
                lalrpop_util::ParseError::UnrecognizedToken{token: Some((a, (c, s), b)), expected} =>
                    ParseError::UnrecognizedToken {
                        token: Some((a, (c, s.into()), b)),
                        expected: expected
                    },
                lalrpop_util::ParseError::UnrecognizedToken{token: None, expected} =>
                    ParseError::UnrecognizedToken{token: None, expected: expected},
                lalrpop_util::ParseError::ExtraToken{token: (a, (c, s), b)} =>
                    ParseError::ExtraToken {
                        token: (a, (c, s.into()), b)
                    },
                lalrpop_util::ParseError::User{..} => unreachable!()
            })
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use typed_arena::Arena;
    use ares_syntax::SymbolIntern;
    pub fn ok_parse_1(program: &str) -> (AstRef<'static>, SymbolIntern) {
        let mut interner = SymbolIntern::new();
        let r = ok_parse_1_full(program, &mut interner);
        (r, interner)
    }

    pub fn ok_parse_1_full(program: &str, interner: &mut SymbolIntern) -> AstRef<'static> {
        use std::mem::{transmute, forget};
        let arena: Arena<Ast> = Arena::new();
        let arena_ref: &'static _ = unsafe{ transmute(&arena)};
        let mut asts = parse(program, interner, arena_ref).unwrap();
        assert!(asts.len() == 1);
        let result = asts.pop().unwrap();
        let result = arena_ref.alloc(result);
        forget(arena);
        result
    }
}
