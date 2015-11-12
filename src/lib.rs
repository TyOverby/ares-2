#![allow(dead_code)]
#![feature(box_patterns)]

#[macro_use]
extern crate gc;
extern crate libc;

pub mod compiler;
pub mod vm;
