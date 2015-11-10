#![allow(dead_code)]
#![feature(box_patterns)]
extern crate ares_vm;
mod parse;
mod emit;
mod error;

use error::CompileError;

pub fn compile(source: &str, interner: &mut ares_vm::SymbolIntern) -> Result<Vec<ares_vm::Instr>, CompileError> {
    let mut out = vec![];
    let asts = try!(parse::parse(source, interner));
    for ast in &asts {
        emit::emit(ast, &mut out);
    }
    Ok(out)
}
