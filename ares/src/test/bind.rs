use compiler::binding::{Bound, SymbolBindSource};
use compiler::parse::Ast;
use ares_syntax::SymbolIntern;
use typed_arena::Arena;
use itertools::Itertools;
use vm::Modules;
use super::{do_binding, TestResult};

fn format<'a, F>(bound: &'a Bound<'a, 'a>, level: u32, interner: &SymbolIntern, f: &mut F) -> Result<(), ::std::fmt::Error>
where F: ::std::fmt::Write {
    use ::compiler::binding::Bound::*;
    fn gen_indent(level: u32) -> String {
        let mut buf = String::new();
        for _ in 0 .. (level * 4) {
            buf.push(' ');
        }
        buf
    }

    fn label<F: ::std::fmt::Write>(name: &str, level: u32, f: &mut F) -> Result<(), ::std::fmt::Error>{
        f.write_str(&gen_indent(level))?;
        f.write_str(name)?;
        f.write_str(":\n")
    }

    fn print_source<F: ::std::fmt::Write>(
        source: &SymbolBindSource,
        level: u32,
        interner: &SymbolIntern,
        f: &mut F) -> Result<(), ::std::fmt::Error> {

        match *source {
            SymbolBindSource::Arg{position, ref upvar} => {
                label("ARG", level, f)?;
                f.write_str(&gen_indent(level + 1))?;
                f.write_str(&position.to_string())?;
                if upvar.get() {
                    f.write_str("\n")?;
                    f.write_str(&gen_indent(level + 1))?;
                    f.write_str("Is an upvar")?;
                }
            }
            SymbolBindSource::Upvar{position, ref upvar} => {
                label("UPVAR", level, f)?;
                f.write_str(&gen_indent(level + 1))?;
                f.write_str(&position.to_string())?;
                if upvar.get() {
                    f.write_str("\n")?;
                    f.write_str(&gen_indent(level + 1))?;
                    f.write_str("Is an upvar")?;
                }
            }
            SymbolBindSource::LocalDefine{position, ref upvar} => {
                label("LOCAL-DEFINE", level, f)?;
                f.write_str(&gen_indent(level + 1))?;
                f.write_str(&position.to_string())?;
                if upvar.get() {
                    f.write_str("\n")?;
                    f.write_str(&gen_indent(level + 1))?;
                    f.write_str("Is an upvar")?;
                }
            }
            SymbolBindSource::Global(a) => {
                label("GLOBAL", level, f)?;
                f.write_str(&gen_indent(level + 1))?;
                f.write_str(&interner.lookup_or_anon(a))?;
            }
        }
        f.write_str("\n")
    }

    match bound {
        &Literal(l) => {
            label("LITERAL", level, f)?;
            f.write_str(&gen_indent(level + 1))?;
            match l {
                &Ast::BoolLit(b, _)  => f.write_str(&format!("{}\n", b)),
                &Ast::StringLit(ref s, _)  => f.write_str(&format!("\"{}\"\n", s)),
                &Ast::FloatLit(fl, _)  => f.write_str(&format!("{}\n", fl)),
                &Ast::SymbolLit(s, _)  => f.write_str(&format!("'{}\n", interner.lookup_or_anon(s))),
                &Ast::IntLit(i, _)  => f.write_str(&format!("{}\n", i)),
                &Ast::NilLit(_)  => f.write_str(&format!("nil\n")),
                _ => panic!("non-literal found in Bound::Literal")
            }
        }
        &Symbol { symbol, ref source, .. } => {
            label("SYMBOL", level, f)?;

            label("NAME", level + 1, f)?;
            f.write_str(&format!("{}{}\n", &gen_indent(level + 2), interner.lookup_or_anon(symbol)))?;

            label("SOURCE", level + 1, f)?;
            print_source(source, level + 2, interner, f)?;

            Ok(())
        }
        &ListLit(ref list, _) => {
            label("LIST", level, f)?;
            for child in list {
                format(child, level + 1, interner, f)?;
            }
            Ok(())
        },
        &MapLit(ref pairs, _) => {
            label("MAP", level, f)?;
            for &(ref k, ref v) in pairs {
                label("MAP-PAIR", level + 1, f)?;
                format(k, level + 2, interner, f)?;
                format(v, level + 2, interner, f)?;
            }
            Ok(())
        }
        &Add(ref l, ref r, _) => {
            label("ADD", level, f)?;
            format(l, level + 1, interner, f)?;
            format(r, level + 1, interner, f)?;
            Ok(())
        },
        &Sub(ref l, ref r, _) => {
            label("SUB", level, f)?;
            format(l, level + 1, interner, f)?;
            format(r, level + 1, interner, f)?;
            Ok(())
        },
        &Mul(ref l, ref r, _) => {
            label("MUL", level, f)?;
            format(l, level + 1, interner, f)?;
            format(r, level + 1, interner, f)?;
            Ok(())
        },
        &Div(ref l, ref r, _) => {
            label("DIV", level, f)?;
            format(l, level + 1, interner, f)?;
            format(r, level + 1, interner, f)?;
            Ok(())
        },
        &LessThan(ref l, ref r, _) => {
            label("LESS-THAN", level, f)?;
            format(l, level + 1, interner, f)?;
            format(r, level + 1, interner, f)?;
            Ok(())
        },
        &LessThanOrEqual(ref l, ref r, _) => {
            label("LESS-THAN-OR-EQUAL", level, f)?;
            format(l, level + 1, interner, f)?;
            format(r, level + 1, interner, f)?;
            Ok(())
        },
        &GreaterThan(ref l, ref r, _) => {
            label("GREATER-THAN", level, f)?;
            format(l, level + 1, interner, f)?;
            format(r, level + 1, interner, f)?;
            Ok(())
        },
        &GreaterThanOrEqual(ref l, ref r, _) => {
            label("GREATER-THAN-OR-EQUAL", level, f)?;
            format(l, level + 1, interner, f)?;
            format(r, level + 1, interner, f)?;
            Ok(())
        },
        &Equal(ref l, ref r, _) => {
            label("EQUAL", level, f)?;
            format(l, level + 1, interner, f)?;
            format(r, level + 1, interner, f)?;
            Ok(())
        },
        &NotEqual(ref l, ref r, _) => {
            label("NotEqual", level, f)?;
            format(l, level + 1, interner, f)?;
            format(r, level + 1, interner, f)?;
            Ok(())
        },
        &FnCall(ref rec, ref args, _) => {
            label("FN-CALL", level, f)?;

            label("RECEIVER", level + 1, f)?;
            format(rec, level + 2, interner, f)?;

            label("ARGS", level + 1, f)?;
            for arg in args {
                format(arg, level + 2, interner, f)?;
            }
            Ok(())
        }
        &IfExpression(ref cond, ref tru, ref fals, _) => {
            label("IF-EXPRESSION", level, f)?;

            label("COND", level + 1, f)?;
            format(cond, level + 2, interner, f)?;

            label("TRUE", level + 1, f)?;
            format(tru, level+2, interner, f)?;

            label("FALSE", level + 1, f)?;
            format(fals, level+2, interner, f)?;
            Ok(())
        }
        &IfStatement(ref cond, ref tru, ref fals, _) => {
            label("IF-STATEMENT", level, f)?;

            label("COND", level + 1, f)?;
            format(cond, level + 2, interner, f)?;

            label("TRUE", level + 1, f)?;
            format(tru, level+2, interner, f)?;

            if let Some(fals) = fals.as_ref() {
                label("FALSE", level + 1, f)?;
                format(fals, level+2, interner, f)?;
            }
            Ok(())
        }
        &Lambda{ ref arg_symbols, ref body, ref bindings, ref upvar_list, ..} => {
            let (_, _, _) = (arg_symbols, body, bindings);
            label("LAMBDA", level, f)?;

            label("NUM-ARGS", level+1, f)?;
            f.write_str(&format!("{}{}\n", &gen_indent(level + 2), bindings.num_args))?;

            label("NUM-UPVARS", level+1, f)?;
            f.write_str(&format!("{}{}\n", &gen_indent(level + 2), bindings.num_upvars))?;

            label("NUM-DECLARATIONS", level+1, f)?;
            f.write_str(&format!("{}{}\n", &gen_indent(level + 2), bindings.num_declarations))?;

            label("ARGS", level+1, f)?;
            for &arg in arg_symbols {
                f.write_str(&format!(
                            "{}{}\n",
                            &gen_indent(level + 2),
                            interner.lookup_or_anon(arg)))?;
            }

            label("BODY", level+1, f)?;
            format(body, level + 2, interner, f)?;

            label("BINDINGS", level + 1, f)?;

            let bindings = bindings.bindings.iter().map(|(a, b)| (b, a)).sorted();

            for (source, symbol) in bindings {
                label("BINDING", level + 2, f)?;

                label("SYMBOL", level + 3, f)?;
                f.write_str(&format!(
                            "{}{}\n",
                            &gen_indent(level + 4),
                            interner.lookup_or_anon(*symbol)))?;

                label("SOURCE", level + 3, f)?;
                print_source(source, level + 4, interner, f)?;
            }

            if upvar_list.len() > 0 {
                label("UPVAR-LIST", level + 1, f)?;
                for (i, source) in upvar_list.iter().enumerate() {
                    f.write_str(&format!("{}{}\n", gen_indent(level + 2), i))?;
                    print_source(source, level + 3, interner, f)?;
                }
            }

            Ok(())
        }
        &BlockExpression(ref bodies, _) => {
            label("BLOCK-EXPRESSION", level, f)?;
            for body in bodies {
                format(body, level + 1, interner, f)?;
            }
            Ok(())
        }
        &ListAccess(ref list, ref idx, _) => {
            label("LIST-ACCESS", level, f)?;

            label("LIST", level + 1, f)?;
                format(list, level + 2, interner, f)?;

            label("INDEX", level + 1, f)?;
                format(idx, level + 2, interner, f)?;
            Ok(())
        }
        &BlockStatement(ref bodies, _) => {
            label("BLOCK-STATEMENT", level, f)?;
            for body in bodies {
                format(body, level + 1, interner, f)?;
            }
            Ok(())
        }
        &Assign(ref name, ref source, ref value, _) => {
            label("ASSIGN", level, f)?;

            label("NAME", level + 1, f)?;
            f.write_str(&gen_indent(level + 2))?;
            f.write_str(&interner.lookup_or_anon(*name))?;
            f.write_str("\n")?;

            label("SOURCE", level + 1, f)?;
            print_source(source, level + 2, interner, f)?;

            label("VALUE", level + 1, f)?;
            format(value, level + 1, interner, f)?;
            Ok(())
        }
        &Define(ref name, ref source, ref value, _) => {
            label("DEFINE", level, f)?;

            label("NAME", level + 1, f)?;
            f.write_str(&gen_indent(level + 2))?;
            f.write_str(&interner.lookup_or_anon(*name))?;
            f.write_str("\n")?;

            label("SOURCE", level + 1, f)?;
            print_source(source, level + 2, interner, f)?;

            label("VALUE", level + 1, f)?;
            format(value, level + 2, interner, f)?;
            Ok(())
        }
        &Shift(ref symbols, ref lambda, _) => {
            label("SHIFT", level, f)?;

            label("SYMBOLS", level + 1, f)?;
            for &symbol in symbols {
                format(symbol, level + 2, interner, f)?;
            }

            label("LAMBDA", level + 1, f)?;
            format(lambda, level + 2, interner, f)?;
            Ok(())
        }
        &Reset(ref symbols, ref lambda, _) => {
            label("RESET", level, f)?;

            label("SYMBOLS", level + 1, f)?;
            for &symbol in symbols {
                format(symbol, level + 2, interner, f)?;
            }

            label("LAMBDA", level + 1, f)?;
            format(lambda, level + 2, interner, f)?;
            Ok(())
        }
    }
    
}

fn str_eq(actual: &str, expected: &str) -> Result<(), String> {
    use itertools::{Itertools, EitherOrBoth};
    fn isnt_whitespace(s: &&str) -> bool { !s.chars().all(char::is_whitespace) }
    for eob in actual.lines().filter(isnt_whitespace).zip_longest(
               expected.lines().filter(isnt_whitespace)) {
        match eob {
            EitherOrBoth::Both(l, r) => {
                let l = l.trim_right();
                let r = r.trim_right();
                if r.trim_left() == "#ignore" {
                    continue;
                }

                if l != r{
                    return Err(format!("actual:\n{}\n=====\nexpected:\n{}", actual, expected));
                }
            }
            EitherOrBoth::Left(l) => {
                return Err(format!("actual has more lines: {} \n actual:\n{}\n=====\nexpected:\n{}",
                                    l, actual, expected));
            }
            EitherOrBoth::Right(l) => {
                return Err(format!("expected has more lines: {} \n actual:\n{}\n=====\nexpected:\n{}",
                                    l, actual, expected));
            }
        }
    }
    Ok(())
}

pub fn test_binding(program: &str, bound_representation: &str, modules: Option<&Modules>, interner: &mut SymbolIntern) -> TestResult {
    let parse_arena = Arena::new();
    let bind_arena = Arena::new();
    let bound = match do_binding(program, &parse_arena, &bind_arena, interner, modules) {
        Ok(b) => b,
        Err(e) => return TestResult::Error(e),
    };

    let mut buffer = String::with_capacity(bound_representation.len());
    for bound in bound {
        format(bound, 0, &interner, &mut buffer).unwrap();
    }

    if let Err(e) = str_eq(&buffer, bound_representation) {
        return TestResult::Bad(e);
    }

    TestResult::Good
}
