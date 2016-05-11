#![allow(dead_code)]

#[macro_use]
extern crate gc;
extern crate libc;
extern crate typed_arena;
#[macro_use(ast)]
extern crate ares_syntax;
extern crate itertools;
extern crate lalrpop_util;
#[cfg(test)]
extern crate latin;

macro_rules! matches {
    ($e: expr, $p: pat) => {
        if let $p = $e { true } else { false }
    }
}

pub mod compiler;
pub mod vm;
pub mod host;
mod util;
