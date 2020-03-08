use crate::Value;
use std::fmt;
use std::fmt::{Display, Formatter};

pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<i32>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
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

#[derive(Debug, PartialEq)]
pub enum OpCode {
    Constant,
    Nil,
    True,
    False,
    Add,
    Subtract,
    Multiple,
    Divide,
    Not,
    Negate,
    Return,
}

impl From<u8> for OpCode {
    fn from(byte: u8) -> Self {
        match byte {
            0 => OpCode::Constant,
            1 => OpCode::Nil,
            2 => OpCode::True,
            3 => OpCode::False,
            4 => OpCode::Add,
            5 => OpCode::Subtract,
            6 => OpCode::Multiple,
            7 => OpCode::Divide,
            8 => OpCode::Not,
            9 => OpCode::Negate,
            10 => OpCode::Return,
            _ => panic!("Unknown Opcode"),
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OpCode::Constant => "CONSTANT",
                OpCode::Nil => "NIL",
                OpCode::True => "TRUE",
                OpCode::False => "FALSE",
                OpCode::Add => "ADD",
                OpCode::Subtract => "SUBTRACT",
                OpCode::Multiple => "MULTIPLE",
                OpCode::Divide => "DIVIDE",
                OpCode::Not => "NOT",
                OpCode::Negate => "NEGATE",
                OpCode::Return => "RETURN",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_write() {
        let mut chunk = Chunk::new();

        chunk.write(0, 1);

        assert_eq!(chunk.code[0], 0);
        assert_eq!(chunk.lines[0], 1);
    }

    #[test]
    fn chunk_add_constant() {
        let mut chunk = Chunk::new();

        let index = chunk.add_constant(Value::Number(1.5));

        assert_eq!(index, 0);
        assert_eq!(chunk.constants[0], Value::Number(1.5));
    }

    #[test]
    fn opcode_from_byte() {
        assert_eq!(OpCode::from(0), OpCode::Constant);
        assert_eq!(OpCode::from(1), OpCode::Nil);
        assert_eq!(OpCode::from(2), OpCode::True);
        assert_eq!(OpCode::from(3), OpCode::False);
        assert_eq!(OpCode::from(4), OpCode::Add);
        assert_eq!(OpCode::from(5), OpCode::Subtract);
        assert_eq!(OpCode::from(6), OpCode::Multiple);
        assert_eq!(OpCode::from(7), OpCode::Divide);
        assert_eq!(OpCode::from(8), OpCode::Not);
        assert_eq!(OpCode::from(9), OpCode::Negate);
        assert_eq!(OpCode::from(10), OpCode::Return);
    }

    #[test]
    #[should_panic(expected = "Unknown Opcode")]
    fn opcode_from_invalid_byte_should_panic() {
        OpCode::from(255);
    }
}
