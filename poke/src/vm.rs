use crate::chunk::{ByteCode, Chunk, ValueType};

#[cfg(feature = "debug_trace_execution")]
use crate::debug::disassemble_instruction;

use std::ops;

#[derive(Debug)]
pub enum InterpretError {
    CompilerError,
    RuntimeError,
}

pub type InterpretResult = Result<(), InterpretError>;

pub struct VirtualMachine<'a> {
    chunk: &'a Chunk,
    // REVIEW - might be a slow solution. In the book a raw pointer is used wich is usafe rust
    /// Holds the index of the current instruction within the bytecode array
    ip: usize,

    stack: Vec<ValueType>,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        VirtualMachine {
            chunk,
            ip: 0,
            stack: Vec::new(),
        }
    }

    fn peek_current_instruction(&self) -> &u8 {
        self.chunk
            .code
            .get(self.ip)
            .expect("Failed to peek into chunk instructions")
    }

    fn advance_ip(&mut self, offset: usize) -> u8 {
        let val = *self.peek_current_instruction();
        self.ip += offset;

        val
    }

    fn binary_op(&mut self, op: fn(ValueType, ValueType) -> ValueType) -> Option<InterpretResult> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        let op_result = op(a, b);

        self.stack.push(op_result);

        Some(Ok(()))
    }

    // TODO - Make return Result with custom interpret errors
    pub fn run_interpreter(&mut self) -> InterpretResult {
        #[cfg(feature = "debug_trace_execution")]
        {
            println!("\n==== Stack Trace ====");
        }

        let mut offset: usize;
        let mut text: String;

        loop {
            // prints each instruction right before executing it.
            #[cfg(feature = "debug_trace_execution")]
            {
                for value in &self.stack {
                    println!("STACK [{:?}]", value);
                }

                println!("-");
                (text, offset) = disassemble_instruction(self.chunk, self.ip);
                println!("{:04} {}", offset, text);
            }

            let instruction: u8 = self.advance_ip(1);
            let operation = ByteCode::all_variants().get(instruction as usize);
            if operation.is_none() {
                panic!("(vm) instruction not found: {}", instruction)
            }

            return match operation.unwrap() {
                ByteCode::Constant => {
                    let constant: ValueType = self.chunk.constants[self.advance_ip(1) as usize];
                    self.stack.push(constant);

                    continue;
                }
                ByteCode::Negate => {
                    if let Some(value) = self.stack.pop() {
                        // Negate the given value
                        self.stack.push(-value);
                    }

                    continue;
                }

                ByteCode::Add => {
                    let _ = self
                        .binary_op(ops::Add::add)
                        .unwrap_or(Err(InterpretError::RuntimeError));

                    continue;
                }
                ByteCode::Subtract => {
                    let _ = self
                        .binary_op(ops::Sub::sub)
                        .unwrap_or(Err(InterpretError::RuntimeError));

                    continue;
                }
                ByteCode::Multiply => {
                    let _ = self
                        .binary_op(ops::Mul::mul)
                        .unwrap_or(Err(InterpretError::RuntimeError));

                    continue;
                }
                ByteCode::Divide => {
                    let _ = self
                        .binary_op(ops::Div::div)
                        .unwrap_or(Err(InterpretError::RuntimeError));

                    continue;
                }

                ByteCode::Return => {
                    // Pop the top
                    if let Some(value) = self.stack.pop() {
                        println!("Popped from stack: {}", value);
                    }

                    return Ok(());
                }

                _ => Err(InterpretError::CompilerError),
            };
        }
    }
}

// TODO - write vm tests
