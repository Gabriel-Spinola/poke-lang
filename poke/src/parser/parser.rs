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

enum Precedence {
    None,
    Assignment, // = ->
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // - ~ not #
    Call,       // : . ()
    Primary,
}

// REVIEW - Consider the usage of `&'a dyn Fn(&'a mut Parser<'_, R>)` if more flexibility is needed
type ParseFn<'a, R> = fn(&'a mut Parser<'_, R>) -> Result<(), ParseError>;

struct ParseRule<'a, R: Read> {
    prefix: Option<ParseFn<'a, R>>,
    infix: Option<ParseFn<'a, R>>,

    precedence: Precedence,
}

impl<'a, R: Read> ParseRule<'a, R> {
    pub fn rules(parser: &mut Parser<'_, R>) {
        let a = [(
            Token::ParL,
            ParseRule {
                prefix: Some(|parser: &mut Parser<'_, R>| parser.parse_grouping()),
                infix: None,
                precedence: Precedence::None,
            },
        )];

        let fun = a[0].1.prefix.unwrap();
        let _ = fun(parser);
    }
}

pub type ParseResult = Result<Token, ParseError>;

// Gleam parser source code, in wich also uses precedence for parsing expressions
// LINK - https://github.com/gleam-lang/gleam/blob/main/compiler-core/src/parse.rs#L182

/// Singe-Pass Compilation?
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

        ParseRule::rules(self);

        let _ = self.advance().map_err(|err| {
            self.chunk.write_chunk(ByteCode::Return as u8, 0);

            err
        })?;
        self.chunk.write_chunk(ByteCode::Return as u8, 0);

        #[cfg(feature = "debug_trace_execution")]
        debug::_disassemble_chunk(self.chunk, "parser test");

        Ok(Token::Nil)
    }

    fn advance(&mut self) -> ParseResult {
        loop {
            let current_token = self.advance_lex()?;
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
            .expect("lex should not be used befored loaded")
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
            .expect("lex should not be used befored loaded")
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

    fn _parse_unary(&mut self, current_token: &Token) -> Result<(), ParseError> {
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
}
