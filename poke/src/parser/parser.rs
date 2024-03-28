// Gleam parser source code, in wich also uses precedence for parsing expressions
// LINK - https://github.com/gleam-lang/gleam/blob/main/compiler-core/src/parse.rs#L182

#[path = "./rules.rs"]
pub mod rules;

use self::rules::{ParseRule, Precedence};

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
/// Takes a mutable reference to the parser
/// Returns a result indicating success or a parse error.
///
/// NOTE - Consider the usage of `&'a dyn Fn(&'a mut Parser<'_, R>)` if more flexibility is needed
pub type ParseFn<'a, R> = fn(&'a mut Parser<'_, R>) -> Result<(), ParseError>;

/// # Singe-Pass Compilation
/// LINK - https://craftinginterpreters.com/compiling-expressions.html#single-pass-compilation
///
/// REVIEW - maybe we should just generete the chunk here instead of borrowing
///
/// TODO - Write line data
pub struct Parser<'a, R: Read> {
    pub chunk: &'a mut Chunk,

    lex: Option<Lexer<R>>,
    previus_token: Token,
    current_token: Token,
}

impl<'a, R: Read> Parser<'a, R> {
    pub fn new(chunk: &'a mut Chunk) -> Self {
        Parser {
            chunk,
            lex: None,
            previus_token: Token::EoS,
            current_token: Token::EoS,
        }
    }

    /// Run the parser (for now)
    pub fn load(&mut self, input: R) -> ParseResult {
        self.lex = Some(Lexer::new(input));

        // #[cfg(feature = "debug_trace_lex_execution")]
        // _disassemble_lexer(&mut lexer, "operators");

        // NOTE - If error found: stop compiling and then propagate error
        self.advance().map_err(|err| {
            self.finish_code_execution(0);

            #[cfg(feature = "debug_trace_execution")]
            debug::_disassemble_chunk(self.chunk, "parser test");

            err
        })?;

        self.parse_expression()?;
        self.consume(Token::EoS)?;

        self.finish_code_execution(0);

        #[cfg(feature = "debug_trace_execution")]
        debug::_disassemble_chunk(self.chunk, "parser test");

        Ok(Token::Nil)
    }

    fn finish_code_execution(&mut self, line: i32) {
        self.chunk.write_chunk(ByteCode::Return as u8, line)
    }

    fn advance(&mut self) -> Result<(), ParseError> {
        self.previus_token = self.current_token.clone();
        self.current_token = self.advance_lex()?;

        Ok(())
    }

    fn consume(&mut self, expected_token: Token) -> Result<(), ParseError> {
        if self.current_token == expected_token {
            return self.advance();
        }

        Err(ParseError {
            error: ParseErrorType::UnexpectedToken {
                token: self.current_token.clone(),
            },
            line: 0,
        })
    }

    // REVIEW - I don't know if borrowing as mutable every iteration is a good ideia
    fn _peek_into_lex(&mut self) -> Result<&Token, ParseError> {
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

    fn parse_precedence(&mut self, precedence: u8) -> Result<(), ParseError> {
        self.advance()?;
        let previous_tok_rule: &ParseRule<'_, R> = rules::get_rule(&self.previus_token);
        let prefix = previous_tok_rule.prefix;

        println!("RUN PREFIX FROM: {:?}", self.previus_token);
        prefix.map_or_else(
            || Err(ParseError::new(ParseErrorType::ExpectedExpression, 156)),
            |prefix_fn| prefix_fn(self),
        )?;

        let mut current_tok_rule: &ParseRule<'_, R> = rules::get_rule(&self.current_token);
        while precedence <= current_tok_rule.precedence as u8 {
            self.advance()?;

            current_tok_rule = rules::get_rule(&self.current_token);

            println!("{:?}", self.previus_token);
            println!(
                "precende {:} <= ({:?}) Precendece of: {:?}",
                precedence, self.previus_token, current_tok_rule.precedence as u8
            );

            println!("RUN INFIX FROM: {:?}", self.previus_token);
            let infix = rules::get_rule(&self.previus_token).infix;
            if let Some(infix_fn) = infix {
                infix_fn(self)?;
            }
        }

        Ok(())
    }

    fn parse_expression(&mut self) -> Result<(), ParseError> {
        self.parse_precedence(Precedence::Assignment as u8)
    }

    fn _parse_prefix_expression(&mut self) -> Result<(), ParseError> {
        todo!()
    }

    fn parse_grouping(&mut self) -> Result<(), ParseError> {
        println!("Started grouping at: {:?}", self.current_token);
        self.parse_expression()?;
        self.consume(Token::ParR)?;

        Ok(())
    }

    fn parse_unary_op(&mut self) -> Result<(), ParseError> {
        let sufix_operator = self.previus_token.clone();

        // Compile operand
        self.parse_precedence(Precedence::Unary as u8)?;

        match sufix_operator {
            Token::Sub => {
                self.chunk.write_chunk(ByteCode::Negate as u8, 0);

                Ok(())
            }
            Token::Len => todo!(),
            Token::Not => todo!(),
            Token::BitNot => todo!(),
            _ => Ok(()), // unreachable
        }
    }

    fn parse_binary_op(&mut self) -> Result<(), ParseError> {
        let operator = self.previus_token.clone();
        let rule: &ParseRule<'_, R> = rules::get_rule(&self.previus_token);

        self.parse_precedence(rule.precedence as u8 + 1)?;

        match operator {
            Token::Add => self.chunk.write_chunk(ByteCode::Add as u8, 0),
            Token::Sub => self.chunk.write_chunk(ByteCode::Subtract as u8, 0),
            Token::Mul => self.chunk.write_chunk(ByteCode::Multiply as u8, 0),
            Token::Div => self.chunk.write_chunk(ByteCode::Divide as u8, 0),

            _ => return Ok(()),
        };

        Ok(())
    }

    fn parse_number(&mut self) -> Result<(), ParseError> {
        match self.previus_token {
            Token::Int { value } => self.chunk.write_constant(ValueType::Int(value), 0),
            Token::Float { value } => self.chunk.write_constant(ValueType::Float(value), 0),
            Token::Byte { value } => self.chunk.write_constant(ValueType::Byte(value), 0),

            _ => return Ok(()),
        }

        Ok(())
    }
}
