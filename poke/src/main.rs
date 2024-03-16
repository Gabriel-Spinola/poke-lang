mod chunk;
mod value;
mod debug;
mod vm;
use chunk::OpCode;
use debug::disassemble_chunk;
use vm::VirtualMachine;

use crate::chunk::Chunk;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write_chunk(OpCode::Return.to_byte(), 123);
    chunk.write_constant(1.2, 123);
    chunk.write_constant(1.2, 123);
    chunk.write_constant(1.2, 128);
    chunk.write_constant(1.2, 182);

    disassemble_chunk(&chunk, "test chunk");

    let mut _vm = VirtualMachine::new(&chunk);
}
