#![feature(pub_restricted)]

#![allow(dead_code)]

#[macro_use]
extern crate gc;
#[macro_use(ast)]
extern crate ares_syntax;
extern crate libc;
extern crate typed_arena;
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
pub mod test;
mod util;
