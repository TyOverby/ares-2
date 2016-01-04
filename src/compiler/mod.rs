mod parse;
mod emit;
mod error;
mod compile_context;
mod binding;

use typed_arena;
use compiler::error::CompileError;
use compiler::emit::EmitBuffer;
use vm::{SymbolIntern, Instr};

pub use self::compile_context::CompileContext;

pub fn compile(source: &str,
               compile_context: &mut CompileContext,
               interner: &mut SymbolIntern)
               -> Result<Vec<Instr>, CompileError> {

    let ast_arena: typed_arena::Arena<parse::Ast> = typed_arena::Arena::new();
    let bound_arena: typed_arena::Arena<binding::Bound> = typed_arena::Arena::new();

    let mut out = EmitBuffer::new();
    let asts = try!(parse::parse(source, interner, &ast_arena));
    for ast in &asts {
        let bound = binding::Bound::bind_top(ast, &bound_arena, interner);
        try!(emit::emit(try!(bound), compile_context, &mut out, None));
    }
    Ok(out.into_instructions())
}
