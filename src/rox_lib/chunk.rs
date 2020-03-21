use crate::value::Value;
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
    Pop,
    GetGlobal,
    DefineGlobal,
    SetGlobal,
    Equal,
    Greater,
    Less,
    Add,
    Subtract,
    Multiple,
    Divide,
    Not,
    Negate,
    Print,
    Return,
}

impl From<u8> for OpCode {
    fn from(byte: u8) -> Self {
        match byte {
            0 => OpCode::Constant,
            1 => OpCode::Nil,
            2 => OpCode::True,
            3 => OpCode::False,
            4 => OpCode::Pop,
            5 => OpCode::GetGlobal,
            6 => OpCode::DefineGlobal,
            7 => OpCode::SetGlobal,
            8 => OpCode::Equal,
            9 => OpCode::Greater,
            10 => OpCode::Less,
            11 => OpCode::Add,
            12 => OpCode::Subtract,
            13 => OpCode::Multiple,
            14 => OpCode::Divide,
            15 => OpCode::Not,
            16 => OpCode::Negate,
            17 => OpCode::Print,
            18 => OpCode::Return,
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
                OpCode::Pop => "POP",
                OpCode::GetGlobal => "GET GLOBAL",
                OpCode::DefineGlobal => "DEFINE GLOBAL",
                OpCode::SetGlobal => "SET GLOBAL",
                OpCode::Equal => "EQUAL",
                OpCode::Greater => "GREATER",
                OpCode::Less => "LESS",
                OpCode::Add => "ADD",
                OpCode::Subtract => "SUBTRACT",
                OpCode::Multiple => "MULTIPLE",
                OpCode::Divide => "DIVIDE",
                OpCode::Not => "NOT",
                OpCode::Negate => "NEGATE",
                OpCode::Print => "PRINT",
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
    #[should_panic(expected = "Unknown Opcode")]
    fn opcode_from_invalid_byte_should_panic() {
        OpCode::from(255);
    }
}
