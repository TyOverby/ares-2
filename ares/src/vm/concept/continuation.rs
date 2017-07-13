use vm::{Value, Frame};

#[derive(Debug, PartialEq, Clone, Trace, Finalize)]
pub struct Continuation {
    pub instruction_pos: u32,
    pub saved_stack: Vec<Value>,
    pub saved_stack_frames: Vec<Frame>,
}
