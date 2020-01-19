use std::fmt::{Display, Error, Formatter};
use crate::Value;

pub enum OpCode {
    Return,
    Constant,
}

impl From<u8> for OpCode {
    fn from(byte: u8) -> Self {
        match byte {
            0 => OpCode::Return,
            1 => OpCode::Constant,
            _ => panic!("Unknown Opcode")
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                OpCode::Return => "RETURN",
                OpCode::Constant => "CONSTANT",
            }
        )
    }
}

pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<i32>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk { code: Vec::new(), constants: Vec::new(), lines: Vec::new() }
    }

    pub fn write(&mut self, byte: u8, line: i32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}
