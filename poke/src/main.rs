mod chunk;
mod debug;
mod value;
mod vm;
use chunk::OpCode;
use debug::*;
use vm::VirtualMachine;

use crate::chunk::Chunk;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write_constant(1.2, 123);
    chunk.write_constant(1.5, 123);
    chunk.write_constant(6.2, 128);
    chunk.write_constant(3.5, 182);
    chunk.write_chunk(OpCode::Return.to_byte(), 123);

    #[cfg(feature = "debug_trace_execution")]
    {
        disassemble_chunk(&chunk, "test chunk");
    }

    let mut vm = VirtualMachine::new(&chunk);
    vm.run_interpreter();
}
