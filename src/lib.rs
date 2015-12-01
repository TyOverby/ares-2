#![allow(dead_code)]
#![feature(box_patterns)]

#[macro_use]
extern crate gc;
extern crate libc;

macro_rules! matches {
    ($e: expr, $p: pat) => {
        if let $p = $e { true } else { false }
    }
}

pub mod compiler;
pub mod vm;
