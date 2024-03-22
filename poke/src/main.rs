mod chunk;
mod debug;
mod parser;
mod value;
mod vm;
use chunk::{ByteCode, Chunk};
use parser::lexer::Lexer;
use std::{env, fs::File, io::BufReader};
use vm::VirtualMachine;

#[cfg(feature = "debug_trace_execution")]
#[cfg(feature = "debug_trace_lex_execution")]
use debug::disassemble_lexer;

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

    let mut chunk = Chunk::new();
    chunk.write_chunk(ByteCode::Return as u8, 0);

    let mut vm = VirtualMachine::new(&chunk);
    match vm.run_interpreter() {
        Ok(_) => println!("VM executed succesfully"),
        Err(error) => panic!("VM failed {:?}", error),
    };
}
