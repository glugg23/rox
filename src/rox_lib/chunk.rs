pub enum OpCode {
    Return,
}

pub struct Chunk {
    pub code: Vec<OpCode>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk { code: Vec::new() }
    }

    pub fn write(&mut self, byte: OpCode) {
        self.code.push(byte);
    }
}
