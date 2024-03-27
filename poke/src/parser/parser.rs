// Gleam parser source code, in wich also uses precedence for parsing expressions
// LINK - https://github.com/gleam-lang/gleam/blob/main/compiler-core/src/parse.rs#L182
// Singe-Pass Compilation
// LINK - https://craftinginterpreters.com/compiling-expressions.html#single-pass-compilation

#[path = "./rules.rs"]
pub mod rules;

use self::rules::ParseRule;

use super::{
    errors::{ParseError, ParseErrorType},
    lexer::Lexer,
    tokens::Token,
};
use crate::{
    chunk::{ByteCode, Chunk},
    debug,
    value::ValueType,
};
use std::io::Read;

pub type ParseResult = Result<Token, ParseError>;

/// Represents a parsing function used by the parser.
/// Takes a mutable reference to the parser and an optional reference to the current token being parsed.
/// Returns a result indicating success or a parse error.
///
/// NOTE - Consider the usage of `&'a dyn Fn(&'a mut Parser<'_, R>)` if more flexibility is needed
pub type ParseFn<'a, R> = fn(&'a mut Parser<'_, R>, Option<&'a Token>) -> Result<(), ParseError>;

/// REVIEW - maybe we should just generete the chunk here instead of borrowing
/// TODO - Write line data
pub struct Parser<'a, R: Read> {
    pub chunk: &'a mut Chunk,

    lex: Option<Lexer<R>>,
}

impl<'a, R: Read> Parser<'a, R> {
    pub fn new(chunk: &'a mut Chunk) -> Self {
        Parser { chunk, lex: None }
    }

    /// Run the parser (for now)
    pub fn load(&mut self, input: R) -> ParseResult {
        self.lex = Some(Lexer::new(input));

        // #[cfg(feature = "debug_trace_lex_execution")]
        // _disassemble_lexer(&mut lexer, "operators");

        // NOTE - If error found: stop compiling and then propagate error
        self.advance().map_err(|err| {
            self.finish_code_execution(0);

            err
        })?;

        self.finish_code_execution(0);

        #[cfg(feature = "debug_trace_execution")]
        debug::_disassemble_chunk(self.chunk, "parser test");

        Ok(Token::Nil)
    }

    fn finish_code_execution(&mut self, line: i32) {
        self.chunk.write_chunk(ByteCode::Return as u8, line)
    }

    fn advance(&mut self) -> ParseResult {
        loop {
            let current_token = self.advance_lex()?;
            if current_token == Token::EoS {
                break;
            }

            match current_token {
                // ANCHOR - Parse numbers
                Token::Int { value } => self.chunk.write_constant(ValueType::Int(value), 0),
                Token::Float { value } => self.chunk.write_constant(ValueType::Float(value), 0),
                Token::Byte { value } => self.chunk.write_constant(ValueType::Byte(value), 0),

                Token::ParL => {
                    let rule = rules::get_rule(Token::ParL.to_rule().unwrap());
                    let prefix = rule.prefix.unwrap();
                    prefix(self, None)?;
                }
                _ => continue,
            }
        }

        Ok(Token::Nil)
    }

    fn consume(&mut self, expected_token: Token) -> ParseResult {
        let next_token = self.peek_into_lex()?;
        if next_token == &expected_token {
            return self.advance();
        }

        Err(ParseError {
            error: ParseErrorType::UnexpectedToken {
                token: next_token.clone(),
            },
            line: 0,
        })
    }

    // REVIEW - I don't know if borrowing as mutable every iteration is a good ideia
    fn peek_into_lex(&mut self) -> Result<&Token, ParseError> {
        self.lex
            .as_mut()
            .expect("lex should not be used before loaded")
            .peek()
            .map_err(|error| ParseError {
                error: ParseErrorType::LexError { error },
                line: 0,
            })
    }

    // REVIEW - I don't know if borrowing as mutable every iteration is a good ideia
    fn advance_lex(&mut self) -> Result<Token, ParseError> {
        self.lex
            .as_mut()
            .expect("lex should not be used before loaded")
            .advance()
            .map_err(|error| ParseError {
                error: ParseErrorType::LexError { error },
                line: 0,
            })
    }

    fn _parse_expression(&mut self) -> Result<(), ParseError> {
        todo!()
    }

    fn _parse_prefix_expression(&mut self) -> Result<(), ParseError> {
        todo!()
    }

    fn parse_grouping(&mut self) -> Result<(), ParseError> {
        //self._parse_expression()?;
        self.consume(Token::ParR)?;
        //self.advance()?;
        Ok(())
    }

    fn parse_unary_op(&mut self, current_token: &Token) -> Result<(), ParseError> {
        // NOTE - Compile operand
        self._parse_expression()?;

        match current_token {
            Token::Sub => {
                self.chunk.write_chunk(ByteCode::Negate as u8, 0);

                Ok(())
            }
            _ => todo!(), // unreachable
        }
    }

    fn parse_binary_op(&mut self, _current_token: &Token) -> Result<(), ParseError> {
        todo!()
    }
}
