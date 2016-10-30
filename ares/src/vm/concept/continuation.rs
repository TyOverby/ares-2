use gc::Trace;
use vm::{Value, Frame};

#[derive(Debug, PartialEq, Clone)]
pub struct Continuation {
    pub instruction_pos: u32,
    pub saved_stack: Vec<Value>,
    pub saved_stack_frames: Vec<Frame>,
}

unsafe impl Trace for Continuation {
    custom_trace!(this, {
        for value in &this.saved_stack {
            mark(value);
        }
    });
}
