mod parse;
mod emit;
mod error;

use compiler::error::CompileError;
use compiler::emit::EmitBuffer;
use vm::{SymbolIntern, Instr};

pub fn compile(source: &str, interner: &mut SymbolIntern) -> Result<Vec<Instr>, CompileError> {
    let mut out = EmitBuffer::new();
    let asts = try!(parse::parse(source, interner));
    for ast in &asts {
        emit::emit(ast, &mut out);
    }
    Ok(out.into_instructions())
}
