// LINK - https://craftinginterpreters.com/chunks-of-bytecode.html

use std::collections::HashMap;

#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    /// Single byte instruction.
    ///
    /// Represents the `OP_RETURN` instruction, which indicates the end of a function or method.
    Return,

    /// 2 byte instruction.
    ///
    /// Represents the `OP_CONSTANT` instruction, which loads a constant value from the constant pool.
    /// - 1: Opcode (`OP_CONSTANT`)
    /// - 2: Index of the constant in the constant pool
    Constant,

    /// 4 bytes instruction.
    ///
    /// Represents the `OP_CONSTANT_LONG` instruction, which loads a constant value from the constant pool using a 24-bit index.
    /// - 1: Opcode (`OP_CONSTANT_LONG`)
    /// - 2: Lowest byte of the index
    /// - 3: Middle byte of the index
    /// - 4: Highest byte of the index
    ConstantLong,
}

impl OpCode {
    pub fn to_byte(&self) -> u8 {
        return match self {
            OpCode::Return => 0,
            OpCode::Constant => 1,
            OpCode::ConstantLong => 2,
        };
    }
}

pub const OP_CODES_MAP: [OpCode; 3] = [OpCode::Return, OpCode::Constant, OpCode::ConstantLong];

type Value = f64;

pub struct Chunk {
    pub count: i32,
    pub capacity: i32,

    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: HashMap<i32, Vec<usize>>,
}

impl Chunk {
    /// Grows by a factor of two
    fn grow_capacity(capacity: i32) -> i32 {
        return if capacity < 8 { 8 } else { capacity * 2 };
    }

    fn add_constant(&mut self, constant: Value) -> usize {
        self.constants.push(constant);

        return self.constants.len() - 1;
    }

    fn write_line(&mut self, new_line: i32, instruction_index: usize) {
        return self
            .lines
            .entry(new_line)
            .or_insert_with(Vec::new)
            .push(instruction_index);
    }

    pub fn init_chunk() -> Chunk {
        return Chunk {
            capacity: 0,
            count: 0,

            code: Vec::new(),
            constants: Vec::new(),
            lines: HashMap::new(),
        };
    }

    pub fn write_chunk(&mut self, byte: u8, new_line: i32) {
        if self.capacity < self.count + 1 {
            self.capacity = Chunk::grow_capacity(self.capacity);
        }

        self.code.push(byte);
        self.write_line(new_line, self.code.len() - 1);

        self.count += 1;
    }

    pub fn write_constant(&mut self, constant: Value, line: i32) {
        let constant_index = self.add_constant(constant);

        if constant_index < 256 {
            self.write_chunk(OpCode::Constant.to_byte(), line);
            self.write_chunk(constant_index as u8, line);

            return;
        }

        self.write_chunk(OpCode::Constant.to_byte(), line);
        self.write_chunk((constant_index & 0xFF) as u8, line); // Lower 8 bits
        self.write_chunk(((constant_index >> 8) & 0xFF) as u8, line); // Next 8 bits
        self.write_chunk(((constant_index >> 16) & 0xFF) as u8, line); // Upper 8 bits
    }

    pub fn get_line(&self, instruction_index: &usize) -> Option<&i32> {
        for (line, instructions) in &self.lines {
            if instructions.contains(instruction_index) {
                return Some(line);
            }
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_constant_small_index() {
        let mut chunk = Chunk::init_chunk();
        let value = 42.0;
        let instructions_count = 2;

        chunk.write_constant(value, 1);

        // Verify that the correct bytecode instructions are written
        assert_eq!(
            chunk.code,
            vec![OpCode::Constant.to_byte(), 0],
            "Incorrect bytecode instructions for OpCode::Constant"
        );

        // Verify chunk bytes count
        assert_eq!(
            chunk.count, instructions_count,
            "Incorrect number of bytes in the chunk"
        );

        // Verify constant added to constants array
        assert_eq!(
            chunk.constants,
            vec![value],
            "Constant not correctly added to constants array"
        );
    }

    #[test]
    fn test_write_constant_large_index() {
        let mut chunk = Chunk::init_chunk();
        let instructions_count = 4;
        let small_const_size = 256;

        // Write small index constants
        for i in 0..small_const_size {
            chunk.write_constant(i as Value, 1);
        }

        // Write and test large index constants
        for i in 0..4 {
            chunk.write_constant(i as Value, 1);

            assert_eq!(
                chunk.count as usize,
                (instructions_count / 2 * small_const_size) + ((i + 1) * instructions_count),
                "Incorrect total count of bytes in the chunk"
            );
        }
    }

    // TODO implement tests
    #[test]
    fn test_lines() {
        let mut chunk = Chunk::init_chunk();

        chunk.write_chunk(OpCode::Return.to_byte(), 123);
        chunk.write_constant(1.2, 123);
        chunk.write_constant(1.2, 123);
        chunk.write_constant(1.2, 123);
        chunk.write_constant(1.2, 128);
        chunk.write_constant(1.2, 182);
    
        println!("LINES LENGTH {}", chunk.lines.len());

        // test for the correct amount of lines
        // test for correct instructions stored
    }
}
