use crate::chunk::{ByteCode, Chunk};
use crate::parser::lexer::Lexer;
use crate::parser::tokens::Token;

fn constant_long_instruction(chunk: &Chunk, offset: usize) -> (String, usize) {
    // by combining the three bytes using `|`, we merge thenm into a single
    // 24 bits unsigned integer, thus representing 2^16 (65.536)
    let constant_index = (chunk.code[offset + 1] as u32) // lowest byte
        | ((chunk.code[offset + 2] as u32) << 8) // mid byte
        | ((chunk.code[offset + 3] as u32) << 16); // highest byte

    let constant_value: f64 = chunk.constants[constant_index as usize];
    let instruction_size = 4;

    (
        format!(
            "OP_CONSTANT {:?} <- {:04} INDEX {:?}",
            constant_value,
            offset + 1,
            constant_index
        ),
        offset + instruction_size,
    )
}

fn simple_instruction(operation: &str, offset: usize) -> (String, usize) {
    (operation.to_string(), offset + 1)
}

fn constant_instruction(chunk: &Chunk, offset: usize) -> (String, usize) {
    let constant_index: u8 = chunk.code[offset + 1];
    let constant_value: f64 = chunk.constants[constant_index as usize];
    let instruction_size = 2;

    (
        format!(
            "OP_CONSTANT {:?} <- {:04} INDEX {:?}",
            constant_value,
            offset + 1,
            constant_index
        ),
        offset + instruction_size,
    )
}

#[cfg(feature = "debug_trace_execution")]
pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> (String, usize) {
    // Print lines info
    let instruction = chunk.code[offset];
    let line = chunk.get_line(&offset).expect(&format!(
        "line not found for given instruction: {:04}",
        instruction
    ));

    if (offset > 0) && (line == chunk.get_line(&(offset - 1)).unwrap()) {
        print!("  |  ");
    } else {
        print!("{:04} ", line);
    }

    // Print Operations info
    if let Some(operation) = ByteCode::all_variants().get(instruction as usize) {
        return match operation {
            ByteCode::Return => simple_instruction("OP_RETURN", offset),
            ByteCode::Constant => constant_instruction(chunk, offset),
            ByteCode::ConstantLong => constant_long_instruction(chunk, offset),
            ByteCode::Negate => simple_instruction("OP_NEGATE", offset),
            ByteCode::Add => simple_instruction("OP_ADD", offset),
            ByteCode::Subtract => simple_instruction("OP_SUBTRACT", offset),
            ByteCode::Multiply => simple_instruction("OP_MULTIPLY", offset),
            ByteCode::Divide => simple_instruction("OP_DIVIDE", offset),
        };
    }

    // Not found case
    (
        format!("Unknown operator code: {}", instruction),
        offset + 1,
    )
}

#[cfg(feature = "debug_trace_execution")]
pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("==== Chunk {:?} Disassemble ====", name);
    println!("LINE | OPCODE | VALUE? | ...\n");

    let mut offset: usize = 0;
    let mut text: String;
    while offset < chunk.count as usize {
        (text, offset) = disassemble_instruction(chunk, offset);

        println!("{:04} {}", offset, text);
    }
}

#[cfg(feature = "debug_trace_lex_execution")]
pub fn disassemble_lexer<R: std::io::Read>(lexer: &mut Lexer<R>, name: &str) {
    println!("==== Lexer {:?} Disassemble ====", name);
    println!("LINE | TOKEN");

    let mut previus_line = lexer.current_line;
    loop {
        let token = lexer.advance();
        if token == Token::EoS {
            println!("END STREAM");
            break;
        }

        match previus_line {
            // lex line starts from 0.
            0 => print!("{:04} ", lexer.current_line + 1),

            _ if previus_line == lexer.current_line + 1 => print!("  |  "),
            _ => print!("{:04} ", lexer.current_line + 1),
        }

        previus_line = lexer.current_line + 1;
        println!("{:?}", token);
    }
}
