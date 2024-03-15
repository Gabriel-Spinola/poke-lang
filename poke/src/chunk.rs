#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    OpReturn,
}

const OP_CODES_MAP: [OpCode; 1] = [OpCode::OpReturn];
pub struct Chunk {
    count: i32,
    capacity: i32,

    code: Vec<u8>,
}

impl Chunk {
    /// Grows by a factor of two
    fn grow_capacity(capacity: i32) -> i32 {
        return if capacity < 8 { 8 } else { capacity * 2 };  
    }
    
    fn disassemble_instruction(&self, offset: usize) -> (String, usize) {
        let instruction = usize::from(self.code[offset]);
        if let Some(operation) = OP_CODES_MAP.get(instruction) {
            return match operation {
                OpCode::OpReturn => ("RETURN".to_string(), offset + 1),
            }
        }

        return (format!("Unknown operator code: {}", instruction), offset + 1);
    }

    pub fn init_chunk() -> Chunk {
        return Chunk {
            capacity: 0,
            count: 0,

            code: Vec::new(),
        };
    }

    pub fn write_chunk(chunk: &mut Chunk, byte: u8) {
        if chunk.capacity < chunk.count + 1 {
            chunk.capacity = Chunk::grow_capacity(chunk.capacity);
            chunk.code.push(byte);
        }

        chunk.code[chunk.count as usize] = byte;
        chunk.count += 1;
    }

    pub fn disassemble_chunk(&self, name: &str) {
        println!("==== Chunk {:?} Disassemble ====", name);

        let mut offset: usize = 0;
        let mut text: String;
        while offset < self.count as usize {
            (text, offset) = self.disassemble_instruction(offset);

            println!("{:04} {}", offset, text);
        }
    }
}
