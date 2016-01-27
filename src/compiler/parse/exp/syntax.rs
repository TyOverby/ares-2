#![allow(unused_imports)]
#![allow(unused_variables)]
use std::str::FromStr;
use super::{Ast, Ident};
use std::boxed::Box;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use super::super::{Ast, Ident};
    use std::boxed::Box;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Expr<
        'input,
    >(
        input: &'input str,
    ) -> Result<Ast<'input>, __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, None, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____Expr(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<'input> {
        Expr(Ast<'input>),
        Factor(Ast<'input>),
        FloatLit(Ast<'input>),
        Ident(Ident<'input>),
        IntLit(Ast<'input>),
        StringLit(Ast<'input>),
        SymbolLit(Ast<'input>),
        Term(Ast<'input>),
        ____Expr(Ast<'input>),
    }

    // State 0
    //   Expr = (*) Expr "+" Factor [EOF]
    //   Expr = (*) Expr "+" Factor ["+"]
    //   Expr = (*) Expr "+" Factor ["-"]
    //   Expr = (*) Expr "-" Factor [EOF]
    //   Expr = (*) Expr "-" Factor ["+"]
    //   Expr = (*) Expr "-" Factor ["-"]
    //   Expr = (*) Factor [EOF]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [EOF]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [EOF]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [EOF]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# [EOF]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["*"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["+"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["-"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["/"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [EOF]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["*"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["-"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["/"]
    //   IntLit = (*) r#"[0-9]+"# [EOF]
    //   IntLit = (*) r#"[0-9]+"# ["*"]
    //   IntLit = (*) r#"[0-9]+"# ["+"]
    //   IntLit = (*) r#"[0-9]+"# ["-"]
    //   IntLit = (*) r#"[0-9]+"# ["/"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# [EOF]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["*"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["+"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["-"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["/"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# [EOF]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["*"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["-"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["/"]
    //   Term = (*) FloatLit [EOF]
    //   Term = (*) FloatLit ["*"]
    //   Term = (*) FloatLit ["+"]
    //   Term = (*) FloatLit ["-"]
    //   Term = (*) FloatLit ["/"]
    //   Term = (*) Ident [EOF]
    //   Term = (*) Ident ["*"]
    //   Term = (*) Ident ["+"]
    //   Term = (*) Ident ["-"]
    //   Term = (*) Ident ["/"]
    //   Term = (*) IntLit [EOF]
    //   Term = (*) IntLit ["*"]
    //   Term = (*) IntLit ["+"]
    //   Term = (*) IntLit ["-"]
    //   Term = (*) IntLit ["/"]
    //   Term = (*) StringLit [EOF]
    //   Term = (*) StringLit ["*"]
    //   Term = (*) StringLit ["+"]
    //   Term = (*) StringLit ["-"]
    //   Term = (*) StringLit ["/"]
    //   Term = (*) SymbolLit [EOF]
    //   Term = (*) SymbolLit ["*"]
    //   Term = (*) SymbolLit ["+"]
    //   Term = (*) SymbolLit ["-"]
    //   Term = (*) SymbolLit ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   __Expr = (*) Expr [EOF]
    //
    //   "(" -> Shift(S9)
    //   r#"\"(\\\\.|[^\"])*\""# -> Shift(S10)
    //   r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S11)
    //   r#"[0-9]+"# -> Shift(S12)
    //   r#"[0-9]+\\.[0-9]*"# -> Shift(S13)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S14)
    //
    //   Expr -> S1
    //   Factor -> S2
    //   FloatLit -> S3
    //   Ident -> S4
    //   IntLit -> S5
    //   StringLit -> S6
    //   SymbolLit -> S7
    //   Term -> S8
    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state10(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state11(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state12(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym0));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expr(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::FloatLit(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Ident(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::IntLit(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state5(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::StringLit(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state6(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::SymbolLit(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state7(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   Expr = Expr (*) "+" Factor [EOF]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [EOF]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   __Expr = Expr (*) [EOF]
    //
    //   EOF -> Reduce(__Expr = Expr => ActionFn(0);)
    //   "+" -> Shift(S15)
    //   "-" -> Shift(S16)
    //
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Ast<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state15(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state16(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action0(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 2
    //   Expr = Factor (*) [EOF]
    //   Expr = Factor (*) ["+"]
    //   Expr = Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [EOF]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [EOF]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   EOF -> Reduce(Expr = Factor => ActionFn(3);)
    //   "*" -> Shift(S17)
    //   "+" -> Reduce(Expr = Factor => ActionFn(3);)
    //   "-" -> Reduce(Expr = Factor => ActionFn(3);)
    //   "/" -> Shift(S18)
    //
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Ast<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state17(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state18(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            None |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 3
    //   Term = FloatLit (*) [EOF]
    //   Term = FloatLit (*) ["*"]
    //   Term = FloatLit (*) ["+"]
    //   Term = FloatLit (*) ["-"]
    //   Term = FloatLit (*) ["/"]
    //
    //   EOF -> Reduce(Term = FloatLit => ActionFn(7);)
    //   "*" -> Reduce(Term = FloatLit => ActionFn(7);)
    //   "+" -> Reduce(Term = FloatLit => ActionFn(7);)
    //   "-" -> Reduce(Term = FloatLit => ActionFn(7);)
    //   "/" -> Reduce(Term = FloatLit => ActionFn(7);)
    //
    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Ast<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 4
    //   Term = Ident (*) [EOF]
    //   Term = Ident (*) ["*"]
    //   Term = Ident (*) ["+"]
    //   Term = Ident (*) ["-"]
    //   Term = Ident (*) ["/"]
    //
    //   EOF -> Reduce(Term = Ident => ActionFn(11);)
    //   "*" -> Reduce(Term = Ident => ActionFn(11);)
    //   "+" -> Reduce(Term = Ident => ActionFn(11);)
    //   "-" -> Reduce(Term = Ident => ActionFn(11);)
    //   "/" -> Reduce(Term = Ident => ActionFn(11);)
    //
    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Ident<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action11(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 5
    //   Term = IntLit (*) [EOF]
    //   Term = IntLit (*) ["*"]
    //   Term = IntLit (*) ["+"]
    //   Term = IntLit (*) ["-"]
    //   Term = IntLit (*) ["/"]
    //
    //   EOF -> Reduce(Term = IntLit => ActionFn(8);)
    //   "*" -> Reduce(Term = IntLit => ActionFn(8);)
    //   "+" -> Reduce(Term = IntLit => ActionFn(8);)
    //   "-" -> Reduce(Term = IntLit => ActionFn(8);)
    //   "/" -> Reduce(Term = IntLit => ActionFn(8);)
    //
    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Ast<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 6
    //   Term = StringLit (*) [EOF]
    //   Term = StringLit (*) ["*"]
    //   Term = StringLit (*) ["+"]
    //   Term = StringLit (*) ["-"]
    //   Term = StringLit (*) ["/"]
    //
    //   EOF -> Reduce(Term = StringLit => ActionFn(9);)
    //   "*" -> Reduce(Term = StringLit => ActionFn(9);)
    //   "+" -> Reduce(Term = StringLit => ActionFn(9);)
    //   "-" -> Reduce(Term = StringLit => ActionFn(9);)
    //   "/" -> Reduce(Term = StringLit => ActionFn(9);)
    //
    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Ast<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action9(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 7
    //   Term = SymbolLit (*) [EOF]
    //   Term = SymbolLit (*) ["*"]
    //   Term = SymbolLit (*) ["+"]
    //   Term = SymbolLit (*) ["-"]
    //   Term = SymbolLit (*) ["/"]
    //
    //   EOF -> Reduce(Term = SymbolLit => ActionFn(10);)
    //   "*" -> Reduce(Term = SymbolLit => ActionFn(10);)
    //   "+" -> Reduce(Term = SymbolLit => ActionFn(10);)
    //   "-" -> Reduce(Term = SymbolLit => ActionFn(10);)
    //   "/" -> Reduce(Term = SymbolLit => ActionFn(10);)
    //
    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Ast<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action10(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 8
    //   Factor = Term (*) [EOF]
    //   Factor = Term (*) ["*"]
    //   Factor = Term (*) ["+"]
    //   Factor = Term (*) ["-"]
    //   Factor = Term (*) ["/"]
    //
    //   EOF -> Reduce(Factor = Term => ActionFn(6);)
    //   "*" -> Reduce(Factor = Term => ActionFn(6);)
    //   "+" -> Reduce(Factor = Term => ActionFn(6);)
    //   "-" -> Reduce(Factor = Term => ActionFn(6);)
    //   "/" -> Reduce(Factor = Term => ActionFn(6);)
    //
    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Ast<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 9
    //   Expr = (*) Expr "+" Factor [")"]
    //   Expr = (*) Expr "+" Factor ["+"]
    //   Expr = (*) Expr "+" Factor ["-"]
    //   Expr = (*) Expr "-" Factor [")"]
    //   Expr = (*) Expr "-" Factor ["+"]
    //   Expr = (*) Expr "-" Factor ["-"]
    //   Expr = (*) Factor [")"]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# [")"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["*"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["+"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["-"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["/"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [")"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["*"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["-"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["/"]
    //   IntLit = (*) r#"[0-9]+"# [")"]
    //   IntLit = (*) r#"[0-9]+"# ["*"]
    //   IntLit = (*) r#"[0-9]+"# ["+"]
    //   IntLit = (*) r#"[0-9]+"# ["-"]
    //   IntLit = (*) r#"[0-9]+"# ["/"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# [")"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["*"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["+"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["-"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["/"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# [")"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["*"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["-"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["/"]
    //   Term = (*) FloatLit [")"]
    //   Term = (*) FloatLit ["*"]
    //   Term = (*) FloatLit ["+"]
    //   Term = (*) FloatLit ["-"]
    //   Term = (*) FloatLit ["/"]
    //   Term = (*) Ident [")"]
    //   Term = (*) Ident ["*"]
    //   Term = (*) Ident ["+"]
    //   Term = (*) Ident ["-"]
    //   Term = (*) Ident ["/"]
    //   Term = (*) IntLit [")"]
    //   Term = (*) IntLit ["*"]
    //   Term = (*) IntLit ["+"]
    //   Term = (*) IntLit ["-"]
    //   Term = (*) IntLit ["/"]
    //   Term = (*) StringLit [")"]
    //   Term = (*) StringLit ["*"]
    //   Term = (*) StringLit ["+"]
    //   Term = (*) StringLit ["-"]
    //   Term = (*) StringLit ["/"]
    //   Term = (*) SymbolLit [")"]
    //   Term = (*) SymbolLit ["*"]
    //   Term = (*) SymbolLit ["+"]
    //   Term = (*) SymbolLit ["-"]
    //   Term = (*) SymbolLit ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = "(" (*) Expr ")" [EOF]
    //   Term = "(" (*) Expr ")" ["*"]
    //   Term = "(" (*) Expr ")" ["+"]
    //   Term = "(" (*) Expr ")" ["-"]
    //   Term = "(" (*) Expr ")" ["/"]
    //
    //   "(" -> Shift(S27)
    //   r#"\"(\\\\.|[^\"])*\""# -> Shift(S28)
    //   r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S29)
    //   r#"[0-9]+"# -> Shift(S30)
    //   r#"[0-9]+\\.[0-9]*"# -> Shift(S31)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S32)
    //
    //   Expr -> S19
    //   Factor -> S20
    //   FloatLit -> S21
    //   Ident -> S22
    //   IntLit -> S23
    //   StringLit -> S24
    //   SymbolLit -> S25
    //   Term -> S26
    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state27(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state28(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state29(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state30(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state31(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state32(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state19(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state20(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::FloatLit(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state21(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Ident(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state22(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::IntLit(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state23(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::StringLit(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state24(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::SymbolLit(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state25(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state26(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 10
    //   StringLit = r#"\"(\\\\.|[^\"])*\""# (*) [EOF]
    //   StringLit = r#"\"(\\\\.|[^\"])*\""# (*) ["*"]
    //   StringLit = r#"\"(\\\\.|[^\"])*\""# (*) ["+"]
    //   StringLit = r#"\"(\\\\.|[^\"])*\""# (*) ["-"]
    //   StringLit = r#"\"(\\\\.|[^\"])*\""# (*) ["/"]
    //
    //   EOF -> Reduce(StringLit = r#"\"(\\\\.|[^\"])*\""# => ActionFn(15);)
    //   "*" -> Reduce(StringLit = r#"\"(\\\\.|[^\"])*\""# => ActionFn(15);)
    //   "+" -> Reduce(StringLit = r#"\"(\\\\.|[^\"])*\""# => ActionFn(15);)
    //   "-" -> Reduce(StringLit = r#"\"(\\\\.|[^\"])*\""# => ActionFn(15);)
    //   "/" -> Reduce(StringLit = r#"\"(\\\\.|[^\"])*\""# => ActionFn(15);)
    //
    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action15(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::StringLit(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 11
    //   SymbolLit = r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# (*) [EOF]
    //   SymbolLit = r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["*"]
    //   SymbolLit = r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["+"]
    //   SymbolLit = r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["-"]
    //   SymbolLit = r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["/"]
    //
    //   EOF -> Reduce(SymbolLit = r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(16);)
    //   "*" -> Reduce(SymbolLit = r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(16);)
    //   "+" -> Reduce(SymbolLit = r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(16);)
    //   "-" -> Reduce(SymbolLit = r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(16);)
    //   "/" -> Reduce(SymbolLit = r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(16);)
    //
    pub fn __state11<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action16(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::SymbolLit(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 12
    //   IntLit = r#"[0-9]+"# (*) [EOF]
    //   IntLit = r#"[0-9]+"# (*) ["*"]
    //   IntLit = r#"[0-9]+"# (*) ["+"]
    //   IntLit = r#"[0-9]+"# (*) ["-"]
    //   IntLit = r#"[0-9]+"# (*) ["/"]
    //
    //   EOF -> Reduce(IntLit = r#"[0-9]+"# => ActionFn(13);)
    //   "*" -> Reduce(IntLit = r#"[0-9]+"# => ActionFn(13);)
    //   "+" -> Reduce(IntLit = r#"[0-9]+"# => ActionFn(13);)
    //   "-" -> Reduce(IntLit = r#"[0-9]+"# => ActionFn(13);)
    //   "/" -> Reduce(IntLit = r#"[0-9]+"# => ActionFn(13);)
    //
    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action13(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::IntLit(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 13
    //   FloatLit = r#"[0-9]+\\.[0-9]*"# (*) [EOF]
    //   FloatLit = r#"[0-9]+\\.[0-9]*"# (*) ["*"]
    //   FloatLit = r#"[0-9]+\\.[0-9]*"# (*) ["+"]
    //   FloatLit = r#"[0-9]+\\.[0-9]*"# (*) ["-"]
    //   FloatLit = r#"[0-9]+\\.[0-9]*"# (*) ["/"]
    //
    //   EOF -> Reduce(FloatLit = r#"[0-9]+\\.[0-9]*"# => ActionFn(14);)
    //   "*" -> Reduce(FloatLit = r#"[0-9]+\\.[0-9]*"# => ActionFn(14);)
    //   "+" -> Reduce(FloatLit = r#"[0-9]+\\.[0-9]*"# => ActionFn(14);)
    //   "-" -> Reduce(FloatLit = r#"[0-9]+\\.[0-9]*"# => ActionFn(14);)
    //   "/" -> Reduce(FloatLit = r#"[0-9]+\\.[0-9]*"# => ActionFn(14);)
    //
    pub fn __state13<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action14(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::FloatLit(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 14
    //   Ident = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) [EOF]
    //   Ident = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["*"]
    //   Ident = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["+"]
    //   Ident = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["-"]
    //   Ident = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["/"]
    //
    //   EOF -> Reduce(Ident = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(17);)
    //   "*" -> Reduce(Ident = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(17);)
    //   "+" -> Reduce(Ident = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(17);)
    //   "-" -> Reduce(Ident = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(17);)
    //   "/" -> Reduce(Ident = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(17);)
    //
    pub fn __state14<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action17(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Ident(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 15
    //   Expr = Expr "+" (*) Factor [EOF]
    //   Expr = Expr "+" (*) Factor ["+"]
    //   Expr = Expr "+" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [EOF]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [EOF]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [EOF]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# [EOF]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["*"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["+"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["-"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["/"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [EOF]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["*"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["-"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["/"]
    //   IntLit = (*) r#"[0-9]+"# [EOF]
    //   IntLit = (*) r#"[0-9]+"# ["*"]
    //   IntLit = (*) r#"[0-9]+"# ["+"]
    //   IntLit = (*) r#"[0-9]+"# ["-"]
    //   IntLit = (*) r#"[0-9]+"# ["/"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# [EOF]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["*"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["+"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["-"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["/"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# [EOF]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["*"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["-"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["/"]
    //   Term = (*) FloatLit [EOF]
    //   Term = (*) FloatLit ["*"]
    //   Term = (*) FloatLit ["+"]
    //   Term = (*) FloatLit ["-"]
    //   Term = (*) FloatLit ["/"]
    //   Term = (*) Ident [EOF]
    //   Term = (*) Ident ["*"]
    //   Term = (*) Ident ["+"]
    //   Term = (*) Ident ["-"]
    //   Term = (*) Ident ["/"]
    //   Term = (*) IntLit [EOF]
    //   Term = (*) IntLit ["*"]
    //   Term = (*) IntLit ["+"]
    //   Term = (*) IntLit ["-"]
    //   Term = (*) IntLit ["/"]
    //   Term = (*) StringLit [EOF]
    //   Term = (*) StringLit ["*"]
    //   Term = (*) StringLit ["+"]
    //   Term = (*) StringLit ["-"]
    //   Term = (*) StringLit ["/"]
    //   Term = (*) SymbolLit [EOF]
    //   Term = (*) SymbolLit ["*"]
    //   Term = (*) SymbolLit ["+"]
    //   Term = (*) SymbolLit ["-"]
    //   Term = (*) SymbolLit ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //
    //   "(" -> Shift(S9)
    //   r#"\"(\\\\.|[^\"])*\""# -> Shift(S10)
    //   r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S11)
    //   r#"[0-9]+"# -> Shift(S12)
    //   r#"[0-9]+\\.[0-9]*"# -> Shift(S13)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S14)
    //
    //   Factor -> S33
    //   FloatLit -> S3
    //   Ident -> S4
    //   IntLit -> S5
    //   StringLit -> S6
    //   SymbolLit -> S7
    //   Term -> S8
    pub fn __state15<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Ast<'input>>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state10(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state11(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state12(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state33(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::FloatLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Ident(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::IntLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state5(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::StringLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state6(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::SymbolLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state7(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 16
    //   Expr = Expr "-" (*) Factor [EOF]
    //   Expr = Expr "-" (*) Factor ["+"]
    //   Expr = Expr "-" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [EOF]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [EOF]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [EOF]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# [EOF]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["*"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["+"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["-"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["/"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [EOF]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["*"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["-"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["/"]
    //   IntLit = (*) r#"[0-9]+"# [EOF]
    //   IntLit = (*) r#"[0-9]+"# ["*"]
    //   IntLit = (*) r#"[0-9]+"# ["+"]
    //   IntLit = (*) r#"[0-9]+"# ["-"]
    //   IntLit = (*) r#"[0-9]+"# ["/"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# [EOF]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["*"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["+"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["-"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["/"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# [EOF]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["*"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["-"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["/"]
    //   Term = (*) FloatLit [EOF]
    //   Term = (*) FloatLit ["*"]
    //   Term = (*) FloatLit ["+"]
    //   Term = (*) FloatLit ["-"]
    //   Term = (*) FloatLit ["/"]
    //   Term = (*) Ident [EOF]
    //   Term = (*) Ident ["*"]
    //   Term = (*) Ident ["+"]
    //   Term = (*) Ident ["-"]
    //   Term = (*) Ident ["/"]
    //   Term = (*) IntLit [EOF]
    //   Term = (*) IntLit ["*"]
    //   Term = (*) IntLit ["+"]
    //   Term = (*) IntLit ["-"]
    //   Term = (*) IntLit ["/"]
    //   Term = (*) StringLit [EOF]
    //   Term = (*) StringLit ["*"]
    //   Term = (*) StringLit ["+"]
    //   Term = (*) StringLit ["-"]
    //   Term = (*) StringLit ["/"]
    //   Term = (*) SymbolLit [EOF]
    //   Term = (*) SymbolLit ["*"]
    //   Term = (*) SymbolLit ["+"]
    //   Term = (*) SymbolLit ["-"]
    //   Term = (*) SymbolLit ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //
    //   "(" -> Shift(S9)
    //   r#"\"(\\\\.|[^\"])*\""# -> Shift(S10)
    //   r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S11)
    //   r#"[0-9]+"# -> Shift(S12)
    //   r#"[0-9]+\\.[0-9]*"# -> Shift(S13)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S14)
    //
    //   Factor -> S34
    //   FloatLit -> S3
    //   Ident -> S4
    //   IntLit -> S5
    //   StringLit -> S6
    //   SymbolLit -> S7
    //   Term -> S8
    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Ast<'input>>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state10(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state11(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state12(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state34(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::FloatLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Ident(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::IntLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state5(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::StringLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state6(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::SymbolLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state7(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state8(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 17
    //   Factor = Factor "*" (*) Term [EOF]
    //   Factor = Factor "*" (*) Term ["*"]
    //   Factor = Factor "*" (*) Term ["+"]
    //   Factor = Factor "*" (*) Term ["-"]
    //   Factor = Factor "*" (*) Term ["/"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# [EOF]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["*"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["+"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["-"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["/"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [EOF]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["*"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["-"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["/"]
    //   IntLit = (*) r#"[0-9]+"# [EOF]
    //   IntLit = (*) r#"[0-9]+"# ["*"]
    //   IntLit = (*) r#"[0-9]+"# ["+"]
    //   IntLit = (*) r#"[0-9]+"# ["-"]
    //   IntLit = (*) r#"[0-9]+"# ["/"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# [EOF]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["*"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["+"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["-"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["/"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# [EOF]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["*"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["-"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["/"]
    //   Term = (*) FloatLit [EOF]
    //   Term = (*) FloatLit ["*"]
    //   Term = (*) FloatLit ["+"]
    //   Term = (*) FloatLit ["-"]
    //   Term = (*) FloatLit ["/"]
    //   Term = (*) Ident [EOF]
    //   Term = (*) Ident ["*"]
    //   Term = (*) Ident ["+"]
    //   Term = (*) Ident ["-"]
    //   Term = (*) Ident ["/"]
    //   Term = (*) IntLit [EOF]
    //   Term = (*) IntLit ["*"]
    //   Term = (*) IntLit ["+"]
    //   Term = (*) IntLit ["-"]
    //   Term = (*) IntLit ["/"]
    //   Term = (*) StringLit [EOF]
    //   Term = (*) StringLit ["*"]
    //   Term = (*) StringLit ["+"]
    //   Term = (*) StringLit ["-"]
    //   Term = (*) StringLit ["/"]
    //   Term = (*) SymbolLit [EOF]
    //   Term = (*) SymbolLit ["*"]
    //   Term = (*) SymbolLit ["+"]
    //   Term = (*) SymbolLit ["-"]
    //   Term = (*) SymbolLit ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //
    //   "(" -> Shift(S9)
    //   r#"\"(\\\\.|[^\"])*\""# -> Shift(S10)
    //   r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S11)
    //   r#"[0-9]+"# -> Shift(S12)
    //   r#"[0-9]+\\.[0-9]*"# -> Shift(S13)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S14)
    //
    //   FloatLit -> S3
    //   Ident -> S4
    //   IntLit -> S5
    //   StringLit -> S6
    //   SymbolLit -> S7
    //   Term -> S35
    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Ast<'input>>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state10(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state11(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state12(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::FloatLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Ident(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::IntLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state5(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::StringLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state6(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::SymbolLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state7(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state35(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 18
    //   Factor = Factor "/" (*) Term [EOF]
    //   Factor = Factor "/" (*) Term ["*"]
    //   Factor = Factor "/" (*) Term ["+"]
    //   Factor = Factor "/" (*) Term ["-"]
    //   Factor = Factor "/" (*) Term ["/"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# [EOF]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["*"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["+"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["-"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["/"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [EOF]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["*"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["-"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["/"]
    //   IntLit = (*) r#"[0-9]+"# [EOF]
    //   IntLit = (*) r#"[0-9]+"# ["*"]
    //   IntLit = (*) r#"[0-9]+"# ["+"]
    //   IntLit = (*) r#"[0-9]+"# ["-"]
    //   IntLit = (*) r#"[0-9]+"# ["/"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# [EOF]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["*"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["+"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["-"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["/"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# [EOF]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["*"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["-"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["/"]
    //   Term = (*) FloatLit [EOF]
    //   Term = (*) FloatLit ["*"]
    //   Term = (*) FloatLit ["+"]
    //   Term = (*) FloatLit ["-"]
    //   Term = (*) FloatLit ["/"]
    //   Term = (*) Ident [EOF]
    //   Term = (*) Ident ["*"]
    //   Term = (*) Ident ["+"]
    //   Term = (*) Ident ["-"]
    //   Term = (*) Ident ["/"]
    //   Term = (*) IntLit [EOF]
    //   Term = (*) IntLit ["*"]
    //   Term = (*) IntLit ["+"]
    //   Term = (*) IntLit ["-"]
    //   Term = (*) IntLit ["/"]
    //   Term = (*) StringLit [EOF]
    //   Term = (*) StringLit ["*"]
    //   Term = (*) StringLit ["+"]
    //   Term = (*) StringLit ["-"]
    //   Term = (*) StringLit ["/"]
    //   Term = (*) SymbolLit [EOF]
    //   Term = (*) SymbolLit ["*"]
    //   Term = (*) SymbolLit ["+"]
    //   Term = (*) SymbolLit ["-"]
    //   Term = (*) SymbolLit ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //
    //   "(" -> Shift(S9)
    //   r#"\"(\\\\.|[^\"])*\""# -> Shift(S10)
    //   r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S11)
    //   r#"[0-9]+"# -> Shift(S12)
    //   r#"[0-9]+\\.[0-9]*"# -> Shift(S13)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S14)
    //
    //   FloatLit -> S3
    //   Ident -> S4
    //   IntLit -> S5
    //   StringLit -> S6
    //   SymbolLit -> S7
    //   Term -> S36
    pub fn __state18<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Ast<'input>>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state10(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state11(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state12(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::FloatLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Ident(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state4(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::IntLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state5(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::StringLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state6(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::SymbolLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state7(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state36(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 19
    //   Expr = Expr (*) "+" Factor [")"]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [")"]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   Term = "(" Expr (*) ")" [EOF]
    //   Term = "(" Expr (*) ")" ["*"]
    //   Term = "(" Expr (*) ")" ["+"]
    //   Term = "(" Expr (*) ")" ["-"]
    //   Term = "(" Expr (*) ")" ["/"]
    //
    //   ")" -> Shift(S37)
    //   "+" -> Shift(S38)
    //   "-" -> Shift(S39)
    //
    pub fn __state19<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Ast<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state37(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state38(input, __lookbehind, __tokens, __sym1, __sym2));
            }
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state39(input, __lookbehind, __tokens, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 20
    //   Expr = Factor (*) [")"]
    //   Expr = Factor (*) ["+"]
    //   Expr = Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   ")" -> Reduce(Expr = Factor => ActionFn(3);)
    //   "*" -> Shift(S40)
    //   "+" -> Reduce(Expr = Factor => ActionFn(3);)
    //   "-" -> Reduce(Expr = Factor => ActionFn(3);)
    //   "/" -> Shift(S41)
    //
    pub fn __state20<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Ast<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state40(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state41(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 21
    //   Term = FloatLit (*) [")"]
    //   Term = FloatLit (*) ["*"]
    //   Term = FloatLit (*) ["+"]
    //   Term = FloatLit (*) ["-"]
    //   Term = FloatLit (*) ["/"]
    //
    //   ")" -> Reduce(Term = FloatLit => ActionFn(7);)
    //   "*" -> Reduce(Term = FloatLit => ActionFn(7);)
    //   "+" -> Reduce(Term = FloatLit => ActionFn(7);)
    //   "-" -> Reduce(Term = FloatLit => ActionFn(7);)
    //   "/" -> Reduce(Term = FloatLit => ActionFn(7);)
    //
    pub fn __state21<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Ast<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 22
    //   Term = Ident (*) [")"]
    //   Term = Ident (*) ["*"]
    //   Term = Ident (*) ["+"]
    //   Term = Ident (*) ["-"]
    //   Term = Ident (*) ["/"]
    //
    //   ")" -> Reduce(Term = Ident => ActionFn(11);)
    //   "*" -> Reduce(Term = Ident => ActionFn(11);)
    //   "+" -> Reduce(Term = Ident => ActionFn(11);)
    //   "-" -> Reduce(Term = Ident => ActionFn(11);)
    //   "/" -> Reduce(Term = Ident => ActionFn(11);)
    //
    pub fn __state22<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Ident<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action11(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 23
    //   Term = IntLit (*) [")"]
    //   Term = IntLit (*) ["*"]
    //   Term = IntLit (*) ["+"]
    //   Term = IntLit (*) ["-"]
    //   Term = IntLit (*) ["/"]
    //
    //   ")" -> Reduce(Term = IntLit => ActionFn(8);)
    //   "*" -> Reduce(Term = IntLit => ActionFn(8);)
    //   "+" -> Reduce(Term = IntLit => ActionFn(8);)
    //   "-" -> Reduce(Term = IntLit => ActionFn(8);)
    //   "/" -> Reduce(Term = IntLit => ActionFn(8);)
    //
    pub fn __state23<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Ast<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 24
    //   Term = StringLit (*) [")"]
    //   Term = StringLit (*) ["*"]
    //   Term = StringLit (*) ["+"]
    //   Term = StringLit (*) ["-"]
    //   Term = StringLit (*) ["/"]
    //
    //   ")" -> Reduce(Term = StringLit => ActionFn(9);)
    //   "*" -> Reduce(Term = StringLit => ActionFn(9);)
    //   "+" -> Reduce(Term = StringLit => ActionFn(9);)
    //   "-" -> Reduce(Term = StringLit => ActionFn(9);)
    //   "/" -> Reduce(Term = StringLit => ActionFn(9);)
    //
    pub fn __state24<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Ast<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action9(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 25
    //   Term = SymbolLit (*) [")"]
    //   Term = SymbolLit (*) ["*"]
    //   Term = SymbolLit (*) ["+"]
    //   Term = SymbolLit (*) ["-"]
    //   Term = SymbolLit (*) ["/"]
    //
    //   ")" -> Reduce(Term = SymbolLit => ActionFn(10);)
    //   "*" -> Reduce(Term = SymbolLit => ActionFn(10);)
    //   "+" -> Reduce(Term = SymbolLit => ActionFn(10);)
    //   "-" -> Reduce(Term = SymbolLit => ActionFn(10);)
    //   "/" -> Reduce(Term = SymbolLit => ActionFn(10);)
    //
    pub fn __state25<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Ast<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action10(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 26
    //   Factor = Term (*) [")"]
    //   Factor = Term (*) ["*"]
    //   Factor = Term (*) ["+"]
    //   Factor = Term (*) ["-"]
    //   Factor = Term (*) ["/"]
    //
    //   ")" -> Reduce(Factor = Term => ActionFn(6);)
    //   "*" -> Reduce(Factor = Term => ActionFn(6);)
    //   "+" -> Reduce(Factor = Term => ActionFn(6);)
    //   "-" -> Reduce(Factor = Term => ActionFn(6);)
    //   "/" -> Reduce(Factor = Term => ActionFn(6);)
    //
    pub fn __state26<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Ast<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 27
    //   Expr = (*) Expr "+" Factor [")"]
    //   Expr = (*) Expr "+" Factor ["+"]
    //   Expr = (*) Expr "+" Factor ["-"]
    //   Expr = (*) Expr "-" Factor [")"]
    //   Expr = (*) Expr "-" Factor ["+"]
    //   Expr = (*) Expr "-" Factor ["-"]
    //   Expr = (*) Factor [")"]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# [")"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["*"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["+"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["-"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["/"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [")"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["*"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["-"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["/"]
    //   IntLit = (*) r#"[0-9]+"# [")"]
    //   IntLit = (*) r#"[0-9]+"# ["*"]
    //   IntLit = (*) r#"[0-9]+"# ["+"]
    //   IntLit = (*) r#"[0-9]+"# ["-"]
    //   IntLit = (*) r#"[0-9]+"# ["/"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# [")"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["*"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["+"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["-"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["/"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# [")"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["*"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["-"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["/"]
    //   Term = (*) FloatLit [")"]
    //   Term = (*) FloatLit ["*"]
    //   Term = (*) FloatLit ["+"]
    //   Term = (*) FloatLit ["-"]
    //   Term = (*) FloatLit ["/"]
    //   Term = (*) Ident [")"]
    //   Term = (*) Ident ["*"]
    //   Term = (*) Ident ["+"]
    //   Term = (*) Ident ["-"]
    //   Term = (*) Ident ["/"]
    //   Term = (*) IntLit [")"]
    //   Term = (*) IntLit ["*"]
    //   Term = (*) IntLit ["+"]
    //   Term = (*) IntLit ["-"]
    //   Term = (*) IntLit ["/"]
    //   Term = (*) StringLit [")"]
    //   Term = (*) StringLit ["*"]
    //   Term = (*) StringLit ["+"]
    //   Term = (*) StringLit ["-"]
    //   Term = (*) StringLit ["/"]
    //   Term = (*) SymbolLit [")"]
    //   Term = (*) SymbolLit ["*"]
    //   Term = (*) SymbolLit ["+"]
    //   Term = (*) SymbolLit ["-"]
    //   Term = (*) SymbolLit ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = "(" (*) Expr ")" [")"]
    //   Term = "(" (*) Expr ")" ["*"]
    //   Term = "(" (*) Expr ")" ["+"]
    //   Term = "(" (*) Expr ")" ["-"]
    //   Term = "(" (*) Expr ")" ["/"]
    //
    //   "(" -> Shift(S27)
    //   r#"\"(\\\\.|[^\"])*\""# -> Shift(S28)
    //   r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S29)
    //   r#"[0-9]+"# -> Shift(S30)
    //   r#"[0-9]+\\.[0-9]*"# -> Shift(S31)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S32)
    //
    //   Expr -> S42
    //   Factor -> S20
    //   FloatLit -> S21
    //   Ident -> S22
    //   IntLit -> S23
    //   StringLit -> S24
    //   SymbolLit -> S25
    //   Term -> S26
    pub fn __state27<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state27(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state28(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state29(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state30(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state31(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state32(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state42(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state20(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::FloatLit(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state21(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Ident(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state22(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::IntLit(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state23(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::StringLit(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state24(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::SymbolLit(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state25(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state26(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 28
    //   StringLit = r#"\"(\\\\.|[^\"])*\""# (*) [")"]
    //   StringLit = r#"\"(\\\\.|[^\"])*\""# (*) ["*"]
    //   StringLit = r#"\"(\\\\.|[^\"])*\""# (*) ["+"]
    //   StringLit = r#"\"(\\\\.|[^\"])*\""# (*) ["-"]
    //   StringLit = r#"\"(\\\\.|[^\"])*\""# (*) ["/"]
    //
    //   ")" -> Reduce(StringLit = r#"\"(\\\\.|[^\"])*\""# => ActionFn(15);)
    //   "*" -> Reduce(StringLit = r#"\"(\\\\.|[^\"])*\""# => ActionFn(15);)
    //   "+" -> Reduce(StringLit = r#"\"(\\\\.|[^\"])*\""# => ActionFn(15);)
    //   "-" -> Reduce(StringLit = r#"\"(\\\\.|[^\"])*\""# => ActionFn(15);)
    //   "/" -> Reduce(StringLit = r#"\"(\\\\.|[^\"])*\""# => ActionFn(15);)
    //
    pub fn __state28<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action15(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::StringLit(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 29
    //   SymbolLit = r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# (*) [")"]
    //   SymbolLit = r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["*"]
    //   SymbolLit = r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["+"]
    //   SymbolLit = r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["-"]
    //   SymbolLit = r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["/"]
    //
    //   ")" -> Reduce(SymbolLit = r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(16);)
    //   "*" -> Reduce(SymbolLit = r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(16);)
    //   "+" -> Reduce(SymbolLit = r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(16);)
    //   "-" -> Reduce(SymbolLit = r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(16);)
    //   "/" -> Reduce(SymbolLit = r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(16);)
    //
    pub fn __state29<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action16(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::SymbolLit(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 30
    //   IntLit = r#"[0-9]+"# (*) [")"]
    //   IntLit = r#"[0-9]+"# (*) ["*"]
    //   IntLit = r#"[0-9]+"# (*) ["+"]
    //   IntLit = r#"[0-9]+"# (*) ["-"]
    //   IntLit = r#"[0-9]+"# (*) ["/"]
    //
    //   ")" -> Reduce(IntLit = r#"[0-9]+"# => ActionFn(13);)
    //   "*" -> Reduce(IntLit = r#"[0-9]+"# => ActionFn(13);)
    //   "+" -> Reduce(IntLit = r#"[0-9]+"# => ActionFn(13);)
    //   "-" -> Reduce(IntLit = r#"[0-9]+"# => ActionFn(13);)
    //   "/" -> Reduce(IntLit = r#"[0-9]+"# => ActionFn(13);)
    //
    pub fn __state30<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action13(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::IntLit(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 31
    //   FloatLit = r#"[0-9]+\\.[0-9]*"# (*) [")"]
    //   FloatLit = r#"[0-9]+\\.[0-9]*"# (*) ["*"]
    //   FloatLit = r#"[0-9]+\\.[0-9]*"# (*) ["+"]
    //   FloatLit = r#"[0-9]+\\.[0-9]*"# (*) ["-"]
    //   FloatLit = r#"[0-9]+\\.[0-9]*"# (*) ["/"]
    //
    //   ")" -> Reduce(FloatLit = r#"[0-9]+\\.[0-9]*"# => ActionFn(14);)
    //   "*" -> Reduce(FloatLit = r#"[0-9]+\\.[0-9]*"# => ActionFn(14);)
    //   "+" -> Reduce(FloatLit = r#"[0-9]+\\.[0-9]*"# => ActionFn(14);)
    //   "-" -> Reduce(FloatLit = r#"[0-9]+\\.[0-9]*"# => ActionFn(14);)
    //   "/" -> Reduce(FloatLit = r#"[0-9]+\\.[0-9]*"# => ActionFn(14);)
    //
    pub fn __state31<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action14(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::FloatLit(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 32
    //   Ident = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) [")"]
    //   Ident = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["*"]
    //   Ident = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["+"]
    //   Ident = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["-"]
    //   Ident = r#"[_a-zA-Z][_a-zA-Z0-9]*"# (*) ["/"]
    //
    //   ")" -> Reduce(Ident = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(17);)
    //   "*" -> Reduce(Ident = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(17);)
    //   "+" -> Reduce(Ident = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(17);)
    //   "-" -> Reduce(Ident = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(17);)
    //   "/" -> Reduce(Ident = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(17);)
    //
    pub fn __state32<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action17(input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Ident(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 33
    //   Expr = Expr "+" Factor (*) [EOF]
    //   Expr = Expr "+" Factor (*) ["+"]
    //   Expr = Expr "+" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [EOF]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [EOF]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   EOF -> Reduce(Expr = Expr, "+", Factor => ActionFn(1);)
    //   "*" -> Shift(S17)
    //   "+" -> Reduce(Expr = Expr, "+", Factor => ActionFn(1);)
    //   "-" -> Reduce(Expr = Expr, "+", Factor => ActionFn(1);)
    //   "/" -> Shift(S18)
    //
    pub fn __state33<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Ast<'input>>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Ast<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state17(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state18(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            None |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 34
    //   Expr = Expr "-" Factor (*) [EOF]
    //   Expr = Expr "-" Factor (*) ["+"]
    //   Expr = Expr "-" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [EOF]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [EOF]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   EOF -> Reduce(Expr = Expr, "-", Factor => ActionFn(2);)
    //   "*" -> Shift(S17)
    //   "+" -> Reduce(Expr = Expr, "-", Factor => ActionFn(2);)
    //   "-" -> Reduce(Expr = Expr, "-", Factor => ActionFn(2);)
    //   "/" -> Shift(S18)
    //
    pub fn __state34<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Ast<'input>>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Ast<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state17(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state18(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            None |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 35
    //   Factor = Factor "*" Term (*) [EOF]
    //   Factor = Factor "*" Term (*) ["*"]
    //   Factor = Factor "*" Term (*) ["+"]
    //   Factor = Factor "*" Term (*) ["-"]
    //   Factor = Factor "*" Term (*) ["/"]
    //
    //   EOF -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "*" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "+" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "-" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "/" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //
    pub fn __state35<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Ast<'input>>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Ast<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 36
    //   Factor = Factor "/" Term (*) [EOF]
    //   Factor = Factor "/" Term (*) ["*"]
    //   Factor = Factor "/" Term (*) ["+"]
    //   Factor = Factor "/" Term (*) ["-"]
    //   Factor = Factor "/" Term (*) ["/"]
    //
    //   EOF -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "*" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "+" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "-" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "/" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //
    pub fn __state36<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Ast<'input>>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Ast<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 37
    //   Term = "(" Expr ")" (*) [EOF]
    //   Term = "(" Expr ")" (*) ["*"]
    //   Term = "(" Expr ")" (*) ["+"]
    //   Term = "(" Expr ")" (*) ["-"]
    //   Term = "(" Expr ")" (*) ["/"]
    //
    //   EOF -> Reduce(Term = "(", Expr, ")" => ActionFn(12);)
    //   "*" -> Reduce(Term = "(", Expr, ")" => ActionFn(12);)
    //   "+" -> Reduce(Term = "(", Expr, ")" => ActionFn(12);)
    //   "-" -> Reduce(Term = "(", Expr, ")" => ActionFn(12);)
    //   "/" -> Reduce(Term = "(", Expr, ")" => ActionFn(12);)
    //
    pub fn __state37<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Ast<'input>>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action12(input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 38
    //   Expr = Expr "+" (*) Factor [")"]
    //   Expr = Expr "+" (*) Factor ["+"]
    //   Expr = Expr "+" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# [")"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["*"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["+"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["-"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["/"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [")"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["*"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["-"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["/"]
    //   IntLit = (*) r#"[0-9]+"# [")"]
    //   IntLit = (*) r#"[0-9]+"# ["*"]
    //   IntLit = (*) r#"[0-9]+"# ["+"]
    //   IntLit = (*) r#"[0-9]+"# ["-"]
    //   IntLit = (*) r#"[0-9]+"# ["/"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# [")"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["*"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["+"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["-"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["/"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# [")"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["*"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["-"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["/"]
    //   Term = (*) FloatLit [")"]
    //   Term = (*) FloatLit ["*"]
    //   Term = (*) FloatLit ["+"]
    //   Term = (*) FloatLit ["-"]
    //   Term = (*) FloatLit ["/"]
    //   Term = (*) Ident [")"]
    //   Term = (*) Ident ["*"]
    //   Term = (*) Ident ["+"]
    //   Term = (*) Ident ["-"]
    //   Term = (*) Ident ["/"]
    //   Term = (*) IntLit [")"]
    //   Term = (*) IntLit ["*"]
    //   Term = (*) IntLit ["+"]
    //   Term = (*) IntLit ["-"]
    //   Term = (*) IntLit ["/"]
    //   Term = (*) StringLit [")"]
    //   Term = (*) StringLit ["*"]
    //   Term = (*) StringLit ["+"]
    //   Term = (*) StringLit ["-"]
    //   Term = (*) StringLit ["/"]
    //   Term = (*) SymbolLit [")"]
    //   Term = (*) SymbolLit ["*"]
    //   Term = (*) SymbolLit ["+"]
    //   Term = (*) SymbolLit ["-"]
    //   Term = (*) SymbolLit ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //
    //   "(" -> Shift(S27)
    //   r#"\"(\\\\.|[^\"])*\""# -> Shift(S28)
    //   r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S29)
    //   r#"[0-9]+"# -> Shift(S30)
    //   r#"[0-9]+\\.[0-9]*"# -> Shift(S31)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S32)
    //
    //   Factor -> S43
    //   FloatLit -> S21
    //   Ident -> S22
    //   IntLit -> S23
    //   StringLit -> S24
    //   SymbolLit -> S25
    //   Term -> S26
    pub fn __state38<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Ast<'input>>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state27(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state28(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state29(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state30(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state31(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state32(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state43(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::FloatLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state21(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Ident(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state22(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::IntLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state23(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::StringLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state24(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::SymbolLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state25(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state26(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 39
    //   Expr = Expr "-" (*) Factor [")"]
    //   Expr = Expr "-" (*) Factor ["+"]
    //   Expr = Expr "-" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# [")"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["*"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["+"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["-"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["/"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [")"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["*"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["-"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["/"]
    //   IntLit = (*) r#"[0-9]+"# [")"]
    //   IntLit = (*) r#"[0-9]+"# ["*"]
    //   IntLit = (*) r#"[0-9]+"# ["+"]
    //   IntLit = (*) r#"[0-9]+"# ["-"]
    //   IntLit = (*) r#"[0-9]+"# ["/"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# [")"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["*"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["+"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["-"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["/"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# [")"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["*"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["-"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["/"]
    //   Term = (*) FloatLit [")"]
    //   Term = (*) FloatLit ["*"]
    //   Term = (*) FloatLit ["+"]
    //   Term = (*) FloatLit ["-"]
    //   Term = (*) FloatLit ["/"]
    //   Term = (*) Ident [")"]
    //   Term = (*) Ident ["*"]
    //   Term = (*) Ident ["+"]
    //   Term = (*) Ident ["-"]
    //   Term = (*) Ident ["/"]
    //   Term = (*) IntLit [")"]
    //   Term = (*) IntLit ["*"]
    //   Term = (*) IntLit ["+"]
    //   Term = (*) IntLit ["-"]
    //   Term = (*) IntLit ["/"]
    //   Term = (*) StringLit [")"]
    //   Term = (*) StringLit ["*"]
    //   Term = (*) StringLit ["+"]
    //   Term = (*) StringLit ["-"]
    //   Term = (*) StringLit ["/"]
    //   Term = (*) SymbolLit [")"]
    //   Term = (*) SymbolLit ["*"]
    //   Term = (*) SymbolLit ["+"]
    //   Term = (*) SymbolLit ["-"]
    //   Term = (*) SymbolLit ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //
    //   "(" -> Shift(S27)
    //   r#"\"(\\\\.|[^\"])*\""# -> Shift(S28)
    //   r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S29)
    //   r#"[0-9]+"# -> Shift(S30)
    //   r#"[0-9]+\\.[0-9]*"# -> Shift(S31)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S32)
    //
    //   Factor -> S44
    //   FloatLit -> S21
    //   Ident -> S22
    //   IntLit -> S23
    //   StringLit -> S24
    //   SymbolLit -> S25
    //   Term -> S26
    pub fn __state39<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Ast<'input>>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state27(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state28(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state29(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state30(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state31(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state32(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state44(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::FloatLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state21(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Ident(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state22(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::IntLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state23(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::StringLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state24(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::SymbolLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state25(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state26(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 40
    //   Factor = Factor "*" (*) Term [")"]
    //   Factor = Factor "*" (*) Term ["*"]
    //   Factor = Factor "*" (*) Term ["+"]
    //   Factor = Factor "*" (*) Term ["-"]
    //   Factor = Factor "*" (*) Term ["/"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# [")"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["*"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["+"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["-"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["/"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [")"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["*"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["-"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["/"]
    //   IntLit = (*) r#"[0-9]+"# [")"]
    //   IntLit = (*) r#"[0-9]+"# ["*"]
    //   IntLit = (*) r#"[0-9]+"# ["+"]
    //   IntLit = (*) r#"[0-9]+"# ["-"]
    //   IntLit = (*) r#"[0-9]+"# ["/"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# [")"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["*"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["+"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["-"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["/"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# [")"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["*"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["-"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["/"]
    //   Term = (*) FloatLit [")"]
    //   Term = (*) FloatLit ["*"]
    //   Term = (*) FloatLit ["+"]
    //   Term = (*) FloatLit ["-"]
    //   Term = (*) FloatLit ["/"]
    //   Term = (*) Ident [")"]
    //   Term = (*) Ident ["*"]
    //   Term = (*) Ident ["+"]
    //   Term = (*) Ident ["-"]
    //   Term = (*) Ident ["/"]
    //   Term = (*) IntLit [")"]
    //   Term = (*) IntLit ["*"]
    //   Term = (*) IntLit ["+"]
    //   Term = (*) IntLit ["-"]
    //   Term = (*) IntLit ["/"]
    //   Term = (*) StringLit [")"]
    //   Term = (*) StringLit ["*"]
    //   Term = (*) StringLit ["+"]
    //   Term = (*) StringLit ["-"]
    //   Term = (*) StringLit ["/"]
    //   Term = (*) SymbolLit [")"]
    //   Term = (*) SymbolLit ["*"]
    //   Term = (*) SymbolLit ["+"]
    //   Term = (*) SymbolLit ["-"]
    //   Term = (*) SymbolLit ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //
    //   "(" -> Shift(S27)
    //   r#"\"(\\\\.|[^\"])*\""# -> Shift(S28)
    //   r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S29)
    //   r#"[0-9]+"# -> Shift(S30)
    //   r#"[0-9]+\\.[0-9]*"# -> Shift(S31)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S32)
    //
    //   FloatLit -> S21
    //   Ident -> S22
    //   IntLit -> S23
    //   StringLit -> S24
    //   SymbolLit -> S25
    //   Term -> S45
    pub fn __state40<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Ast<'input>>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state27(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state28(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state29(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state30(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state31(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state32(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::FloatLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state21(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Ident(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state22(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::IntLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state23(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::StringLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state24(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::SymbolLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state25(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state45(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 41
    //   Factor = Factor "/" (*) Term [")"]
    //   Factor = Factor "/" (*) Term ["*"]
    //   Factor = Factor "/" (*) Term ["+"]
    //   Factor = Factor "/" (*) Term ["-"]
    //   Factor = Factor "/" (*) Term ["/"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# [")"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["*"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["+"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["-"]
    //   FloatLit = (*) r#"[0-9]+\\.[0-9]*"# ["/"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# [")"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["*"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["-"]
    //   Ident = (*) r#"[_a-zA-Z][_a-zA-Z0-9]*"# ["/"]
    //   IntLit = (*) r#"[0-9]+"# [")"]
    //   IntLit = (*) r#"[0-9]+"# ["*"]
    //   IntLit = (*) r#"[0-9]+"# ["+"]
    //   IntLit = (*) r#"[0-9]+"# ["-"]
    //   IntLit = (*) r#"[0-9]+"# ["/"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# [")"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["*"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["+"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["-"]
    //   StringLit = (*) r#"\"(\\\\.|[^\"])*\""# ["/"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# [")"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["*"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["+"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["-"]
    //   SymbolLit = (*) r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# ["/"]
    //   Term = (*) FloatLit [")"]
    //   Term = (*) FloatLit ["*"]
    //   Term = (*) FloatLit ["+"]
    //   Term = (*) FloatLit ["-"]
    //   Term = (*) FloatLit ["/"]
    //   Term = (*) Ident [")"]
    //   Term = (*) Ident ["*"]
    //   Term = (*) Ident ["+"]
    //   Term = (*) Ident ["-"]
    //   Term = (*) Ident ["/"]
    //   Term = (*) IntLit [")"]
    //   Term = (*) IntLit ["*"]
    //   Term = (*) IntLit ["+"]
    //   Term = (*) IntLit ["-"]
    //   Term = (*) IntLit ["/"]
    //   Term = (*) StringLit [")"]
    //   Term = (*) StringLit ["*"]
    //   Term = (*) StringLit ["+"]
    //   Term = (*) StringLit ["-"]
    //   Term = (*) StringLit ["/"]
    //   Term = (*) SymbolLit [")"]
    //   Term = (*) SymbolLit ["*"]
    //   Term = (*) SymbolLit ["+"]
    //   Term = (*) SymbolLit ["-"]
    //   Term = (*) SymbolLit ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //
    //   "(" -> Shift(S27)
    //   r#"\"(\\\\.|[^\"])*\""# -> Shift(S28)
    //   r#"\'[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S29)
    //   r#"[0-9]+"# -> Shift(S30)
    //   r#"[0-9]+\\.[0-9]*"# -> Shift(S31)
    //   r#"[_a-zA-Z][_a-zA-Z0-9]*"# -> Shift(S32)
    //
    //   FloatLit -> S21
    //   Ident -> S22
    //   IntLit -> S23
    //   StringLit -> S24
    //   SymbolLit -> S25
    //   Term -> S46
    pub fn __state41<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Ast<'input>>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state27(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state28(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state29(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state30(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state31(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state32(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::FloatLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state21(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Ident(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state22(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::IntLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state23(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::StringLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state24(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::SymbolLit(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state25(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state46(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 42
    //   Expr = Expr (*) "+" Factor [")"]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [")"]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   Term = "(" Expr (*) ")" [")"]
    //   Term = "(" Expr (*) ")" ["*"]
    //   Term = "(" Expr (*) ")" ["+"]
    //   Term = "(" Expr (*) ")" ["-"]
    //   Term = "(" Expr (*) ")" ["/"]
    //
    //   ")" -> Shift(S47)
    //   "+" -> Shift(S38)
    //   "-" -> Shift(S39)
    //
    pub fn __state42<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Ast<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state47(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state38(input, __lookbehind, __tokens, __sym1, __sym2));
            }
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state39(input, __lookbehind, __tokens, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 43
    //   Expr = Expr "+" Factor (*) [")"]
    //   Expr = Expr "+" Factor (*) ["+"]
    //   Expr = Expr "+" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   ")" -> Reduce(Expr = Expr, "+", Factor => ActionFn(1);)
    //   "*" -> Shift(S40)
    //   "+" -> Reduce(Expr = Expr, "+", Factor => ActionFn(1);)
    //   "-" -> Reduce(Expr = Expr, "+", Factor => ActionFn(1);)
    //   "/" -> Shift(S41)
    //
    pub fn __state43<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Ast<'input>>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Ast<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state40(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state41(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 44
    //   Expr = Expr "-" Factor (*) [")"]
    //   Expr = Expr "-" Factor (*) ["+"]
    //   Expr = Expr "-" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   ")" -> Reduce(Expr = Expr, "-", Factor => ActionFn(2);)
    //   "*" -> Shift(S40)
    //   "+" -> Reduce(Expr = Expr, "-", Factor => ActionFn(2);)
    //   "-" -> Reduce(Expr = Expr, "-", Factor => ActionFn(2);)
    //   "/" -> Shift(S41)
    //
    pub fn __state44<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Ast<'input>>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Ast<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state40(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state41(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 45
    //   Factor = Factor "*" Term (*) [")"]
    //   Factor = Factor "*" Term (*) ["*"]
    //   Factor = Factor "*" Term (*) ["+"]
    //   Factor = Factor "*" Term (*) ["-"]
    //   Factor = Factor "*" Term (*) ["/"]
    //
    //   ")" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "*" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "+" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "-" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //   "/" -> Reduce(Factor = Factor, "*", Term => ActionFn(4);)
    //
    pub fn __state45<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Ast<'input>>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Ast<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 46
    //   Factor = Factor "/" Term (*) [")"]
    //   Factor = Factor "/" Term (*) ["*"]
    //   Factor = Factor "/" Term (*) ["+"]
    //   Factor = Factor "/" Term (*) ["-"]
    //   Factor = Factor "/" Term (*) ["/"]
    //
    //   ")" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "*" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "+" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "-" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //   "/" -> Reduce(Factor = Factor, "/", Term => ActionFn(5);)
    //
    pub fn __state46<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Ast<'input>>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<Ast<'input>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 47
    //   Term = "(" Expr ")" (*) [")"]
    //   Term = "(" Expr ")" (*) ["*"]
    //   Term = "(" Expr ")" (*) ["+"]
    //   Term = "(" Expr ")" (*) ["-"]
    //   Term = "(" Expr ")" (*) ["/"]
    //
    //   ")" -> Reduce(Term = "(", Expr, ")" => ActionFn(12);)
    //   "*" -> Reduce(Term = "(", Expr, ")" => ActionFn(12);)
    //   "+" -> Reduce(Term = "(", Expr, ")" => ActionFn(12);)
    //   "-" -> Reduce(Term = "(", Expr, ")" => ActionFn(12);)
    //   "/" -> Reduce(Term = "(", Expr, ")" => ActionFn(12);)
    //
    pub fn __state47<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<Ast<'input>>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action12(input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__Expr::parse_Expr;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '\"' => {
                            __current_state = 1;
                            continue;
                        }
                        '\'' => {
                            __current_state = 2;
                            continue;
                        }
                        '(' => {
                            __current_match = Some((0, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        ')' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        '*' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '+' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '-' => {
                            __current_match = Some((4, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '/' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        '0' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '\"' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        '\\' => {
                            __current_state = 13;
                            continue;
                        }
                        _ => {
                            __current_state = 14;
                            continue;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        'A' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        's' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        't' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '.' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        '0' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        's' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        't' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '\"' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        '\\' => {
                            __current_state = 13;
                            continue;
                        }
                        _ => {
                            __current_state = 14;
                            continue;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '\"' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        '\\' => {
                            __current_state = 13;
                            continue;
                        }
                        _ => {
                            __current_state = 14;
                            continue;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        's' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        't' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '\"' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        '\\' => {
                            __current_state = 13;
                            continue;
                        }
                        _ => {
                            __current_state = 14;
                            continue;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

pub fn __action0<
    'input,
>(
    input: &'input str,
    __0: Ast<'input>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Ast<'input>
{
    (__0)
}

pub fn __action1<
    'input,
>(
    input: &'input str,
    l: Ast<'input>,
    _: &'input str,
    r: Ast<'input>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Ast<'input>
{
    Ast::Add(Box::new(l), Box::new(r))
}

pub fn __action2<
    'input,
>(
    input: &'input str,
    l: Ast<'input>,
    _: &'input str,
    r: Ast<'input>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Ast<'input>
{
    Ast::Sub(Box::new(l), Box::new(r))
}

pub fn __action3<
    'input,
>(
    input: &'input str,
    __0: Ast<'input>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Ast<'input>
{
    (__0)
}

pub fn __action4<
    'input,
>(
    input: &'input str,
    l: Ast<'input>,
    _: &'input str,
    r: Ast<'input>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Ast<'input>
{
    Ast::Mul(Box::new(l), Box::new(r))
}

pub fn __action5<
    'input,
>(
    input: &'input str,
    l: Ast<'input>,
    _: &'input str,
    r: Ast<'input>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Ast<'input>
{
    Ast::Div(Box::new(l), Box::new(r))
}

pub fn __action6<
    'input,
>(
    input: &'input str,
    __0: Ast<'input>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Ast<'input>
{
    (__0)
}

pub fn __action7<
    'input,
>(
    input: &'input str,
    __0: Ast<'input>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Ast<'input>
{
    (__0)
}

pub fn __action8<
    'input,
>(
    input: &'input str,
    __0: Ast<'input>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Ast<'input>
{
    (__0)
}

pub fn __action9<
    'input,
>(
    input: &'input str,
    __0: Ast<'input>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Ast<'input>
{
    (__0)
}

pub fn __action10<
    'input,
>(
    input: &'input str,
    __0: Ast<'input>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Ast<'input>
{
    (__0)
}

pub fn __action11<
    'input,
>(
    input: &'input str,
    __0: Ident<'input>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Ast<'input>
{
    Ast::Identifier(__0)
}

pub fn __action12<
    'input,
>(
    input: &'input str,
    _: &'input str,
    __0: Ast<'input>,
    _: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Ast<'input>
{
    (__0)
}

pub fn __action13<
    'input,
>(
    input: &'input str,
    __0: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Ast<'input>
{
    Ast::IntLit(i64::from_str(__0).unwrap())
}

pub fn __action14<
    'input,
>(
    input: &'input str,
    __0: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Ast<'input>
{
    Ast::FloatLit(f64::from_str(__0).unwrap())
}

pub fn __action15<
    'input,
>(
    input: &'input str,
    __0: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Ast<'input>
{
    Ast::StringLit(&__0[1 .. __0.len() - 1])
}

pub fn __action16<
    'input,
>(
    input: &'input str,
    __0: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Ast<'input>
{
    Ast::SymbolLit(&__0[1..])
}

pub fn __action17<
    'input,
>(
    input: &'input str,
    __0: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Ident<'input>
{
    Ident(__0)
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
