use crate::chunk::{Chunk, OpCode};
use crate::Value;

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
        OpCode::Return => simple_instruction(instruction, offset),
        OpCode::Constant => constant_instruction(instruction, chunk, offset),
    }
}

fn simple_instruction(instruction: OpCode, offset: usize) -> usize {
    println!("{}", instruction);
    offset + 1
}

fn constant_instruction(instruction: OpCode, chunk: &Chunk, offset: usize) -> usize {
    let constant =  chunk.code[offset + 1] as usize;
    print!("{:<16} {:>4} ", instruction.to_string(), constant);
    print_value(chunk.constants[constant]);
    println!();
    offset + 2
}

fn print_value(value: Value) {
    //Might be possible to remove this function call, depending on how value is implemented further
    print!("'{}'", value);
}
