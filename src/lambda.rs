use {ReferenceMap, Instr};

#[derive(Debug, PartialEq, Clone)]
pub struct Lambda {
    pub upvars: ReferenceMap,
    pub code: Vec<Instr>
}

impl Lambda {
    pub fn new(ref_map: ReferenceMap, code: Vec<Instr>) -> Lambda {
        Lambda {
            upvars: ref_map,
            code: code
        }
    }
}
