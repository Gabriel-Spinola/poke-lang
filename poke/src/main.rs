mod chunk;
mod value;
use chunk::OpCode;

use crate::chunk::Chunk;

fn main() {
    let mut chunk = Chunk::init_chunk();
    Chunk::write_chunk(&mut chunk, OpCode::Return.to_byte());
    
    let constant_index = chunk.add_constant(1.2);
    Chunk::write_chunk(&mut chunk, OpCode::Constant.to_byte());
    Chunk::write_chunk(&mut chunk, constant_index as u8);

    chunk.disassemble_chunk("test");
}
