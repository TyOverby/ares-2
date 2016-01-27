use std::boxed::Box;

#[derive(Debug, PartialEq)]
pub enum Ast<'input> {
    FloatLit(f64),
    IntLit(i64),
    StringLit(&'input str),
    SymbolLit(&'input str),
    Add(Box<Ast<'input>>, Box<Ast<'input>>),
    Sub(Box<Ast<'input>>, Box<Ast<'input>>),
    Mul(Box<Ast<'input>>, Box<Ast<'input>>),
    Div(Box<Ast<'input>>, Box<Ast<'input>>),
    Identifier(Ident<'input>),
    FnCall(Box<Ast<'input>>, Vec<Ast<'input>>),
    Closure(Option<Ident<'input>>, Vec<Vec<Ident<'input>>>, Vec<Ast<'input>>),
}

#[derive(Debug, PartialEq)]
pub struct Ident<'input>(&'input str);

mod syntax;

#[cfg(test)]
mod test {
    use super::*;
    use super::syntax::parse_Expr;

    #[test]
    fn literals() {
        assert_eq!(parse_Expr("5").unwrap(), Ast::IntLit(5));
        assert_eq!(parse_Expr("5.5").unwrap(), Ast::FloatLit(5.5));
        assert_eq!(parse_Expr("\"hello\"").unwrap(), Ast::StringLit("hello"));
        assert_eq!(parse_Expr("'foo").unwrap(), Ast::SymbolLit("foo"));
    }

    #[test]
    fn operators() {
        assert_eq!(parse_Expr("5 + 6").unwrap(), Ast::Add(
                Box::new(Ast::IntLit(5)),
                Box::new(Ast::IntLit(6))));

        assert_eq!(parse_Expr("5 - 6").unwrap(), Ast::Sub(
                Box::new(Ast::IntLit(5)),
                Box::new(Ast::IntLit(6))));

        assert_eq!(parse_Expr("5 * 6").unwrap(), Ast::Mul(
                Box::new(Ast::IntLit(5)),
                Box::new(Ast::IntLit(6))));

        assert_eq!(parse_Expr("5 / 6").unwrap(), Ast::Div(
                Box::new(Ast::IntLit(5)),
                Box::new(Ast::IntLit(6))));

        assert_eq!(parse_Expr("5 + 6 * 7").unwrap(), Ast::Add(
                Box::new(Ast::IntLit(5)),
                Box::new(Ast::Mul(
                        Box::new(Ast::IntLit(6)),
                        Box::new(Ast::IntLit(7))))));

        assert_eq!(parse_Expr("5 * 6 + 7").unwrap(), Ast::Add(
                Box::new(Ast::Mul(
                        Box::new(Ast::IntLit(5)),
                        Box::new(Ast::IntLit(6)))),
                Box::new(Ast::IntLit(7))));
    }

    #[test]
    fn closures() {
        assert!(parse_Expr("fn foo {}").is_err());
        assert!(parse_Expr("fn").is_err());

        // named
        assert_eq!(parse_Expr("fn foo() { }").unwrap(),
                   Ast::Closure(Some(Ident("foo")),
                                vec![vec![]], vec![]));
        // no name
        assert_eq!(parse_Expr("fn() { }").unwrap(),
                   Ast::Closure(None, vec![vec![]], vec![]));
        // no name with params
        assert_eq!(parse_Expr("fn(a, b) { }").unwrap(),
                   Ast::Closure(None, vec![vec![Ident("a"), Ident("b")]], vec![]));

        // named with prams
        assert_eq!(parse_Expr("fn foo(a, b,) { }").unwrap(),
                   Ast::Closure(Some(Ident("foo")),
                                vec![vec![Ident("a"), Ident("b")]], vec![]));

        // multi-param list
        assert_eq!(parse_Expr("fn foo()()() { }").unwrap(),
                   Ast::Closure(Some(Ident("foo")),
                                vec![vec![], vec![], vec![]], vec![]));
        assert_eq!(parse_Expr("fn foo(a, b)(c, d)(e, f) { }").unwrap(),
                   Ast::Closure(Some(Ident("foo")),
                                vec![vec![Ident("a"), Ident("b")],
                                     vec![Ident("c"), Ident("d")],
                                     vec![Ident("e"), Ident("f")]],
                                vec![]));
    }

    #[test]
    fn func_calling() {
        assert_eq!(parse_Expr("foo(a, b)").unwrap(),
            Ast::FnCall(Box::new(Ast::Identifier(Ident("foo"))),
                        vec![Ast::Identifier(Ident("a")),
                             Ast::Identifier(Ident("b"))]));

        assert_eq!(parse_Expr("foo(a, b)(c, d)").unwrap(),
            Ast::FnCall(Box::new(Ast::FnCall(Box::new(Ast::Identifier(Ident("foo"))),
                                             vec![Ast::Identifier(Ident("a")),
                                                 Ast::Identifier(Ident("b"))])),
                        vec![Ast::Identifier(Ident("c")),
                             Ast::Identifier(Ident("d"))]));

        assert_eq!(parse_Expr("fn foo(a, b) {} (1, 2)").unwrap(),
            Ast::FnCall(Box::new(
                    Ast::Closure(Some(Ident("foo")),
                                 vec![vec![Ident("a"), Ident("b")]], vec![])),
                    vec![Ast::IntLit(1), Ast::IntLit(2)]));

        assert_eq!(parse_Expr("fn(a, b) {} (1, 2)").unwrap(),
            Ast::FnCall(Box::new(
                    Ast::Closure(None, vec![vec![Ident("a"), Ident("b")]], vec![])),
                    vec![Ast::IntLit(1), Ast::IntLit(2)]));
    }
}
