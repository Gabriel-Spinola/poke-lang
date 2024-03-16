use crate::chunk::{Chunk, OpCode, OP_CODES_MAP};

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
    }
    else {
        print!("{:04} ", chunk.lines[offset]);
    }
    
    let instruction = chunk.code[offset] as usize;

    if let Some(operation) = OP_CODES_MAP.get(instruction) {
        return match operation {
            OpCode::Return => ("OP_RETURN".to_string(), offset + 1),
            OpCode::Constant => constant_instruction(chunk, offset),
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