pub(crate) mod parse;
pub(crate) mod emit;
mod error;
pub(crate) mod compile_context;
pub(crate) mod binding;


use typed_arena;
pub use compiler::error::*;
use compiler::emit::EmitBuffer;
use ares_syntax::SymbolIntern;
use vm::{Instr, Modules};

pub use self::compile_context::{CompileContext, ShiftMeta};

pub fn compile(source: &str,
               compile_context: &mut CompileContext,
               modules: Option<&Modules>,
               interner: &mut SymbolIntern,
               emit_offset: usize)
               -> Result<Vec<Instr>, CompileError> {

    let ast_arena: typed_arena::Arena<parse::Ast> = typed_arena::Arena::new();
    let bound_arena: typed_arena::Arena<binding::Bound> = typed_arena::Arena::new();

    let mut out = EmitBuffer::new(emit_offset);
    let asts: Vec<parse::Ast> = try!(parse::parse(source, interner, &ast_arena));
    let asts: Vec<&parse::Ast> = asts.into_iter().map(|a| ast_arena.alloc(a) as &_).collect();
    let bounds = try!(binding::Bound::bind_top(&asts, &bound_arena, modules, interner));

    try!(emit::emit_all(bounds, compile_context, interner, &mut out, None));

    Ok(out.into_instructions())
}
