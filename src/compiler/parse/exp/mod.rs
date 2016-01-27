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
    Identifier(Ident<'input>)
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
}
