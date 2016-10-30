use vm::{Value, Instr, ClosureClass};

pub enum CompileOptLevel {
    None,
    Few,
    All,
}

#[derive(Debug, Copy, Clone)]
pub struct ShiftMeta {
    pub num_symbols: u32,
    pub return_pos: u32,
}

#[derive(Debug)]
pub struct CompileContext {
    constants: Vec<Value>,
    closure_classes: Vec<ClosureClass>,
    shift_metadata: Vec<ShiftMeta>,
}

impl CompileContext {
    pub fn new() -> CompileContext {
        CompileContext {
            constants: vec![],
            closure_classes: vec![],
            shift_metadata: vec![],
        }
    }

    pub fn add_shift_meta(&mut self, sm: ShiftMeta) -> u32 {
        let id = self.shift_metadata.len();
        self.shift_metadata.push(sm);
        id as u32
    }

    pub fn get_shift_meta(&self, id: u32) -> ShiftMeta {
        self.shift_metadata[id as usize]
    }

    pub fn add_closure_class(&mut self, class: ClosureClass) -> u32 {
        let id = self.closure_classes.len();
        self.closure_classes.push(class);
        id as u32
    }

    pub fn get_lambda_class(&self, id: u32) -> ClosureClass {
        self.closure_classes[id as usize].clone()
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
