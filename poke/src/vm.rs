use crate::{
    chunk::{Chunk, OpCode, Value},
    disassemble_instruction,
};

pub enum InterpretResult {
    OK,
    CompilerError,
    RuntimeError,
}

pub struct VirtualMachine<'a> {
    chunk: &'a Chunk,
    // REVIEW - might be a slow solution. In the book a raw pointer is used wich is usafe rust
    /// Holds the index of the current instruction within the bytecode array
    ip: usize,

    stack: Vec<Value>,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        return VirtualMachine {
            chunk,
            ip: 0,
            stack: Vec::new(),
        };
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
        #[cfg(feature = "debug_trace_execution")]
        println!("\n==== VM Logging ====");

        let mut offset;
        let mut text: String;

        loop {
            // prints each instruction right before executing it.
            #[cfg(feature = "debug_trace_execution")]
            {
                for value in &self.stack {
                    println!("STACK [{}]", value);
                }

                (text, offset) = disassemble_instruction(&self.chunk, self.ip);
                println!("{:04} {}", offset, text);
            }

            let instruction: u8 = self.advance_ip(1);
            if let Some(operation) = OpCode::all_variants().get(instruction as usize) {
                return match operation {
                    OpCode::Constant => {
                        let constant: Value = self.chunk.constants[self.advance_ip(1) as usize];
                        self.stack.push(constant);

                        continue;
                    }
                    OpCode::Negate => {
                        if let Some(value) = self.stack.pop() {
                            // Negate the given value
                            self.stack.push(-value);
                        }

                        continue;
                    },
                    OpCode::Return => {
                        // Pop the top
                        if let Some(value) = self.stack.pop() {
                            println!("Popped from stack: {}", value);
                        }

                        return InterpretResult::OK;
                    }
                    _ => InterpretResult::CompilerError,
                };
            }
        }
    }
}
