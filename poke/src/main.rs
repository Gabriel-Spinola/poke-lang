mod chunk;
mod debug;
mod lexer;
mod value;
mod vm;
use std::{env, fs::File, io::BufReader};

use chunk::ByteCode;
use debug::*;
use lexer::{Lexer, Token};
use vm::{InterpretResult, VirtualMachine};

use crate::chunk::Chunk;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} script", args[0]);

        return;
    }

    let file = File::open(&args[1]).unwrap();
    let mut lexer = Lexer::new(BufReader::new(file));

    #[cfg(feature = "debug_trace_lex_execution")]
    disassemble_lexer(&mut lexer, "operators");

    run_vm();
}

fn run_vm() {
    let mut chunk = Chunk::new();
    chunk.write_constant(1.2, 123);
    chunk.write_constant(1.5, 123);

    chunk.write_constant(6.2, 128);
    chunk.write_chunk(ByteCode::Negate as u8, 2);

    chunk.write_constant(1.0, 132);
    chunk.write_constant(2.0, 132);
    chunk.write_chunk(ByteCode::Add as u8, 132);

    chunk.write_constant(1.0, 132);
    chunk.write_constant(2.0, 132);
    chunk.write_chunk(ByteCode::Subtract as u8, 132);

    chunk.write_constant(1.0, 132);
    chunk.write_constant(2.0, 132);
    chunk.write_chunk(ByteCode::Multiply as u8, 132);

    chunk.write_constant(1.0, 132);
    chunk.write_constant(2.0, 132);
    chunk.write_chunk(ByteCode::Divide as u8, 132);

    chunk.write_chunk(ByteCode::Return as u8, 123);

    #[cfg(feature = "debug_trace_execution")]
    disassemble_chunk(&chunk, "test chunk");

    let mut vm = VirtualMachine::new(&chunk);
    let result = vm.run_interpreter();
    if result != InterpretResult::OK {
        println!("VM Failed")
    }
}
