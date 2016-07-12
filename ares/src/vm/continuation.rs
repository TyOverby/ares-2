use gc::Trace;
use vm::{Value, UtilityStackItem};

#[derive(Debug, PartialEq, Clone)]
pub struct Continuation {
    pub instruction_pos: u32,
    pub saved_stack: Vec<Value>,
    pub saved_utility_stack: Vec<UtilityStackItem>,
}

unsafe impl Trace for Continuation {
    custom_trace!(this, {
        for value in &this.saved_stack {
            mark(value);
        }
    });
}
