mod chunk;
mod value;
mod debug;
use chunk::OpCode;
use debug::disassemble_chunk;

use crate::chunk::Chunk;

fn main() {
    let mut chunk = Chunk::init_chunk();
    chunk.write_chunk(OpCode::Return.to_byte(), 123);
    chunk.write_constant(1.2, 123);

    disassemble_chunk(&chunk, "test chunk");
}
