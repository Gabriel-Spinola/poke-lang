mod chunk;
mod debug;
mod parser;
mod value;
mod vm;
use chunk::Chunk;
use parser::parser::Parser;
use std::{env, fs::File, io::BufReader};
use vm::VirtualMachine;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} script", args[0]);

        return;
    }

    let file = File::open(&args[1]).expect("poke file");

    let mut chunk = Chunk::new();
    let mut _parser = Parser::new(&mut chunk).load(BufReader::new(file));

    let mut vm = VirtualMachine::new(&chunk);
    match vm.run_interpreter() {
        Ok(_) => println!("VM executed succesfully"),
        Err(error) => panic!("VM failed {:?}", error),
    };
}
