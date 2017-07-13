#![recursion_limit="1024"]

#![allow(dead_code)]

#[macro_use]
extern crate gc;
#[macro_use]
extern crate gc_derive;
extern crate ares_syntax;
extern crate libc;
extern crate typed_arena;
extern crate itertools;
extern crate lalrpop_util;

#[cfg(test)]
extern crate latin;

pub mod compiler;
pub mod vm;
pub mod host;
pub mod test;
mod util;
