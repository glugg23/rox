use crate::chunk::{Chunk, OpCode};
use crate::debug::{disassemble_instruction, print_value};
use crate::Value;

pub struct VM {
    chunk: Chunk,
    ip: usize, //Instruction Pointer
}

impl VM {
    pub fn new() -> Self {
        VM {
            chunk: Chunk::new(), //Create throwaway Chunk to avoid Option<Chunk>
            ip: 0,
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            if cfg!(debug_assertions) {
                disassemble_instruction(&self.chunk, self.ip);
            }

            let instruction = OpCode::from(self.read_byte());

            match instruction {
                OpCode::Return => return InterpretResult::Ok,
                OpCode::Constant => {
                    let constant = self.read_constant();
                    print_value(constant);
                    println!();
                }
            };
        }
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.chunk.code[self.ip];
        self.ip += 1;
        byte
    }

    fn read_constant(&mut self) -> Value {
        let index = self.read_byte() as usize;
        self.chunk.constants[index]
    }
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}
