use super::{Bound, SymbolBindSource};
use compiler::parse::Ast;
use ares_syntax::SymbolIntern;
use compiler::parse::test::ok_parse_1;
use typed_arena::Arena;
use itertools::Itertools;

fn format<'a, F>(bound: &'a Bound<'a, 'a>, level: u32, interner: &SymbolIntern, f: &mut F) -> Result<(), ::std::fmt::Error>
where F: ::std::fmt::Write {
    use super::Bound::*;
    fn gen_indent(level: u32) -> String {
        let mut buf = String::new();
        for _ in 0 .. (level * 4) {
            buf.push(' ');
        }
        buf
    }
    fn label<F: ::std::fmt::Write>(name: &str, level: u32, f: &mut F) -> Result<(), ::std::fmt::Error>{
        try!(f.write_str(&gen_indent(level)));
        try!(f.write_str(name));
        f.write_str(":\n")
    }
    fn print_source<F: ::std::fmt::Write>(
        source: &SymbolBindSource,
        level: u32,
        interner: &SymbolIntern,
        f: &mut F) -> Result<(), ::std::fmt::Error> {

        match *source {
            SymbolBindSource::Arg(a) => {
                try!(label("ARG", level, f));
                try!(f.write_str(&gen_indent(level + 1)));
                try!(f.write_str(&a.to_string()));
            }
            SymbolBindSource::Upvar(a) => {
                try!(label("UPVAR", level, f));
                try!(f.write_str(&gen_indent(level + 1)));
                try!(f.write_str(&a.to_string()));
            }
            SymbolBindSource::LocalDefine(a) => {
                try!(label("LOCAL-DEFINE", level, f));
                try!(f.write_str(&gen_indent(level + 1)));
                try!(f.write_str(&a.to_string()));
            }
            SymbolBindSource::Global(a) => {
                try!(label("GLOBAL", level, f));
                try!(f.write_str(&gen_indent(level)));
                try!(f.write_str(&interner.lookup_or_anon(a)));
            }
        }
        f.write_str("\n")
    }

    match bound {
        &Literal(l) => {
            try!(label("LITERAL", level, f));
            try!(f.write_str(&gen_indent(level + 1)));
            match l {
                &Ast::BoolLit(b, _)  => f.write_str(&format!("{}\n", b)),
                &Ast::StringLit(ref s, _)  => f.write_str(&format!("\"{}\"\n", s)),
                &Ast::FloatLit(fl, _)  => f.write_str(&format!("{}\n", fl)),
                &Ast::SymbolLit(s, _)  => f.write_str(&format!("'{}\n", interner.lookup_or_anon(s))),
                &Ast::IntLit(i, _)  => f.write_str(&format!("{}\n", i)),
                _ => panic!("non-literal found in Bound::Literal")
            }
        }
        &Symbol { symbol, ref source, .. } => {
            try!(label("SYMBOL", level, f));

            try!(label("NAME", level + 1, f));
            try!(f.write_str(&format!("{}{}\n", &gen_indent(level + 2), interner.lookup_or_anon(symbol))));

            try!(label("SOURCE", level + 1, f));
            try!(print_source(source, level + 2, interner, f));

            Ok(())
        }
        &ListLit(ref list, _) => {
            try!(label("LIST", level, f));
            for child in list {
                try!(format(child, level + 1, interner, f));
            }
            Ok(())
        },
        &MapLit(ref pairs, _) => {
            try!(label("MAP", level, f));
            for &(ref k, ref v) in pairs {
                try!(label("MAP-PAIR", level + 1, f));
                try!(format(k, level + 2, interner, f));
                try!(format(v, level + 2, interner, f));
            }
            Ok(())
        }
        &Add(ref l, ref r, _) => {
            try!(label("ADD", level, f));
            try!(format(l, level + 1, interner, f));
            try!(format(r, level + 1, interner, f));
            Ok(())
        },
        &Sub(ref l, ref r, _) => {
            try!(label("SUB", level, f));
            try!(format(l, level + 1, interner, f));
            try!(format(r, level + 1, interner, f));
            Ok(())
        },
        &Mul(ref l, ref r, _) => {
            try!(label("MUL", level, f));
            try!(format(l, level + 1, interner, f));
            try!(format(r, level + 1, interner, f));
            Ok(())
        },
        &Div(ref l, ref r, _) => {
            try!(label("DIV", level, f));
            try!(format(l, level + 1, interner, f));
            try!(format(r, level + 1, interner, f));
            Ok(())
        },
        &FnCall(ref rec, ref args, _) => {
            try!(label("FN-CALL", level, f));

            try!(label("RECEIVER", level + 1, f));
            try!(format(rec, level + 2, interner, f));

            try!(label("ARGS", level + 1, f));
            for arg in args {
                try!(format(arg, level + 2, interner, f));
            }
            Ok(())
        }
        &IfExpression(ref cond, ref tru, ref fals, _) => {
            try!(label("IF-EXPRESSION", level, f));

            try!(label("COND", level + 1, f));
            try!(format(cond, level + 2, interner, f));

            try!(label("TRUE", level + 1, f));
            try!(format(tru, level+2, interner, f));

            try!(label("FALSE", level + 1, f));
            try!(format(fals, level+2, interner, f));
            Ok(())
        }
        &IfStatement(ref cond, ref tru, ref fals, _) => {
            try!(label("IF-STATEMENT", level, f));

            try!(label("COND", level + 1, f));
            try!(format(cond, level + 2, interner, f));

            try!(label("TRUE", level + 1, f));
            for statement in tru {
                try!(format(statement, level+2, interner, f));
            }

            if let Some(fals) = fals.as_ref() {
                try!(label("FALSE", level + 1, f));
                for statement in fals {
                    try!(format(statement, level+2, interner, f));
                }
            }
            Ok(())
        }
        &Lambda{ ref arg_symbols, ref body, ref bindings, ..} => {
            let (_, _, _) = (arg_symbols, body, bindings);
            try!(label("LAMBDA", level, f));

            try!(label("NUM-ARGS", level+1, f));
            try!(f.write_str(&format!("{}{}\n", &gen_indent(level + 2), bindings.num_args)));

            try!(label("NUM-UPVARS", level+1, f));
            try!(f.write_str(&format!("{}{}\n", &gen_indent(level + 2), bindings.num_upvars)));

            try!(label("NUM-DECLARATIONS", level+1, f));
            try!(f.write_str(&format!("{}{}\n", &gen_indent(level + 2), bindings.num_declarations)));

            try!(label("ARGS", level+1, f));
            for &arg in arg_symbols {
                try!(f.write_str(&format!(
                            "{}{}\n",
                            &gen_indent(level + 2),
                            interner.lookup_or_anon(arg))));
            }

            try!(label("BODY", level+1, f));
            try!(format(body, level + 2, interner, f));

            try!(label("BINDINGS", level + 1, f));

            let bindings = bindings.bindings.iter().map(|(a, b)| (b, a)).sorted();

            for (source, symbol) in bindings {
                try!(label("BINDING", level + 2, f));

                try!(label("SYMBOL", level + 3, f));
                try!(f.write_str(&format!(
                            "{}{}\n",
                            &gen_indent(level + 4),
                            interner.lookup_or_anon(*symbol))));

                try!(label("SOURCE", level + 3, f));
                try!(print_source(source, level + 4, interner, f));
            }

            Ok(())
        }
        &Block(ref bodies, _) => {
            try!(label("BLOCK", level, f));
            for body in bodies {
                try!(format(body, level + 1, interner, f));
            }
            Ok(())
        }
        &Define(ref name, ref source, ref value, _) => {
            try!(label("DEFINE", level, f));

            try!(label("NAME", level + 1, f));
            try!(f.write_str(&gen_indent(level + 2)));
            try!(f.write_str(&interner.lookup_or_anon(*name)));
            try!(f.write_str("\n"));

            try!(label("SOURCE", level + 1, f));
            try!(print_source(source, level + 2, interner, f));

            try!(label("VALUE", level + 1, f));
            try!(format(value, level + 1, interner, f));
            Ok(())
        }
    }
    
}

fn str_eq(actual: &str, expected: &str) {
    use itertools::{Itertools, EitherOrBoth};
    fn isnt_whitespace(s: &&str) -> bool { !s.chars().all(char::is_whitespace) }
    let mut padding = None;
    for eob in actual.lines().filter(isnt_whitespace).zip_longest(
               expected.lines().filter(isnt_whitespace)) {
        match eob {
            EitherOrBoth::Both(l, r) => {
                let w_left = (l.len() - l.trim_left().len()) as i32;
                let w_right = (r.len() - r.trim_left().len()) as i32;
                let d = w_left - w_right;

                if padding.is_none() {
                    padding = Some(d);
                } else {
                    assert!(padding.unwrap() == d,
                            "indentation isn't the same at {} {}",
                            l, r);
                }
                if l.trim() != r.trim() {
                    println!("actual:\n{}\n=====\nexpected:\n{}", actual, expected);
                }
                assert_eq!(l.trim(), r.trim());
            }
            EitherOrBoth::Left(l) => {
                println!("actual has more lines: {}", l);
                println!("actual:\n{}\n=====\nexpected:\n{}", actual, expected);
                panic!();
            }
            EitherOrBoth::Right(l) => {
                println!("expected has more lines: {}", l);
                println!("actual:\n{}\n=====\nexpected:\n{}", actual, expected);
                panic!();
            }
        }
    }
}

fn bound_form(program: &str, bound_representation: &str) {
    let bind_arena = Arena::new();
    let (ast, mut interner) = ok_parse_1(program);
    let bound = Bound::bind_top(ast, &bind_arena, &mut interner).unwrap();

    let mut buffer = String::with_capacity(bound_representation.len());
    format(bound, 0, &interner, &mut buffer).unwrap();

    str_eq(&buffer, bound_representation)
}

#[test]
fn literals() {
    bound_form("1",
    r#"
        LITERAL:
            1
    "#);

    bound_form("1.2",
    r#"
        LITERAL:
            1.2
    "#);

    bound_form("\"hi\"",
    r#"
        LITERAL:
            "hi"
    "#);

    bound_form("true",
    r#"
        LITERAL:
            true
    "#);

    bound_form("false",
    r#"
        LITERAL:
            false
    "#);

    bound_form("'a",
    r#"
        LITERAL:
            'a
    "#);
}

#[test]
fn operators() {
    bound_form("1 + 2",
    r#"
        ADD:
            LITERAL:
                1
            LITERAL:
                2
    "#);

    bound_form("1 - 2",
    r#"
        SUB:
            LITERAL:
                1
            LITERAL:
                2
    "#);
}

#[test]
fn bind_lambda_one_arg() {
    bound_form("fn(a) { a }",
    r#"
        LAMBDA:
            NUM-ARGS:
                1
            NUM-UPVARS:
                0
            NUM-DECLARATIONS:
                0
            ARGS:
                a
            BODY:
                BLOCK:
                    SYMBOL:
                        NAME:
                            a
                        SOURCE:
                            ARG:
                                0
            BINDINGS:
                BINDING:
                    SYMBOL:
                        a
                    SOURCE:
                        ARG:
                            0
    "#);
}

#[test]
fn bind_lambda_two_args() {
    bound_form("fn(a, b) { a + b }",
    r#"
        LAMBDA:
            NUM-ARGS:
                2
            NUM-UPVARS:
                0
            NUM-DECLARATIONS:
                0
            ARGS:
                a
                b
            BODY:
                BLOCK:
                    ADD:
                        SYMBOL:
                            NAME:
                                a
                            SOURCE:
                                ARG:
                                    0
                        SYMBOL:
                            NAME:
                                b
                            SOURCE:
                                ARG:
                                    1
            BINDINGS:
                BINDING:
                    SYMBOL:
                        a
                    SOURCE:
                        ARG:
                            0
                BINDING:
                    SYMBOL:
                        b
                    SOURCE:
                        ARG:
                            1
    "#);
}
