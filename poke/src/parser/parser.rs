use std::io::Read;

use crate::{
    chunk::{ByteCode, Chunk, ValueType},
    debug,
};

use super::{
    errors::{ParseError, ParseErrorType},
    lexer::Lexer,
    tokens::Token,
};

pub type ParseResult = Result<Token, ParseError>;

/// Singe-Pass Compilation
/// LINK - https://craftinginterpreters.com/compiling-expressions.html#single-pass-compilation
pub struct Parser<'a> {
    pub chunk: &'a mut Chunk,
}

impl<'a> Parser<'a> {
    pub fn new(chunk: &'a mut Chunk) -> Self {
        Parser { chunk }
    }

    pub fn load(&mut self, input: impl Read) -> ParseResult {
        let mut lexer = Lexer::new(input);

        // #[cfg(feature = "debug_trace_lex_execution")]
        // _disassemble_lexer(&mut lexer, "operators");

        let mut current_token: Token;
        loop {
            let current = lexer.advance();
            if let Err(error) = current {
                return Err(ParseError {
                    error: ParseErrorType::LexError { error },
                    line: 0,
                });
            }

            current_token = current.unwrap();
            if current_token == Token::EoS {
                break;
            }

            if let Token::Int { value } = current_token {
                self.chunk.write_constant(ValueType::Int(value), 0);
            }

            println!("{:?}", current_token);
        }

        self.chunk.write_chunk(ByteCode::Return as u8, 0);

        #[cfg(feature = "debug_trace_execution")]
        debug::_disassemble_chunk(self.chunk, "parser test");

        Ok(Token::Nil)
    }
}
