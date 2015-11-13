use vm::{Value, Instr};

pub enum CompileOptLevel {
    None,
    Few,
    All
}

#[derive(Debug)]
pub struct CompileContext {
    constants: Vec<Value>
}

impl CompileContext {
    pub fn new() -> CompileContext {
        CompileContext {
            constants: vec![]
        }
    }

    pub fn add_constant(&mut self, constant: Value) -> Instr {
        let id = self.constants.len();
        self.constants.push(constant);
        Instr::LoadConstant(id as u32)
    }

    pub fn get_constant(&self, id: u32) -> Value {
        self.constants[id as usize].clone()
    }
}
