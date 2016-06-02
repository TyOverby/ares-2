pub(crate) mod parse;
pub(crate) mod emit;
mod error;
pub(crate) mod compile_context;
pub(crate) mod binding;


use typed_arena;
pub use compiler::error::CompileError;
use compiler::emit::EmitBuffer;
use ares_syntax::SymbolIntern;
use vm::{Instr, Modules};

pub use self::compile_context::CompileContext;

pub fn compile(source: &str,
               compile_context: &mut CompileContext,
               modules: Option<&Modules>,
               interner: &mut SymbolIntern)
               -> Result<Vec<Instr>, CompileError> {

    let ast_arena: typed_arena::Arena<parse::Ast> = typed_arena::Arena::new();
    let bound_arena: typed_arena::Arena<binding::Bound> = typed_arena::Arena::new();

    let mut out = EmitBuffer::new();
    let asts: Vec<parse::Ast> = try!(parse::parse(source, interner, &ast_arena));
    let asts: Vec<&parse::Ast> = asts.into_iter().map(|a| ast_arena.alloc(a) as &_).collect();
    for ast in asts {
        let bound = binding::Bound::bind_top(ast, &bound_arena, modules, interner);
        try!(emit::emit(try!(bound), compile_context, &mut out, None));
        // Pop because an expression just completed, so we don't
        // want to just leave the result on the stack.
        out.push(Instr::Pop);
    }

    if out.len() != 0 {
        // Pop the last pop.
        out.pop();
    }

    Ok(out.into_instructions())
}
