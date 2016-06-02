use super::compiler;
use vm::{Instr, Modules};
use compiler::CompileContext;
use ares_syntax::{SymbolIntern};
use typed_arena::Arena;
pub use ares_syntax::{Span, Ast, AstRef};

pub fn ok_parse_1_full(program: &str, interner: &mut SymbolIntern) -> AstRef<'static> {
    use std::mem::{transmute, forget};
    let arena: Arena<Ast> = Arena::new();
    let arena_ref: &'static _ = unsafe{ transmute(&arena)};
    let mut asts = ::compiler::parse::parse(program, interner, arena_ref).unwrap();
    assert!(asts.len() == 1);
    let result = asts.pop().unwrap();
    let result = arena_ref.alloc(result);
    forget(arena);
    result
}

pub fn compile_this(program: &str, globals: Option<&Modules>) -> (Vec<Instr>, CompileContext) {
    use compiler::emit::emit;
    use compiler::emit::EmitBuffer;
    use compiler::binding::Bound;

    let mut out = EmitBuffer::new();
    let mut compile_context = CompileContext::new();
    let mut bound_arena = Arena::new();
    let mut interner = SymbolIntern::new();
    let ast = ok_parse_1_full(program, &mut interner);
    let bound = Bound::bind_top(ast, &mut bound_arena, globals, &mut interner).unwrap();
    emit(&bound, &mut compile_context, &mut out, None).unwrap();
    (out.into_instructions(), compile_context)
}

fn assert_instrs(program: &str, output: &str, globals: Option<&Modules>) {
    let (instrs, _) = compile_this(program, globals);
    let instrs_lines: Vec<String> = instrs.iter().map(|a| format!("{:?}", a)).collect();
    let expected_lines: Vec<String> = output.lines().map(String::from).collect();
    if instrs_lines.len() != expected_lines.len() {
        println!("{} instructions were expected, but only {} were actually produced", expected_lines.len(), instrs_lines.len());
        println!("EXPECTED: \n{}", expected_lines.join("\n"));
        println!("ACTUAL: \n{}", instrs_lines.join("\n"));
        panic!();
    }

    for (i, (expected, actual)) in expected_lines.into_iter().zip(instrs_lines.into_iter()).enumerate() {
        if expected != actual {
            panic!("at {}, {} was expected, but {} was produced", i, expected, actual);
        }
    }
}

pub fn assert_compilation_steps(program: &str, bound: Option<String>, instr: Option<String>, output: Option<String>, result: Option<String>) -> bool {
    use host::*;
    use vm::*;

    let mut ctx: UnloadedContext<Vec<String>> = UnloadedContext::new();

    ctx.set_global("print", user_function(None, |args, state: &mut Vec<String>, ctx| {
        assert!(args.len() == 1);
        let formatted: String = ctx.format_value(&args[0]);
        state.push(formatted);
        0.into()
    }));

    // Binding
    if let Some(bound) = bound {
        let &mut UnloadedContext{vm: Vm {ref mut interner, ref mut globals, ..}} = &mut ctx;
        compiler::binding::test::assert_bound_form(program, &bound, Some(globals), interner);
    }

    if let Some(instr) = instr {
        assert_instrs(program, &instr, Some(ctx.modules()));
    }

    if output.is_some() || result.is_some() {

        let mut actual_output = vec![];
        let actual_result = {
            let mut ctx = ctx.load(&mut actual_output);
            ctx.eval(&program).unwrap()
        };

        if let Some(expected_output) = output {
            assert_eq!(expected_output.lines().map(String::from).collect::<Vec<_>>(), actual_output)
        }
        if let Some(expected_result) = result {
            if let Some(actual_result) = actual_result {
                let as_string = ctx.format_value(&actual_result);
                assert!(expected_result == as_string, "The program \n{}\n had a result of {:?} but you thought it was {:?}", program, actual_result, expected_result);
            } else {
                assert!(expected_result.is_empty(), "The program \n{}\n had no return value, but you provieded {}", program, expected_result);
            }
        }
        return actual_output.len() == 0;
    }
    return false;
}
