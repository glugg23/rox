use crate::chunk::{Chunk, OpCode};
use crate::debug::{disassemble_instruction, print_value};
use crate::Value;

#[macro_export]
macro_rules! binary_op {
    ($vm:ident, $op:tt) => {
        let b = $vm.pop();
        let a = $vm.pop();
        $vm.push(a $op b);
    };
}

pub struct VM {
    chunk: Chunk,
    ip: usize, //Instruction Pointer
    stack: Vec<Value>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            chunk: Chunk::new(), //Create throwaway Chunk to avoid Option<Chunk>
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            if cfg!(debug_assertions) {
                print!("          ");

                for slot in &self.stack {
                    print!("[ ");
                    print_value(*slot);
                    print!(" ]");
                }
                println!();

                disassemble_instruction(&self.chunk, self.ip);
            }

            let instruction = OpCode::from(self.read_byte());

            match instruction {
                OpCode::Constant => {
                    let constant = self.read_constant();
                    self.push(constant);
                    print_value(constant);
                    println!();
                }
                OpCode::Negate => {
                    let negated = -self.pop();
                    self.push(negated);
                }
                OpCode::Return => {
                    print_value(self.pop());
                    println!();
                    return InterpretResult::Ok;
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

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        //Unwrap for now
        self.stack.pop().unwrap()
    }
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}
