use gc::{Trace};
use {ReferenceMap};

#[derive(Debug, PartialEq, Clone)]
pub struct Lambda {
    pub upvars: ReferenceMap,
    pub code_offset: u32,
}

impl Lambda {
    pub fn new(ref_map: ReferenceMap, code_offset: u32) -> Lambda {
        Lambda {
            upvars: ref_map,
            code_offset: code_offset,
        }
    }
}

unsafe impl Trace for Lambda {
    custom_trace!(this, {
        mark(&this.upvars);
    });
}
