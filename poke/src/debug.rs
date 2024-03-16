use crate::chunk::{Chunk, OpCode, OP_CODES_MAP};

fn constant_long_instruction(chunk: &Chunk, offset: usize) -> (String, usize) {
    // by combining the three bytes using `|`, we merge thenm into a single 
    // 24 bits unsigned integer, thus representing 2^16 (65.536)
    let constant_index = (chunk.code[offset + 1] as u32) // lowest byte
        | ((chunk.code[offset + 2] as u32) << 8) // mid byte
        | ((chunk.code[offset + 3] as u32) << 16); // highest byte

    let constant_value: f64 = chunk.constants[constant_index as usize];
    let instruction_size = 4;

    return (
        format!(
            "OP_CONSTANT {:?} <- {:04} INDEX {:?}",
            constant_value,
            offset + 1,
            constant_index
        ),
        offset + instruction_size,
    );
}

fn constant_instruction(chunk: &Chunk, offset: usize) -> (String, usize) {
    let constant_index: u8 = chunk.code[offset + 1];
    let constant_value: f64 = chunk.constants[constant_index as usize];
    let instruction_size = 2;

    return (
        format!(
            "OP_CONSTANT {:?} <- {:04} INDEX {:?}",
            constant_value,
            offset + 1,
            constant_index
        ),
        offset + instruction_size,
    );
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> (String, usize) {
    if (offset > 0) && (chunk.lines[offset] == chunk.lines[offset - 1]) {
        print!("  |  ");
    } else {
        print!("{:04} ", chunk.lines[offset]);
    }

    let instruction = chunk.code[offset] as usize;

    if let Some(operation) = OP_CODES_MAP.get(instruction) {
        return match operation {
            OpCode::Return => ("OP_RETURN".to_string(), offset + 1),
            OpCode::Constant => constant_instruction(chunk, offset),
            OpCode::ConstantLong => constant_long_instruction(chunk, offset),
        };
    }

    return (
        format!("Unknown operator code: {}", instruction),
        offset + 1,
    );
}

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
