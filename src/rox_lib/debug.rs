use crate::chunk::OpCode::*;
use crate::chunk::{Chunk, OpCode};

pub fn disassemble_chuck(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:0>4} ", offset);

    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{:>4} ", chunk.lines[offset]);
    }

    let instruction = OpCode::from(chunk.code[offset]);
    match instruction {
        Constant | GetGlobal | DefineGlobal | SetGlobal => {
            constant_instruction(instruction, chunk, offset)
        }
        GetLocal | SetLocal => byte_instruction(instruction, chunk, offset),
        Nil | True | False | Pop | Equal | Greater | Less | Add | Subtract | Multiple | Divide
        | Not | Negate | Print | Return => simple_instruction(instruction, offset),
    }
}

fn simple_instruction(instruction: OpCode, offset: usize) -> usize {
    println!("{}", instruction);
    offset + 1
}

fn constant_instruction(instruction: OpCode, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1] as usize;
    print!("{:<16} {:>4} ", instruction.to_string(), constant);
    println!("{}", chunk.constants[constant]);
    offset + 2
}

fn byte_instruction(instruction: OpCode, chunk: &Chunk, offset: usize) -> usize {
    let slot = chunk.code[offset + 1] as usize;
    println!("{:<16} {:>4} ", instruction.to_string(), slot);
    offset + 2
}
