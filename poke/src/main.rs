mod chunk;
mod value;
mod debug;
use chunk::OpCode;
use debug::disassemble_chunk;

use crate::chunk::Chunk;

fn main() {
    let mut chunk = Chunk::init_chunk();
    Chunk::write_chunk(&mut chunk, OpCode::Return.to_byte(), 123);
    
    let constant_index = chunk.add_constant(1.2);
    Chunk::write_chunk(&mut chunk, OpCode::Constant.to_byte(), 123);
    Chunk::write_chunk(&mut chunk, constant_index as u8, 123);

    disassemble_chunk(&chunk, "test chunk");
}
