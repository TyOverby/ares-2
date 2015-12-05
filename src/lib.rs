#![allow(dead_code)]

#[macro_use]
extern crate gc;
extern crate libc;
extern crate typed_arena;

macro_rules! matches {
    ($e: expr, $p: pat) => {
        if let $p = $e { true } else { false }
    }
}

pub mod compiler;
pub mod vm;
mod util;
