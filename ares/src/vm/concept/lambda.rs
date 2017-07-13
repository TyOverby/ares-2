use vm::Value;
use ares_syntax::Symbol;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Clone, Trace, Finalize)]
pub struct ClosureClass {
    pub code_offset: u32,
    pub arg_count: u32,
    pub local_defines_count: u32,
    pub upvars_count: u32,
    pub has_rest_params: bool,
    pub namespace: Symbol,

    pub is_shifter: bool,
}

#[derive(Debug, PartialEq, Clone, Trace, Finalize)]
pub struct Closure {
    pub class: ClosureClass,
    pub upvars: Vec<Value>,
    #[unsafe_ignore_trace]
    pub reset_symbols: RefCell<Option<Vec<Symbol>>>,
}
