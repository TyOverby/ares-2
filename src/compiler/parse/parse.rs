use std::error::Error;
use typed_arena::Arena;
use vm::SymbolIntern;

use compiler::parse::Ast;
use compiler::parse::tokens::{TokenType, Token, Open, TokenIter};
pub use compiler::parse::errors::ParseError;
use compiler::parse::errors::ParseError::*;

fn one_expr<'a, 'b, 'ast>(tok: Token,
                    tok_stream: &'a mut TokenIter<'b>,
                    interner: &mut SymbolIntern,
                    arena: &'ast Arena<Ast<'ast>>)
                    -> Result<&'ast Ast<'ast>, ParseError> {
    use compiler::parse::tokens::TokenType;
    match tok.clone().tt {
        TokenType::Number(s) => {
            let as_int: Result<i64, _> = s.parse();
            if let Ok(int) = as_int {
                Ok(arena.alloc(Ast::IntLit(int, tok.span)))
            } else {
                let as_float: Result<f64, _> = s.parse();
                let as_float = as_float.map_err(|e| ConversionError(s, Box::new(e) as Box<Error>));
                as_float.map(|flt| arena.alloc(Ast::FloatLit(flt, tok.span)) as & _)
            }
        }
        TokenType::Symbol(s) => {
            match &s[..] {
                "true" => Ok(arena.alloc(Ast::BoolLit(true, tok.span))),
                "false" => Ok(arena.alloc(Ast::BoolLit(false, tok.span))),
                other => Ok(arena.alloc(Ast::Symbol(interner.intern(other), tok.span)))
            }
        }

        TokenType::String(s) => Ok(arena.alloc(Ast::StringLit(s, tok.span))),
        // TODO: understand this
        TokenType::FormLike(_fl) => {
            unimplemented!();
            /* Ok({
            let quoted = try!(parse_one_expr(tok_stream, interner));
            let interned = Ast::Symbol(interner.intern(fl.form_name()));
            Ast::list(match quoted {
                None => vec![interned],
                Some(v) => vec![interned, v],
            })
            }), */
        }
        TokenType::Close(close) => Err(ExtraRightDelimiter(close, tok.span)),
        TokenType::Open(open) => {
            let (mut values, end_tok) = try!(parse_delimited(tok_stream, open, interner, arena));
            match open {
                Open::LParen => {
                    if values.len() == 0 {
                        return Ok(arena.alloc(Ast::List(values, tok.span.join(end_tok.span))))
                    }
                    if values[0].is_symbol_lit_with(&interner.precomputed.iff) {
                        let len = values.len();
                        let mut values = values.into_iter();

                        if len != 4 { return Err(UnexpectedIfArity(len, tok.span)); }
                        let _ = values.next();
                        let (cond, tru, fals) =
                            (values.next().unwrap(),
                             values.next().unwrap(),
                             values.next().unwrap());
                        Ok(arena.alloc(Ast::If( cond, tru, fals,
                            tok.span.join(end_tok.span))))
                    } else if values[0].is_symbol_lit_with(&interner.precomputed.plus) {
                        values.remove(0);
                        Ok(arena.alloc(Ast::Add(values, tok.span.join(end_tok.span))))
                    } else if values[0].is_symbol_lit_with(&interner.precomputed.lambda) {
                        // TODO: take varargs into account
                        if values.len() < 2 { return Err(UnexpectedLambdaArity(values.len(), tok.span)); }
                        values.remove(0); // remove the "lambda"
                        let args_list = values.remove(0);
                        let bodies = values;
                        if let &Ast::List(ref args, t) = args_list {
                            let mut arg_list = vec![];
                            for arg in args {
                                if let &&Ast::Symbol(symbol, _) = arg {
                                    arg_list.push(symbol);
                                } else {
                                    return Err(BadLambdaArgs(t));
                                }
                            }
                            Ok(arena.alloc(Ast::Lambda(arg_list, bodies, tok.span.join(end_tok.span))))
                        } else {
                            return Err(BadLambdaArgs(tok.span.join(end_tok.span)))
                        }
                    } else {
                        Ok(arena.alloc(Ast::List(values, tok.span.join(end_tok.span))))
                    }
                },
                Open::LBracket => {
                    unimplemented!()
                    // TODO: add list literals back.
                    /*
                    if values.iter().all(|a| util::immediate_value(a, interner)) {
                        let values = values.into_iter().map(util::unquote).collect();
                        Ok(Ast::list(vec![Ast::Symbol(interner.intern("quote")),
                                            Ast::list(values)]))
                    } else {
                        values.insert(0, Ast::Symbol(interner.intern("list")));
                        Ok(Ast::list(values))
                    }*/
                }
                Open::LBrace => {
                    unimplemented!()
                    // TODO: add map literals back
                    /*
                    if values.len() % 2 == 1 {
                        return Err(InvalidMapLiteral(tok.start));
                    }
                    if values.iter().all(|a| util::immediate_value(a, interner)) {
                        let (keys, values): (Vec<_>, _) = values.into_iter()
                                                                .enumerate()
                                                                .partition(|&(i, _)| i % 2 == 0);
                        if keys.iter().all(|&(_, ref k)| util::can_be_hash_key(k, interner)) {
                            let m = keys.into_iter()
                                        .map(|(_, k)| util::unquote(k))
                                        .zip(values.into_iter().map(|(_, v)| util::unquote(v)))
                                        .collect();
                            Ok(Ast::Map(Rc::new(m)))
                        } else {
                            Err(InvalidMapLiteral(tok.start))
                        }
                    } else {
                        values.insert(0, Ast::Symbol(interner.intern("hash-map")));
                        Ok(Ast::list(values))
                    }
                    */
                }
            }
        }
    }
}

fn parse_one_expr<'a, 'b, 'ast>(tok_stream: &'a mut TokenIter<'b>,
                          interner: &mut SymbolIntern,
                          arena: &'ast Arena<Ast<'ast>>)
                          -> Result<Option<&'ast Ast<'ast>>, ParseError> {
    if let Some(tok) = tok_stream.next() {
        one_expr(try!(tok), tok_stream, interner, arena).map(Some)
    } else {
        Ok(None)
    }
}

fn parse_delimited<'a, 'b, 'ast>(tok_stream: &'a mut TokenIter<'b>,
                           opener: Open,
                           interner: &mut SymbolIntern,
                          arena: &'ast Arena<Ast<'ast>>)
                           -> Result<(Vec<&'ast Ast<'ast>>, Token), ParseError> {
    let mut v = vec![];
    loop {
        if let Some(tok_or_err) = tok_stream.next() {
            let tok = try!(tok_or_err);
            match tok.tt {
                TokenType::Close(close) => if close == opener.closed_by() {
                    return Ok((v, tok));
                } else {
                    return Err(ExtraRightDelimiter(opener.closed_by(), tok.span));
                },
                _ => v.push(try!(one_expr(tok, tok_stream, interner, arena))),
            }
        } else {
            return Err(MissingRightDelimiter(opener.closed_by()));
        }
    }
}

pub fn parse<'ast>(input: &str, interner: &mut SymbolIntern, arena: &'ast Arena<Ast<'ast>>) -> Result<Vec<&'ast Ast<'ast>>, ParseError> {
    let mut v = vec![];
    let mut tok_iter = TokenIter::new(input);
    while let Some(value) = try!(parse_one_expr(&mut tok_iter, interner, arena)) {
        v.push(value)
    }
    Ok(v)
}


#[cfg(test)]
pub mod test {
    use compiler::parse::{Ast, Span};
    use typed_arena::Arena;
    use vm::SymbolIntern;
    use super::parse;

    pub fn ok_parse<'ast>(s: &str, arena: &'ast Arena<Ast<'ast>>) -> (Vec<&'ast Ast<'ast>>, SymbolIntern) {
        let mut interner = SymbolIntern::new();
        (parse(s, &mut interner, arena).unwrap(), interner)
    }

    pub fn ok_parse_1<'ast>(s: &str, arena: &'ast Arena<Ast<'ast>>) -> (&'ast Ast<'ast>, SymbolIntern) {
        let (mut parsed, interner) = ok_parse(s, arena);
        assert!(parsed.len() == 1);
        (parsed.pop().unwrap(), interner)
    }

    #[test]
    fn test_parse_literals() {
        let arena = Arena::new();
        assert!(matches!(ok_parse_1("1", &arena).0, &Ast::IntLit(1, _)));
        assert!(matches!(ok_parse_1("10.0", &arena).0, &Ast::FloatLit(10.0, _)));
        assert!(matches!(ok_parse_1("true", &arena).0, &Ast::BoolLit(true, _)));
        assert!(matches!(ok_parse_1("false", &arena).0, &Ast::BoolLit(false, _)));
    }

    #[test]
    fn test_parse_if() {
        let arena = Arena::new();
        assert!(matches!(ok_parse_1("(if true 1 2)", &arena).0,
                         &Ast::If(
                             &Ast::BoolLit(true, _),
                             &Ast::IntLit(1, _),
                             &Ast::IntLit(2, _),
                             _)));
        assert!(matches!(ok_parse_1("(if true (if false 1 3) 2)", &arena).0,
                         &Ast::If(
                             &Ast::BoolLit(true, _),
                             &Ast::If(
                                 &Ast::BoolLit(false, _),
                                 &Ast::IntLit(1, _),
                                 &Ast::IntLit(3, _),
                                 _),
                             &Ast::IntLit(2, _),
                             _)));
    }

    #[test]
    fn test_parse_plus() {
        let arena = Arena::new();

        let ast = ok_parse_1("(+ 0 1 2)", &arena).0;
        let should = arena.alloc(Ast::Add(vec![
            arena.alloc(Ast::IntLit(0, Span::dummy())),
            arena.alloc(Ast::IntLit(1, Span::dummy())),
            arena.alloc(Ast::IntLit(2, Span::dummy()))], Span::dummy()));
        assert!(ast.equals_sans_span(&should));

        let ast = ok_parse_1("(+)", &arena).0;
        let should = arena.alloc(Ast::Add(vec![], Span::dummy()));
        assert!(ast.equals_sans_span(&should));
    }

    #[test]
    fn test_parse_lambda_no_args() {
        let arena = Arena::new();
        let (ast, _) = ok_parse_1("(lambda () 5)", &arena);
        let should = arena.alloc(Ast::Lambda(
            vec![],
            vec![arena.alloc(Ast::IntLit(5, Span::dummy()))],
            Span::dummy()));
        assert!(ast.equals_sans_span(should), "\n{:?}\n!=\n{:?}", ast, should);
    }

    #[test]
    fn test_parse_lambda_with_args() {
        let arena = Arena::new();
        let (ast, mut interner) = ok_parse_1("(lambda (a b c) (+ a b c))", &arena);
        let a = interner.intern("a");
        let b = interner.intern("b");
        let c = interner.intern("c");

        let should = arena.alloc(Ast::Lambda(
            vec![a, b, c],
            vec![arena.alloc(Ast::Add(vec![
                          arena.alloc(Ast::Symbol(a, Span::dummy())),
                          arena.alloc(Ast::Symbol(b, Span::dummy())),
                          arena.alloc(Ast::Symbol(c, Span::dummy())),
                          ], Span::dummy()))],
            Span::dummy()));
        assert!(ast.equals_sans_span(should));
    }
}
