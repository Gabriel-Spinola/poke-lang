use crate::chunk::{Chunk, OpCode, OP_CODES_MAP};

pub enum InterpretResult {
    OK,
    CompilerError,
    RuntimeError,
}

pub struct VirtualMachine<'a> {
    chunk: &'a Chunk,

    /// Holds the index of the current instruction within the bytecode array
    ip: usize,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        return VirtualMachine { chunk, ip: 0 };
    }

    fn peek_current_instruction(&self) -> &u8 {
        return self
            .chunk
            .code
            .get(self.ip)
            .expect("Failed to peek into chunk instructions");
    }

    fn advance_ip(&mut self, offset: usize) -> &u8 {
        self.ip += offset;

        return self.peek_current_instruction();
    }

    fn interpret(&mut self, chunk: &'a Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.ip = chunk.code.len() - 1;

        return self.run_interpreter();
    }

    pub fn run_interpreter(&mut self) -> InterpretResult {
        loop {
            let instruction: &u8 = self.advance_ip(1);

            if let Some(operation) = OP_CODES_MAP.get(*instruction as usize) {
                return match operation {
                    OpCode::Return => InterpretResult::OK,
                    _ => todo!(),
                };
            }
        }
    }
}
