use crate::chunk::OpCode::*;
use crate::chunk::{Chunk, OpCode};
use crate::compiler::compile;
use crate::debug::disassemble_instruction;
use crate::object::ObjectType;
use crate::value::Value;
use std::collections::HashMap;

macro_rules! binary_op {
    ($vm:ident, $type:expr, $op:tt) => (
        {
            if matches!($vm.peek(0), Value::Number(_)) && matches!($vm.peek(1), Value::Number(_)) {
                let b: f64 = $vm.pop().into();
                let a: f64 = $vm.pop().into();
                $vm.push($type(a $op b));
            } else {
                $vm.runtime_error("Operands must be numbers.");
                return InterpretResult::RuntimeError;
            }
        };
    )
}

pub struct VM {
    chunk: Chunk,
    ip: usize, //Instruction Pointer
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            chunk: Chunk::new(), //Create throwaway Chunk to avoid Option<Chunk>
            ip: 0,
            stack: Vec::new(),
            globals: HashMap::new(),
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
                Pop => {
                    self.pop();
                }
                GetLocal => {
                    let slot = self.read_byte() as usize;
                    let value = self.stack[slot].clone();
                    self.push(value);
                }
                SetLocal => {
                    let slot = self.read_byte() as usize;
                    self.stack[slot] = self.peek(0).clone();
                }
                GetGlobal => {
                    let name = self.read_constant().to_string();
                    let value = match self.globals.get(&name) {
                        Some(v) => v.clone(),
                        None => {
                            self.runtime_error(format!("Undefined variable '{}'.", name).as_str());
                            return InterpretResult::RuntimeError;
                        }
                    };
                    self.push(value);
                }
                DefineGlobal => {
                    let name = self.read_constant().to_string();
                    let value = self.pop();
                    self.globals.insert(name, value);
                }
                SetGlobal => {
                    let name = self.read_constant().to_string();
                    if !self.globals.contains_key(&name) {
                        self.runtime_error(format!("Undefined variable '{}'.", name).as_str());
                        return InterpretResult::RuntimeError;
                    }
                    self.globals.insert(name, self.peek(0).clone());
                }
                Equal => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Boolean(a == b));
                }
                Greater => binary_op!(self, Value::Boolean, >),
                Less => binary_op!(self, Value::Boolean, <),
                Add => {
                    if matches!(self.peek(0), Value::Object(ObjectType::String(_)))
                        && matches!(self.peek(1), Value::Object(ObjectType::String(_)))
                    {
                        let b = self.pop();
                        let a = self.pop();

                        self.push(Value::Object(ObjectType::String(Box::from(
                            a.to_string() + &b.to_string(),
                        ))));
                    } else if matches!(self.peek(0), Value::Number(_))
                        && matches!(self.peek(1), Value::Number(_))
                    {
                        let b: f64 = self.pop().into();
                        let a: f64 = self.pop().into();

                        self.push(Value::Number(a + b));
                    } else {
                        self.runtime_error("Operands must be two numbers or two strings.");
                        return InterpretResult::RuntimeError;
                    }
                }
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
                Print => {
                    println!("{}", self.pop());
                }
                Jump => {
                    let offset = self.read_short() as usize;
                    self.ip += offset;
                }
                JumpIfFalse => {
                    let offset = self.read_short() as usize;

                    if self.peek(0).is_falsey() {
                        self.ip += offset;
                    }
                }
                Loop => {
                    let offset = self.read_short() as usize;
                    self.ip -= offset;
                }
                Return => {
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

    fn read_short(&mut self) -> u16 {
        self.ip += 2;

        u16::from_be_bytes([self.chunk.code[self.ip - 2], self.chunk.code[self.ip - 1]])
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
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
    fn vm_read_short() {
        let mut vm = VM::new();
        vm.chunk = Chunk {
            code: vec![255, 1],
            constants: Vec::new(),
            lines: Vec::new(),
        };

        let result = vm.read_short();

        assert_eq!(result, 65281);
    }

    #[test]
    fn vm_interpret_compile_error() {
        let mut vm = VM::new();

        let result = vm.interpret("+1");

        assert_eq!(result, InterpretResult::CompileError);
    }

    #[test]
    fn vm_interpret_get_local() {
        let mut vm = VM::new();

        let result = vm.interpret("{var a = 1; a;}");

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_set_local() {
        let mut vm = VM::new();

        let result = vm.interpret("{var a = 1; a = 2;}");

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_mismatching_block() {
        let mut vm = VM::new();

        let result = vm.interpret("{");

        assert_eq!(result, InterpretResult::CompileError);
    }

    #[test]
    fn vm_interpret_same_name_same_scope() {
        let mut vm = VM::new();

        let result = vm.interpret("{var a; var a;}");

        assert_eq!(result, InterpretResult::CompileError);
    }

    #[test]
    fn vm_interpret_cannot_read_local_variable_in_own_initializer() {
        let mut vm = VM::new();

        let result = vm.interpret("{var a = a;}");

        assert_eq!(result, InterpretResult::CompileError);
    }

    #[test]
    fn vm_interpret_define_global() {
        let mut vm = VM::new();

        let result = vm.interpret("var a = 1;");

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_define_global_no_value() {
        let mut vm = VM::new();

        let result = vm.interpret("var a;");

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_define_global_no_semicolon() {
        let mut vm = VM::new();

        let result = vm.interpret("var a");

        assert_eq!(result, InterpretResult::CompileError);
    }

    #[test]
    fn vm_interpret_define_global_no_identifier() {
        let mut vm = VM::new();

        let result = vm.interpret("var = 1;");

        assert_eq!(result, InterpretResult::CompileError);
    }

    #[test]
    fn vm_interpret_get_global() {
        let mut vm = VM::new();

        let result = vm.interpret("var a = 1; a;");

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_get_global_undefined() {
        let mut vm = VM::new();

        let result = vm.interpret("a;");

        assert_eq!(result, InterpretResult::RuntimeError);
    }

    #[test]
    fn vm_interpret_set_global() {
        let mut vm = VM::new();

        let result = vm.interpret("var a = 1; a = 2;");

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_set_global_undefined() {
        let mut vm = VM::new();

        let result = vm.interpret("a = 1;");

        assert_eq!(result, InterpretResult::RuntimeError);
    }

    #[test]
    fn vm_interpret_set_global_invalid_target() {
        let mut vm = VM::new();

        let result = vm.interpret("true = 1;");

        assert_eq!(result, InterpretResult::CompileError);
    }

    #[test]
    fn vm_interpret_negate() {
        let mut vm = VM::new();

        let result = vm.interpret("-1;");

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_negate_not_number() {
        let mut vm = VM::new();

        let result = vm.interpret("-false;");

        assert_eq!(result, InterpretResult::RuntimeError);
    }

    #[test]
    fn vm_interpret_equal() {
        let mut vm = VM::new();

        let result = vm.interpret("true == nil;");
        assert_eq!(result, InterpretResult::Ok);

        let result = vm.interpret("1.0 == 1.0;");
        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_not() {
        let mut vm = VM::new();

        let result = vm.interpret("!true;");

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_not_equal() {
        let mut vm = VM::new();

        let result = vm.interpret("true != false;");

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_greater() {
        let mut vm = VM::new();

        let result = vm.interpret("2 > 1;");

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_greater_equal() {
        let mut vm = VM::new();

        let result = vm.interpret("1 >= 1;");

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_less() {
        let mut vm = VM::new();

        let result = vm.interpret("2 < 1;");

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_less_equal() {
        let mut vm = VM::new();

        let result = vm.interpret("1 <= 1;");

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_binary_op_wrong_types() {
        let mut vm = VM::new();

        let result = vm.interpret("1 + true;");
        assert_eq!(result, InterpretResult::RuntimeError);

        let result = vm.interpret("false / 0;");
        assert_eq!(result, InterpretResult::RuntimeError);
    }

    #[test]
    fn vm_interpret_add() {
        let mut vm = VM::new();

        let result = vm.interpret("1 + 2;");

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_add_strings() {
        let mut vm = VM::new();

        let result = vm.interpret("\"hello\" + \" \" + \"world\";");

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_can_not_add_string_and_number() {
        let mut vm = VM::new();

        let result = vm.interpret("\"hello\" + 123;");

        assert_eq!(result, InterpretResult::RuntimeError);
    }

    #[test]
    fn vm_interpret_subtract() {
        let mut vm = VM::new();

        let result = vm.interpret("1 - 0.5;");

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_multiply() {
        let mut vm = VM::new();

        let result = vm.interpret("1 * 10;");

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_divide() {
        let mut vm = VM::new();

        let result = vm.interpret("1 / 0;");

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_print() {
        let mut vm = VM::new();

        let result = vm.interpret("print \"hello world\";");

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_print_with_no_semicolon() {
        let mut vm = VM::new();

        let result = vm.interpret("print \"hello world\"");

        assert_eq!(result, InterpretResult::CompileError);
    }

    #[test]
    fn vm_interpret_if() {
        let mut vm = VM::new();

        let result = vm.interpret("if(true){1;}");

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_if_no_paren() {
        let mut vm = VM::new();

        let result = vm.interpret("if false {0;}");

        assert_eq!(result, InterpretResult::CompileError);
    }

    #[test]
    fn vm_interpret_if_else() {
        let mut vm = VM::new();

        let result = vm.interpret("if(false){0;}else{1;}");

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_and() {
        let mut vm = VM::new();

        let result = vm.interpret("true and true;");

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_or() {
        let mut vm = VM::new();

        let result = vm.interpret("true or true;");

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_while() {
        let mut vm = VM::new();

        let result = vm.interpret("while(false){0;}");

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_while_no_paren() {
        let mut vm = VM::new();

        let result = vm.interpret("while false {0;}");

        assert_eq!(result, InterpretResult::CompileError);
    }

    #[test]
    fn vm_interpret_for() {
        let mut vm = VM::new();

        let result = vm.interpret("for(var i=0;i<5;i=i+1){i;}");

        assert_eq!(result, InterpretResult::Ok);
    }

    #[test]
    fn vm_interpret_for_syntax_error() {
        let mut vm = VM::new();

        let result = vm.interpret("for{0;}");

        assert_eq!(result, InterpretResult::CompileError);
    }
}
