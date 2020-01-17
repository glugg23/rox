use std::fmt::{Display, Error, Formatter};

pub enum OpCode {
    Return,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                OpCode::Return => "Return",
                _ => "Unknown",
            }
        )
    }
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
