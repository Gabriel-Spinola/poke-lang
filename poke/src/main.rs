pub mod chunk;
use crate::chunk::Chunk;

fn main() {
    let mut chunk: Chunk = Chunk::init_chunk();
    Chunk::write_chunk(&mut chunk, chunk::OpCode::OpReturn as u8);

    chunk.disassemble_chunk("test");
}
