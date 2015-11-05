use {ReferenceMap, Instr};

#[derive(Debug, PartialEq, Clone)]
pub struct Lambda {
    pub upvars: ReferenceMap,
    pub code_id: u32,
    pub offset: u32,
}

impl Lambda {
    pub fn new(ref_map: ReferenceMap, code_id: u32, offset: u32) -> Lambda {
        Lambda {
            upvars: ref_map,
            code_id: code_id,
            offset: offset,
        }
    }
}
