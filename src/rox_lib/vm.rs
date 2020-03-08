use crate::chunk::OpCode::*;
use crate::chunk::{Chunk, OpCode};
use crate::compiler::compile;
use crate::debug::disassemble_instruction;
use crate::Value;

macro_rules! binary_op {
    ($vm:ident, $type:expr, $op:tt) => (
        {
            if let Value::Number(_) = $vm.peek(0) {
                if let Value::Number(_) = $vm.peek(1) {
                    let b: f64 = $vm.pop().into();
                    let a: f64 = $vm.pop().into();
                    $vm.push($type(a $op b));
                } else {
                    $vm.runtime_error("Operand must be a number.");
                    return InterpretResult::RuntimeError;
                }
            } else {
                $vm.runtime_error("Operand must be a number.");
                return InterpretResult::RuntimeError;
            }
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
                    print!("{}", slot);
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
                Nil => self.push(Value::Nil),
                True => self.push(Value::Boolean(true)),
                False => self.push(Value::Boolean(false)),
                Add => binary_op!(self, Value::Number, +),
                Subtract => binary_op!(self, Value::Number, -),
                Multiple => binary_op!(self, Value::Number, *),
                Divide => binary_op!(self, Value::Number, /),
                Not => {
                    let value = self.pop().is_falsey();
                    self.push(Value::Boolean(value));
                }
                Negate => match self.peek(0) {
                    Value::Number(_) => {
                        let n: f64 = self.pop().into();
                        self.push(Value::Number(-n))
                    }
                    _ => {
                        self.runtime_error("Operand must be a number.");
                        return InterpretResult::RuntimeError;
                    }
                },
                Return => {
                    print!("{}", self.pop());
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
        self.chunk.constants[index].clone()
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        //Unwrap for now
        self.stack.pop().unwrap()
    }

    fn peek(&self, distance: usize) -> &Value {
        &self.stack[(self.stack.len() - 1) - distance]
    }

    fn runtime_error(&mut self, message: &str) {
        eprintln!(
            "{}\n[line {}] in script",
            message, self.chunk.lines[self.ip]
        );
        self.stack.clear();
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

        vm.push(Value::Number(1.0));

        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Number(1.0));
    }

    #[test]
    fn vm_pop() {
        let mut vm = VM::new();
        vm.push(Value::Number(1.0));

        let result = vm.pop();

        assert_eq!(vm.stack.len(), 0);
        assert_eq!(result, Value::Number(1.0));
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
            constants: vec![Value::Number(1.0)],
            lines: Vec::new(),
        };

        let result = vm.read_constant();

        assert_eq!(result, Value::Number(1.0));
    }

    #[test]
    fn vm_run_constant() {
        let mut vm = VM::new();
        vm.chunk = Chunk {
            code: vec![Constant as u8, 0, Return as u8],
            constants: vec![Value::Number(1.0)],
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
            constants: vec![Value::Number(1.0)],
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
            constants: vec![Value::Number(1.0), Value::Number(2.0)],
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
            constants: vec![Value::Number(1.0), Value::Number(2.0)],
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
            constants: vec![Value::Number(1.0), Value::Number(2.0)],
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
            constants: vec![Value::Number(1.0), Value::Number(2.0)],
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
