use crate::chunk::OpCode::*;
use crate::chunk::{Chunk, OpCode};
use crate::compiler::compile;
use crate::debug::{disassemble_instruction, print_value};
use crate::Value;

macro_rules! binary_op {
    ($vm:ident, $op:tt) => (
        {
            let b = $vm.pop();
            let a = $vm.pop();
            $vm.push(a $op b);
        };
    )
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

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        self.chunk = match compile(source) {
            Some(c) => c,
            None => return InterpretResult::CompileError,
        };

        self.ip = 0;

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
                Constant => {
                    let constant = self.read_constant();
                    self.push(constant);
                }
                Add => binary_op!(self, +),
                Subtract => binary_op!(self, -),
                Multiple => binary_op!(self, *),
                Divide => binary_op!(self, /),
                Negate => {
                    let negated = -self.pop();
                    self.push(negated);
                }
                Return => {
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

#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vm_push() {
        let mut vm = VM::new();

        vm.push(1.0);

        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], 1.0);
    }

    #[test]
    fn vm_pop() {
        let mut vm = VM::new();
        vm.push(1.0);

        let result = vm.pop();

        assert_eq!(vm.stack.len(), 0);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn vm_read_byte() {
        let mut vm = VM::new();
        vm.chunk = Chunk {
            code: vec![0],
            constants: Vec::new(),
            lines: Vec::new(),
        };

        let result = vm.read_byte();

        assert_eq!(result, 0);
    }

    #[test]
    fn vm_read_constant() {
        let mut vm = VM::new();
        vm.chunk = Chunk {
            code: vec![0],
            constants: vec![1.0],
            lines: Vec::new(),
        };

        let result = vm.read_constant();

        assert_eq!(result, 1.0);
    }

    #[test]
    fn vm_run_constant() {
        let mut vm = VM::new();
        vm.chunk = Chunk {
            code: vec![Constant as u8, 0, Return as u8],
            constants: vec![1.0],
            lines: vec![1, 1, 1],
        };

        let result = vm.run();

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_run_negate() {
        let mut vm = VM::new();
        vm.chunk = Chunk {
            code: vec![Constant as u8, 0, Negate as u8, Return as u8],
            constants: vec![1.0],
            lines: vec![1, 1, 1, 1],
        };

        let result = vm.run();

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_run_add() {
        let mut vm = VM::new();
        vm.chunk = Chunk {
            code: vec![
                Constant as u8,
                0,
                Constant as u8,
                1,
                Add as u8,
                Return as u8,
            ],
            constants: vec![1.0, 2.0],
            lines: vec![1, 1, 1, 1, 1, 1],
        };

        let result = vm.run();

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_run_subtract() {
        let mut vm = VM::new();
        vm.chunk = Chunk {
            code: vec![
                Constant as u8,
                0,
                Constant as u8,
                1,
                Subtract as u8,
                Return as u8,
            ],
            constants: vec![1.0, 2.0],
            lines: vec![1, 1, 1, 1, 1, 1],
        };

        let result = vm.run();

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_run_multiply() {
        let mut vm = VM::new();
        vm.chunk = Chunk {
            code: vec![
                Constant as u8,
                0,
                Constant as u8,
                1,
                Multiple as u8,
                Return as u8,
            ],
            constants: vec![1.0, 2.0],
            lines: vec![1, 1, 1, 1, 1, 1],
        };

        let result = vm.run();

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_run_divide() {
        let mut vm = VM::new();
        vm.chunk = Chunk {
            code: vec![
                Constant as u8,
                0,
                Constant as u8,
                1,
                Divide as u8,
                Return as u8,
            ],
            constants: vec![1.0, 2.0],
            lines: vec![1, 1, 1, 1, 1, 1],
        };

        let result = vm.run();

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret() {
        let mut vm = VM::new();

        let result = vm.interpret("42 * (1 - (1 / 0))");

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_compile_error() {
        let mut vm = VM::new();

        let result = vm.interpret("+1");

        assert_eq!(result, InterpretResult::CompileError);
    }
}
