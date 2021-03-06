extern crate typed_arena;
#[macro_use]
extern crate gc;

mod intern;
mod syntax;

pub use intern::*;
pub use syntax::{parse_Expr, parse_Statement, parse_Program};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Span(u32, u32);

impl Span {
    pub fn dummy() -> Span {
        Span(0, 0)
    }
}

pub type AstRef<'ast> = &'ast Ast<'ast>;

#[derive(Clone, Debug, PartialEq)]
pub enum Ast<'ast> {
    Identifier(Symbol, Span),
    BoolLit(bool, Span),
    StringLit(String, Span),
    IntLit(i64, Span),
    FloatLit(f64, Span),
    SymbolLit(Symbol, Span),
    NilLit(Span),

    ListLit(Vec<Ast<'ast>>, Span),
    MapLit(Vec<(Ast<'ast>, AstRef<'ast>)>, Span),

    ListAccess(AstRef<'ast>, AstRef<'ast>, Span),

    Add(AstRef<'ast>, AstRef<'ast>, Span),
    Sub(AstRef<'ast>, AstRef<'ast>, Span),
    Mul(AstRef<'ast>, AstRef<'ast>, Span),
    Div(AstRef<'ast>, AstRef<'ast>, Span),

    LessThan(AstRef<'ast>, AstRef<'ast>, Span),
    LessThanOrEqual(AstRef<'ast>, AstRef<'ast>, Span),
    GreaterThan(AstRef<'ast>, AstRef<'ast>, Span),
    GreaterThanOrEqual(AstRef<'ast>, AstRef<'ast>, Span),
    Equal(AstRef<'ast>, AstRef<'ast>, Span),
    NotEqual(AstRef<'ast>, AstRef<'ast>, Span),

    FnCall(AstRef<'ast>, Vec<Ast<'ast>>, Span),
    IfExpression(AstRef<'ast>, AstRef<'ast>, AstRef<'ast>, Span),
    IfStatement(AstRef<'ast>, AstRef<'ast>, Option<AstRef<'ast>>, Span),
    Closure(Option<Symbol>, Vec<Vec<Symbol>>, AstRef<'ast>, Span),
    Assign(Symbol, AstRef<'ast>, Span),
    Define(Symbol, AstRef<'ast>, Span),
    BlockExpression(Vec<Ast<'ast>>, Span),
    BlockStatement(Vec<Ast<'ast>>, Span),
    Import(Vec<Symbol>, Symbol, AstRef<'ast>, Span),

    Reset(Vec<Ast<'ast>>, AstRef<'ast>, Span),
    Shift(Vec<Ast<'ast>>, AstRef<'ast>, Span),
}

impl <'ast> Ast<'ast> {
    pub fn dummy() -> Ast<'static> {
        Ast::BoolLit(true, Span::dummy())
    }

    pub fn span(&self) -> Span {
        match *self {
            Ast::BoolLit(_, s) |
            Ast::StringLit(_, s) |
            Ast::IntLit(_, s) |
            Ast::FloatLit(_, s) |
            Ast::SymbolLit(_, s) |
            Ast::NilLit(s) |
            Ast::Identifier(_, s) |
            Ast::ListLit(_,  s) |
            Ast::ListAccess(_,  _, s) |
            Ast::MapLit(_, s) |
            Ast::Add(_, _, s) |
            Ast::Sub(_, _, s) |
            Ast::Mul(_, _, s) |
            Ast::Div(_, _, s) |

            Ast::GreaterThan(_, _, s) |
            Ast::GreaterThanOrEqual(_, _, s) |
            Ast::LessThan(_, _, s) |
            Ast::LessThanOrEqual(_, _, s) |
            Ast::Equal(_, _, s) |
            Ast::NotEqual(_, _, s) |

            Ast::FnCall(_, _, s) |
            Ast::IfExpression(_, _, _, s) |
            Ast::IfStatement(_, _, _, s) |
            Ast::Closure(_, _, _, s) |
            Ast::Assign(_, _, s) |
            Ast::Define(_, _, s) |
            Ast::BlockExpression(_, s) |
            Ast::BlockStatement(_, s) |
            Ast::Shift(_, _, s) |
            Ast::Reset(_, _, s)  |
            Ast::Import(_, _, _, s)  => s
        }
    }

    fn with_default_span(self) -> Ast<'ast> {
        use self::Ast::*;
        match self {
            BoolLit(a, _) => BoolLit(a, Span::dummy()),
            StringLit(a, _) => StringLit(a, Span::dummy()),
            IntLit(a, _) => IntLit(a, Span::dummy()),
            NilLit(_) => NilLit(Span::dummy()),
            FloatLit(a, _) => FloatLit(a, Span::dummy()),
            SymbolLit(a, _) => SymbolLit(a, Span::dummy()),
            Identifier(a, _) => Identifier(a, Span::dummy()),
            ListLit(a,  _) => ListLit(a, Span::dummy()),
            ListAccess(a, b,  _) => ListAccess(a, b, Span::dummy()),
            MapLit(a, _) => MapLit(a, Span::dummy()),
            Add(l, r, _) => Add(l, r, Span::dummy()),
            Sub(l, r, _) => Sub(l, r, Span::dummy()),
            Mul(l, r, _) => Mul(l, r, Span::dummy()),
            Div(l, r, _) => Div(l, r, Span::dummy()),
            LessThan(l, r, _) => LessThan(l, r, Span::dummy()),
            LessThanOrEqual(l, r, _) => LessThanOrEqual(l, r, Span::dummy()),
            GreaterThan(l, r, _) => GreaterThan(l, r, Span::dummy()),
            GreaterThanOrEqual(l, r, _) => GreaterThanOrEqual(l, r, Span::dummy()),
            Equal(l, r, _) => Equal(l, r, Span::dummy()),
            NotEqual(l, r, _) => NotEqual(l, r, Span::dummy()),
            FnCall(o, a, _) => FnCall(o, a, Span::dummy()),
            IfExpression(c, t, f, _) => IfExpression(c, t, f, Span::dummy()),
            IfStatement(c, t, f, _) => IfStatement(c, t, f, Span::dummy()),
            Closure(n, a, b, _) => Closure(n, a, b, Span::dummy()),
            Assign(n, v, _) => Assign(n, v, Span::dummy()),
            Define(n, v, _) => Define(n, v, Span::dummy()),
            BlockExpression(b, _) => BlockExpression(b, Span::dummy()),
            BlockStatement(b, _) => BlockStatement(b, Span::dummy()),
            Shift(a, b, _) => Shift(a, b, Span::dummy()),
            Reset(a, b, _) => Reset(a, b, Span::dummy()),
            Import(a, b, c, _) => Import(a, b, c, Span::dummy()),
        }
    }

    pub fn equals_sans_span(&self, other: &Ast<'ast>) -> bool {
        self.clone().with_default_span() == other.clone().with_default_span()
    }
}


#[macro_export]
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
                $arena.alloc(Ast::Block(vec![$(ast!($arena, $intern, $name $arg))*], Span::dummy())),
                Span::dummy()))
    };
    ($arena: expr, $intern: expr, Lambda($c_name:ident, ($( ($( $symbol:tt ),*) ),*), $($name:tt $arg:tt),*)) => {
        (Ast::Closure(
                Some($intern.intern(stringify!($c_name))),
                vec![$(vec![$($intern.intern(stringify!($symbol))),*]),*],
                $arena.alloc(Ast::Block(vec![$(ast!($arena, $intern, $name $arg))*], Span::dummy())),
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
