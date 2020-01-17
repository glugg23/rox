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

    let instruction = &chunk.code[offset];
    match *instruction {
        OpCode::Return => simple_instruction(instruction, offset),
        _ => {
            println!("Unknown opcode");
            offset + 1
        }
    }
}

fn simple_instruction(instruction: &OpCode, offset: usize) -> usize {
    println!("{}", instruction);
    offset + 1
}
