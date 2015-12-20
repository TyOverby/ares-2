use gc::Trace;
use vm::Value;

#[derive(Debug, PartialEq, Clone)]
pub struct ClosureClass {
    pub code_offset: u32,
    pub arg_count: u32,
    pub local_defines_count: u32,
    pub has_rest_params: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Closure {
    pub class: ClosureClass,
    pub upvars: Vec<Value>,
}

unsafe impl Trace for Closure {
    custom_trace!(this, {
        for upvar in &this.upvars {
            mark(upvar);
        }
    });
}
