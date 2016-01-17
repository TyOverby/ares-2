use std::collections::HashMap;

use vm::Instr;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Standin(i32);

#[derive(Eq, PartialEq, Hash)]
pub struct Fulfill(i32);

pub struct EmitBuffer {
    next_standin_id: i32,
    code: Vec<Instr>,
    // Standin values -> the spot that they need
    // to be rewritten to.
    rewrite: Vec<(Standin, usize)>,
    // Fulfilled standins with the instruction that
    // they should be filled by.
    fulfilled: HashMap<Fulfill, Instr>,
    // A list of positions that have relative instructions
    relative_instrs: Vec<usize>,
}

impl EmitBuffer {
    pub fn new() -> EmitBuffer {
        EmitBuffer {
            next_standin_id: 0,
            code: Vec::new(),
            rewrite: Vec::new(),
            fulfilled: HashMap::new(),
            relative_instrs: Vec::new(),
        }
    }

    pub fn push(&mut self, instr: Instr) {
        self.code.push(instr)
    }

    pub fn push_relative(&mut self, instr: Instr) {
        match instr {
            a@Instr::IntLit(_) |
            a@Instr::Call(_) |
            a@Instr::Jump(_) => {
                let len = self.code.len();
                self.code.push(a);
                self.relative_instrs.push(len);
            }
            a => panic!("non-relative instruction pushed: `push_relative({:?})`", a),
        }
    }

    pub fn standin(&mut self) -> (Standin, Fulfill) {
        let id = self.next_standin_id;
        self.next_standin_id += 1;
        (Standin(id), Fulfill(id))
    }

    pub fn push_standin(&mut self, standin: Standin) {
        let fake_fulfil = Fulfill(standin.0);
        if let Some(instr) = self.fulfilled.get(&fake_fulfil) {
            self.code.push(instr.clone());
        } else {
            let pos = self.code.len();
            self.code.push(Instr::Nop);
            self.rewrite.push((standin, pos));
        }
    }

    pub fn fulfill(&mut self, fulfill: Fulfill, instr: Instr) {
        let fulfill_dup = Fulfill(fulfill.0);
        self.fulfilled.insert(fulfill, instr.clone());
        let fulfill = fulfill_dup;
        let &mut EmitBuffer{ ref mut code, ref mut rewrite, .. } = self;
        rewrite.retain(move |&(ref ff, ref p)| {
            if fulfill.0 == ff.0 {
                code[*p] = instr.clone();
                false
            } else {
                true
            }
        });
    }

    pub fn merge(&mut self, other: EmitBuffer) {
        assert!(other.rewrite.len() == 0);
        let left_length = self.code.len();
        let EmitBuffer { mut code, relative_instrs, .. } = other;
        for relative_pos in relative_instrs {
            match &mut code[relative_pos] {
                &mut Instr::IntLit(ref mut i) => {
                    *i += left_length as i32;
                }
                &mut Instr::Call(ref mut p) => {
                    *p += left_length as u32;
                }
                &mut Instr::Jump(ref mut p) => {
                    *p += left_length as u32;
                }
                a => {
                    panic!("non-relative instruction found in relative positions: `{:?}`",
                           a)
                }
            }
        }
        self.code.extend(code)
    }

    pub fn into_instructions(self) -> Vec<Instr> {
        assert!(self.rewrite.len() == 0);
        self.code
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }
}

#[test]
fn basic_emit_buffer_test() {
    let mut buffer = EmitBuffer::new();
    buffer.push(Instr::AddInt);
    let (standin, fulfill) = buffer.standin();
    buffer.push_standin(standin);
    buffer.fulfill(fulfill, Instr::IntLit(5));
    buffer.push_standin(standin);
    assert_eq!(buffer.code,
               vec![Instr::AddInt, Instr::IntLit(5), Instr::IntLit(5)]);
    assert_eq!(buffer.rewrite.len(), 0);
}
