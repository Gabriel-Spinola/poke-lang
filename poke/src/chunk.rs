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

const OP_CODES_MAP: [OpCode; 2] = [OpCode::Return, OpCode::Constant];

type Value = f64;

pub struct Chunk {
    pub count: i32,
    pub capacity: i32,

    pub code: Vec<u8>,

    ///
    pub constants: Vec<Value>,
}

impl Chunk {
    /// Grows by a factor of two
    fn grow_capacity(capacity: i32) -> i32 {
        return if capacity < 8 { 8 } else { capacity * 2 };
    }

    fn constant_instruction(&self, offset: usize) -> (String, usize) {
        let constant_index: u8 = self.code[offset + 1];
        let constant_value = self.constants[constant_index as usize];
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

    fn disassemble_instruction(&self, offset: usize) -> (String, usize) {
        let instruction = self.code[offset] as usize;

        if let Some(operation) = OP_CODES_MAP.get(instruction) {
            return match operation {
                OpCode::Return => ("OP_RETURN".to_string(), offset + 1),
                OpCode::Constant => self.constant_instruction(offset),
            };
        }

        return (
            format!("Unknown operator code: {}", instruction),
            offset + 1,
        );
    }

    pub fn init_chunk() -> Chunk {
        return Chunk {
            capacity: 0,
            count: 0,

            code: Vec::new(),
            constants: Vec::new(),
        };
    }

    pub fn write_chunk(chunk: &mut Chunk, byte: u8) {
        if chunk.capacity < chunk.count + 1 {
            chunk.capacity = Chunk::grow_capacity(chunk.capacity);
        }

        chunk.code.push(byte);
        chunk.count += 1;
    }

    pub fn disassemble_chunk(&self, name: &str) {
        println!("==== Chunk {:?} Disassemble ====", name);

        let mut offset: usize = 0;
        let mut text: String;
        while offset < self.count as usize {
            (text, offset) = self.disassemble_instruction(offset);

            println!("\t- {:04} {}", offset, text);
        }
    }

    pub fn add_constant(&mut self, constant: Value) -> usize {
        self.constants.push(constant);

        return self.constants.len() - 1;
    }
}
