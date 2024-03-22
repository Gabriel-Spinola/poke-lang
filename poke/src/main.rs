mod chunk;
mod debug;
mod parser;
mod value;
mod vm;
use parser::lexer::Lexer;
use std::{env, fs::File, io::BufReader};

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
}
