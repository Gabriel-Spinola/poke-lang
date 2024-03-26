use std::io::Read;

use crate::{
    chunk::{ByteCode, Chunk},
    debug,
    value::ValueType,
};

use super::{
    errors::{ParseError, ParseErrorType},
    lexer::Lexer,
    tokens::Token,
};

pub type ParseResult = Result<Token, ParseError>;

/// Singe-Pass Compilation
/// LINK - https://craftinginterpreters.com/compiling-expressions.html#single-pass-compilation
pub struct Parser<'a, R: Read> {
    pub chunk: &'a mut Chunk,

    lex: Option<Lexer<R>>,
}

impl<'a, R: Read> Parser<'a, R> {
    pub fn new(chunk: &'a mut Chunk) -> Self {
        Parser { chunk, lex: None }
    }

    pub fn load(&mut self, input: R) -> ParseResult {
        self.lex = Some(Lexer::new(input));

        // #[cfg(feature = "debug_trace_lex_execution")]
        // _disassemble_lexer(&mut lexer, "operators");

        let _ = self.advance()?;
        self.chunk.write_chunk(ByteCode::Return as u8, 0);

        #[cfg(feature = "debug_trace_execution")]
        debug::_disassemble_chunk(self.chunk, "parser test");

        Ok(Token::Nil)
    }

    fn advance(&mut self) -> ParseResult {
        loop {
            // REVIEW - I don't know if borrowing as mutable every iteration is a good ideia
            let current_token = self
                .lex
                .as_mut()
                .expect("lex should not be used befored loaded")
                .advance()
                .map_err(|error| ParseError {
                    error: ParseErrorType::LexError { error },
                    line: 0,
                })?;

            if current_token == Token::EoS {
                break;
            }

            // TODO - Line data
            match current_token {
                // ANCHOR - Parse numbers
                Token::Int { value } => self.chunk.write_constant(ValueType::Int(value), 0),
                Token::Float { value } => self.chunk.write_constant(ValueType::Float(value), 0),
                Token::Byte { value } => self.chunk.write_constant(ValueType::Byte(value), 0),

                Token::ParL => self.parse_grouping()?,

                _ => continue,
            }
        }

        Ok(Token::Nil)
    }

    fn consume(&mut self, expected_token: Token) -> ParseResult {
        let current = self
            .lex
            .as_mut()
            .expect("lex should not be used befored loaded")
            .peek()
            .map_err(|error| ParseError {
                error: ParseErrorType::LexError { error },
                line: 0,
            })?;

        if current == &expected_token {
            return self.advance();
        }

        Err(ParseError {
            error: ParseErrorType::UnexpectedToken,
            line: 0,
        })
    }

    fn parse_grouping(&mut self) -> Result<(), ParseError> {
        let _ = self.advance()?;
        let _ = self.consume(Token::ParR)?;

        Ok(())
    }

    fn _parse_expression(&mut self) -> Result<(), ParseError> {
        todo!()
    }
}
