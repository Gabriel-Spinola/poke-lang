use crate::{
    chunk::{ByteCode, Chunk},
    value::ValueType,
};

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

    // FIXME - binary operation automatically conver type to float
    fn binary_op(&mut self, op: fn(f64, f64) -> f64) -> Option<InterpretResult> {
        let left = match self.stack.pop()? {
            ValueType::Float(value) => value,
            ValueType::Int(value) => value as f64,
            ValueType::Byte(value) => value as f64,
            ValueType::Nil => todo!(),
        };

        let right = match self.stack.pop()? {
            ValueType::Float(value) => value,
            ValueType::Int(value) => value as f64,
            ValueType::Byte(value) => value as f64,
            ValueType::Nil => todo!(),
        };

        let op_result = op(right, left);

        self.stack.push(ValueType::Float(op_result));

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
                        self.stack.push(
                            value
                                .negate()
                                .unwrap_or_else(|err| panic!("failed to negate value: {:?}", err)),
                        );
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
                    return Ok(());
                }

                _ => Err(InterpretError::CompilerError),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::_disassemble_chunk;

    #[test]
    fn test_binary_unary_operations() {
        let mut chunk = Chunk::new();

        chunk.write_constant(ValueType::Float(6.2), 128);
        chunk.write_chunk(ByteCode::Negate as u8, 2);

        chunk.write_constant(ValueType::Float(1.0), 132);
        chunk.write_chunk(ByteCode::Add as u8, 132);

        chunk.write_constant(ValueType::Float(5.0), 132);
        chunk.write_constant(ValueType::Float(1.0), 132);
        chunk.write_chunk(ByteCode::Subtract as u8, 132);

        chunk.write_constant(ValueType::Float(4.2), 132);
        chunk.write_constant(ValueType::Float(3.0), 132);
        chunk.write_chunk(ByteCode::Multiply as u8, 132);

        chunk.write_constant(ValueType::Float(4.0), 132);
        chunk.write_constant(ValueType::Float(0.5), 132);
        chunk.write_chunk(ByteCode::Divide as u8, 132);

        chunk.write_chunk(ByteCode::Return as u8, 123);

        #[cfg(feature = "debug_trace_execution")]
        _disassemble_chunk(&chunk, "test chunk");

        let mut vm = VirtualMachine::new(&chunk);
        vm.run_interpreter()
            .unwrap_or_else(|error| panic!("VM failed: {:?}", error));

        assert_eq!(vm.stack.pop(), Some(ValueType::Float(8.0))); // Divide
        assert_eq!(vm.stack.pop(), Some(ValueType::Float(12.600000000000001))); // Multiply
        assert_eq!(vm.stack.pop(), Some(ValueType::Float(4.0))); // Subtract
        assert_eq!(vm.stack.pop(), Some(ValueType::Float(-5.2))); // Add & Negate
    }
}
