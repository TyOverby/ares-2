use std::collections::HashMap;
use ares_vm::{Value, Instr};


struct CompileContext {
    next_constant_id: u32,
    constants: HashMap<u32, Value>
}

impl CompileContext {
    fn new() -> CompileContext {
        CompileContext {
            next_constant_id: 0,
            constants: HashMap::new(),
        }
    }

    fn add_constant(&mut self, constant: Value) -> Instr {
        let id = self.next_constant_id;
        self.next_constant_id += 1;
        self.constants.insert(id, constant);
        Instr::
    }
}
