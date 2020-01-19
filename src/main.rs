use rox_lib::chunk::{Chunk, OpCode};
use rox_lib::debug::disassemble_chuck;

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write(OpCode::Constant as u8, 1);
    chunk.write(constant as u8, 1);

    chunk.write(OpCode::Return as u8, 1);

    disassemble_chuck(&chunk, "test chunk");
}
