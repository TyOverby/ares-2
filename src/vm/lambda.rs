use gc::Trace;
use vm::ReferenceMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Lambda {
    pub upvars: ReferenceMap,
    pub code_offset: u32,
    pub arg_count: u32,
    pub has_rest_params: bool
}

unsafe impl Trace for Lambda {
    custom_trace!(this, {
        mark(&this.upvars);
    });
}
