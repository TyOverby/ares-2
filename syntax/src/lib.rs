extern crate typed_arena;

mod intern;
mod syntax;

pub use intern::*;
pub use syntax::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Span(u32, u32);

impl Span {
    pub fn dummy() -> Span {
        Span(0, 0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Ast<'ast> {
    BoolLit(bool, Span),
    StringLit(String, Span),
    IntLit(i64, Span),
    FloatLit(f64, Span),
    SymbolLit(Symbol, Span),
    Identifier(Symbol, Span),
//  ListLit(Vec<&'ast Ast<'ast>>, Span),
//  MapLit(Vec<(&'ast Ast<'ast>, &'ast Ast<'ast>)>, Span),
    Add(&'ast Ast<'ast>, &'ast Ast<'ast>, Span),
    Sub(&'ast Ast<'ast>, &'ast Ast<'ast>, Span),
    Mul(&'ast Ast<'ast>, &'ast Ast<'ast>, Span),
    Div(&'ast Ast<'ast>, &'ast Ast<'ast>, Span),
    FnCall(&'ast Ast<'ast>, Vec<Ast<'ast>>, Span),
//  Quote(&'ast Ast<'ast>, Span),
    IfExpression(&'ast Ast<'ast>, &'ast Ast<'ast>, &'ast Ast<'ast>, Span),
    IfStatement(&'ast Ast<'ast>, Vec<Ast<'ast>>, Option<Vec<Ast<'ast>>>, Span),
    Closure(Option<Symbol>, Vec<Vec<Symbol>>, Vec<Ast<'ast>>, Span),
//  Define(Symbol, &'ast Ast<'ast>, Span),
//  Block(Vec<&'ast Ast<'ast>>, Span),
}


macro_rules! ast {
    ($arena: expr, $intern: expr, BoolLit($value: expr)) => {
        (Ast::BoolLit($value, Span::dummy()))
    };
    ($arena: expr, $intern: expr, StringLit($value: expr)) => {
        (Ast::StringLit($value.to_string(), Span::dummy()))
    };
    ($arena: expr, $intern: expr, FloatLit($value: expr)) => {
        (Ast::FloatLit($value, Span::dummy()))
    };
    ($arena: expr, $intern: expr, IntLit($value: expr)) => {
        (Ast::IntLit($value, Span::dummy()))
    };
    ($arena: expr, $intern: expr, SymbolLit($value: ident)) => {
        (Ast::SymbolLit($intern.intern(stringify!($value)), Span::dummy()))
    };
    ($arena: expr, $intern: expr, Identifier($value: ident)) => {
        (Ast::Identifier($intern.intern(stringify!($value)), Span::dummy()))
    };
    ($arena: expr, $intern: expr, ListLit($($name:tt $args: tt),*)) =>  {
        (Ast::ListLit(vec![
            $(ast!($arena, $intern, $name $args)),*
        ], Span::dummy()))
    };

    ($arena: expr, $intern: expr, Add($l_name:tt $l_args:tt, $r_name:tt $r_args:tt)) =>  {
        (Ast::Add($arena.alloc(ast!($arena, $intern, $l_name $l_args)),
                  $arena.alloc(ast!($arena, $intern, $r_name $r_args)), Span::dummy()))
    };
    ($arena: expr, $intern: expr, Sub($l_name:tt $l_args:tt, $r_name:tt $r_args:tt)) =>  {
        (Ast::Sub($arena.alloc(ast!($arena, $intern, $l_name $l_args)),
                  $arena.alloc(ast!($arena, $intern, $r_name $r_args)), Span::dummy()))
    };
    ($arena: expr, $intern: expr, Mul($l_name:tt $l_args:tt, $r_name:tt $r_args:tt)) =>  {
        (Ast::Mul($arena.alloc(ast!($arena, $intern, $l_name $l_args)),
                  $arena.alloc(ast!($arena, $intern, $r_name $r_args)), Span::dummy()))
    };
    ($arena: expr, $intern: expr, Div($l_name:tt $l_args:tt, $r_name:tt $r_args:tt)) =>  {
        (Ast::Div($arena.alloc(ast!($arena, $intern, $l_name $l_args)),
                  $arena.alloc(ast!($arena, $intern, $r_name $r_args)), Span::dummy()))
    };

    ($arena: expr, $intern: expr, Quote($name:tt $args: tt)) =>  {
        (Ast::Quote($arena.alloc(ast!($arena, $intern, $name $args)), Span::dummy()))
    };

    ($arena: expr, $intern: expr, IfExpr($cond_name:tt $cond_args:tt, $tru_name:tt $tru_args:tt, $fal_name:tt $fal_args:tt)) => {
        (Ast::IfExpression(
                $arena.alloc(ast!($arena, $intern, $cond_name $cond_args)),
                $arena.alloc(ast!($arena, $intern, $tru_name $tru_args)),
                $arena.alloc(ast!($arena, $intern, $fal_name $fal_args)),
                Span::dummy()))
    };
    ($arena: expr, $intern: expr,
        IfStatement($cond_name:tt $cond_args:tt,
                    {$($tru_name:tt $tru_args:tt);*}, {$($fal_name:tt $fal_args:tt);*})) => {
        (Ast::IfStatement(
                $arena.alloc(ast!($arena, $intern, $cond_name $cond_args)),
                vec![$(ast!($arena, $intern, $tru_name $tru_args)),*],
                Some(vec![$(ast!($arena, $intern, $fal_name $fal_args)),*]),
                Span::dummy()))
    };
    ($arena: expr, $intern: expr,
        IfStatement($cond_name:tt $cond_args:tt,
                    {$($tru_name:tt $tru_args:tt);*})) => {
        (Ast::IfStatement(
                $arena.alloc(ast!($arena, $intern, $cond_name $cond_args)),
                vec![$(ast!($arena, $intern, $tru_name $tru_args)),*],
                None,
                Span::dummy()))
    };

    ($arena: expr, $intern: expr, Lambda(($( ($( $symbol:tt ),*) ),*), $($name:tt $arg:tt),*)) => {
        (Ast::Closure(
                None,
                vec![$(vec![$($intern.intern(stringify!($symbol))),*]),*],
                vec![$(ast!($arena, $intern, $name $arg))*],
                Span::dummy()))
    };
    ($arena: expr, $intern: expr, Lambda($c_name:ident, ($( ($( $symbol:tt ),*) ),*), $($name:tt $arg:tt),*)) => {
        (Ast::Closure(
                Some($intern.intern(stringify!($c_name))),
                vec![$(vec![$($intern.intern(stringify!($symbol))),*]),*],
                vec![$(ast!($arena, $intern, $name $arg))*],
                Span::dummy()))
    };

    ($arena: expr, $intern: expr, Define($name: ident, $value_name:tt $value_args:tt)) => {
        (Ast::Define(
                $intern.intern(stringify!($name)),
                $arena.alloc(ast!($arena, $intern, $value_name $value_args)),
                Span::dummy()))
    };

    ($arena: expr, $intern: expr, Block($($name:tt $args: tt),*)) =>  {
        (Ast::Block(vec![
            $(ast!($arena, $intern, $name $args)),*
        ], Span::dummy()))
    };
    ($arena: expr, $intern: expr, FnCall($r_name:tt $r_args:tt ($($name:tt $args: tt),*))) =>  {
        (Ast::FnCall(
            $arena.alloc(ast!($arena, $intern, $r_name $r_args)),
            vec![$(ast!($arena, $intern, $name $args)),*],
            Span::dummy()))
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use typed_arena::Arena;

    #[test]
    fn terminals() {
        let arena = Arena::new();
        let mut interner = SymbolIntern::new();
        let arena = &arena;
        let interner = &mut interner;

        assert_eq!(parse_Expr(arena, interner, "5").unwrap(),
                   ast!(arena, interner, IntLit(5)));
        assert_eq!(parse_Expr(arena, interner, "5.5").unwrap(),
                   ast!(arena, interner, FloatLit(5.5)));
        assert_eq!(parse_Expr(arena, interner, "\"hello\"").unwrap(),
                   ast!(arena, interner, StringLit("hello")));
        assert_eq!(parse_Expr(arena, interner, "'foo").unwrap(),
                   ast!(arena, interner, SymbolLit(foo)));

        assert_eq!(parse_Expr(arena, interner, "foobar").unwrap(),
                   ast!(arena, interner, Identifier(foobar)));
    }
    #[test]
    fn operators() {
        let arena = Arena::new();
        let mut interner = SymbolIntern::new();
        let arena = &arena;
        let interner = &mut interner;

        assert_eq!(parse_Expr(arena, interner, "5 + 6").unwrap(),
                   ast!(arena, interner, Add(IntLit(5), IntLit(6))));

        assert_eq!(parse_Expr(arena, interner, "5 - 6").unwrap(),
                   ast!(arena, interner, Sub(IntLit(5), IntLit(6))));

        assert_eq!(parse_Expr(arena, interner, "5 * 6").unwrap(),
                   ast!(arena, interner, Mul(IntLit(5), IntLit(6))));

        assert_eq!(parse_Expr(arena, interner, "5 / 6").unwrap(),
                   ast!(arena, interner, Div(IntLit(5), IntLit(6))));

        assert_eq!(parse_Expr(arena, interner, "5 + 6 * 7").unwrap(),
                   ast!(arena, interner, Add(IntLit(5),
                                             Mul(IntLit(6), IntLit(7)))));

        assert_eq!(parse_Expr(arena, interner, "5 * 6 + 7").unwrap(),
                   ast!(arena, interner, Add(Mul(IntLit(5), IntLit(6)), IntLit(7))))
    }

    #[test]
    fn closures() {
        let arena = Arena::new();
        let mut interner = SymbolIntern::new();
        let arena = &arena;
        let interner = &mut interner;

        assert!(parse_Expr(arena, interner, "fn foo {}").is_err());
        assert!(parse_Expr(arena, interner, "fn").is_err());
        assert!(parse_Expr(arena, interner, "if").is_err());

        // no name
        assert_eq!(parse_Expr(arena, interner, "fn() { }").unwrap(),
                   ast!(arena, interner, Lambda((()),)));
        // no name with params
        assert_eq!(parse_Expr(arena, interner, "fn(a, b) { }").unwrap(),
                   ast!(arena, interner, Lambda(((a, b)),) ));

        // named
        assert_eq!(parse_Expr(arena, interner, "fn foo() { }").unwrap(),
                   ast!(arena, interner, Lambda(foo, (()),) ));
        // named with prams
        assert_eq!(parse_Expr(arena, interner, "fn foo(a, b,) { }").unwrap(),
                   ast!(arena, interner, Lambda(foo, ((a, b)),) ));

        // multi-param list
        assert_eq!(parse_Expr(arena, interner, "fn foo()()() { }").unwrap(),
                   ast!(arena, interner, Lambda(foo, ((), (), ()),) ));

        assert_eq!(parse_Expr(arena, interner, "fn foo(a, b)(c, d)(e, f) { }").unwrap(),
                   ast!(arena, interner, Lambda(foo, ((a, b), (c, d), (e, f)),) ));
    }

    #[test]
    fn func_calling() {
        let arena = Arena::new();
        let mut interner = SymbolIntern::new();
        let arena = &arena;
        let interner = &mut interner;

        assert_eq!(parse_Expr(arena, interner, "foo(a, b)").unwrap(),
                   ast!(arena, interner,
                        FnCall(Identifier(foo) (Identifier(a), Identifier(b)))));

        assert_eq!(parse_Expr(arena, interner, "foo(a, b)(c, d)").unwrap(),
            ast!(arena, interner,
                 FnCall(FnCall(Identifier(foo) (Identifier(a), Identifier(b)))
                        (Identifier(c), Identifier(d)))));

        assert_eq!(parse_Expr(arena, interner, "fn foo(a, b) {} (1, 2)").unwrap(),
            ast!(arena, interner,
                 FnCall(Lambda(foo, ((a, b)),) (IntLit(1), IntLit(2)))));

        assert_eq!(parse_Expr(arena, interner, "fn(a, b) {} (1, 2)").unwrap(),
            ast!(arena, interner,
                 FnCall(Lambda(((a, b)),) (IntLit(1), IntLit(2)))));
    }

    #[test]
    fn if_expr() {
        let arena = Arena::new();
        let mut interner = SymbolIntern::new();
        let arena = &arena;
        let interner = &mut interner;

        assert_eq!(parse_Expr(arena, interner, "if a then b else c").unwrap(),
                   ast!(arena, interner,
                        IfExpr(Identifier(a), Identifier(b), Identifier(c))));
        assert!(parse_Expr(arena, interner, "if a then {b}").is_err());
    }

    #[test]
    fn statements() {
        let arena = Arena::new();
        let mut interner = SymbolIntern::new();
        let arena = &arena;
        let interner = &mut interner;

        assert!(parse_Statement(arena, interner, "1 + 2").is_err());
        assert_eq!(parse_Statement(arena, interner, "1 + 2;").unwrap(),
            ast!(arena, interner, Add(IntLit(1), IntLit(2))));
        assert_eq!(parse_Statement(arena, interner, "if a then {} else {}").unwrap(),
            ast!(arena, interner, IfStatement(Identifier(a), {}, {})));
        assert_eq!(parse_Statement(arena, interner, "if a then {} ").unwrap(),
            ast!(arena, interner, IfStatement(Identifier(a), {})));
    }
}
