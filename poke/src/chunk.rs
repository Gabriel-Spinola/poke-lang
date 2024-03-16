// LINK - https://craftinginterpreters.com/chunks-of-bytecode.html

#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    /// Single byte instruction
    Return,

    /// 2 byte instruction.
    /// 1st: opcode, 2nd: constant index
    Constant,
}

impl OpCode {
    pub fn to_byte(&self) -> u8 {
        return match self {
            OpCode::Return => 0,
            OpCode::Constant => 1,
        };
    }
}

pub enum Data {}

pub const OP_CODES_MAP: [OpCode; 2] = [OpCode::Return, OpCode::Constant];

type Value = f64;

pub struct Chunk {
    pub count: i32,
    pub capacity: i32,

    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<i32>,
}

impl Chunk {
    /// Grows by a factor of two
    fn grow_capacity(capacity: i32) -> i32 {
        return if capacity < 8 { 8 } else { capacity * 2 };
    }

    pub fn init_chunk() -> Chunk {
        return Chunk {
            capacity: 0,
            count: 0,

            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        };
    }

    pub fn write_chunk(chunk: &mut Chunk, byte: u8, line: i32) {
        if chunk.capacity < chunk.count + 1 {
            chunk.capacity = Chunk::grow_capacity(chunk.capacity);
        }

        chunk.code.push(byte);
        chunk.lines.push(line);

        chunk.count += 1;
    }

    pub fn add_constant(&mut self, constant: Value) -> usize {
        self.constants.push(constant);

        return self.constants.len() - 1;
    }
}
