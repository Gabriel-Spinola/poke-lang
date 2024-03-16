use crate::{
    chunk::{Chunk, OpCode, Value, OP_CODES_MAP},
    disassemble_chunk, disassemble_instruction,
};

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

    fn advance_ip(&mut self, offset: usize) -> u8 {
        let val = self.peek_current_instruction().clone();
        self.ip += offset;

        return val;
    }

    pub fn run_interpreter(&mut self) -> InterpretResult {
        loop {
            // prints each instruction right before executing it.
            #[cfg(feature = "debug_trace_execution")]
            {
                disassemble_instruction(&self.chunk, self.ip);
            }

            let instruction: u8 = self.advance_ip(1);
            if let Some(operation) = OP_CODES_MAP.get(instruction as usize) {
                return match operation {
                    OpCode::Constant => {
                        let constant: Value = self.chunk.constants[self.advance_ip(1) as usize];
                        println!("CONSTANT VALUE {:?}", constant);

                        continue;
                    }
                    OpCode::Return => InterpretResult::OK,
                    _ => InterpretResult::CompilerError,
                };
            }
        }
    }
}
