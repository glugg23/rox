use rox_lib::chunk::{Chunk, OpCode};
use rox_lib::debug::disassemble_chuck;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::Return);
    disassemble_chuck(&chunk, "test chunk");
}
