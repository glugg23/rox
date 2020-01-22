use rox_lib::chunk::{Chunk, OpCode};
use rox_lib::vm::VM;

fn main() {
    let mut vm = VM::new();

    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write(OpCode::Constant as u8, 1);
    chunk.write(constant as u8, 1);

    let constant = chunk.add_constant(3.4);
    chunk.write(OpCode::Constant as u8, 2);
    chunk.write(constant as u8, 2);

    chunk.write(OpCode::Add as u8, 3);

    let constant = chunk.add_constant(5.6);
    chunk.write(OpCode::Constant as u8, 4);
    chunk.write(constant as u8, 4);

    chunk.write(OpCode::Divide as u8, 5);

    chunk.write(OpCode::Negate as u8, 6);

    chunk.write(OpCode::Return as u8, 7);

    vm.interpret(chunk);
}
