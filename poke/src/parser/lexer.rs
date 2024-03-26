// Code I'm taking as reference (stealing)
// LINK - https://github.com/WuBingzheng/build-lua-in-rust/blob/main/listing/ch09.closure/src/lex.rs
// LINK - https://github.com/gleam-lang/gleam/blob/main/compiler-core/src/parse/

use crate::parser::tokens::Token;
use core::panic;
use std::{
    io::{Bytes, Read},
    iter::Peekable,
    mem,
};

use super::errors::{LexicalError, LexicalErrorType};

#[cfg(test)]
#[path = "./tests.rs"]
mod tests;

pub type LexResult = Result<Token, LexicalError>;

// TODO - Implement lexical errors
// LINK - https://github.com/gleam-lang/gleam/blob/main/compiler-core/src/parse/lexer.rs#L19
pub struct Lexer<R: Read> {
    pub current_line: i32,

    input: Peekable<Bytes<R>>,
    ahead: Token,
}

impl<R: Read> Lexer<R> {
    pub fn new(input: R) -> Self {
        Lexer {
            input: input.bytes().peekable(),
            ahead: Token::EoS,
            current_line: 0,
        }
    }

    pub fn peek(&mut self) -> Result<&Token, LexicalError> {
        if self.ahead == Token::EoS {
            self.ahead = self.advance()?;
        }

        Ok(&self.ahead)
    }

    #[cfg(test)]
    pub fn expect(&mut self, expected_token: Token) {
        if let Ok(token) = self.advance() {
            assert_eq!(token, expected_token);
        }

        panic!("No token found")
    }

    pub fn advance(&mut self) -> LexResult {
        // If ahead is not Token::EoS, it means that the next token is already
        // stored in ahead, so it returns that token. Otherwise, it fetches the
        // next token from the input stream and returns it.
        if self.ahead != Token::EoS {
            return Ok(mem::replace(&mut self.ahead, Token::EoS));
        }

        let byte_char = self.next_byte_char();
        if byte_char.is_none() {
            return Ok(Token::EoS);
        }

        match byte_char.unwrap() {
            // ANCHOR - Symbols
            b'+' => Ok(Token::Add),
            b'*' => Ok(Token::Mul),
            b'%' => Ok(Token::Mod),
            b'^' => Ok(Token::Pow),
            b'#' => Ok(Token::Len),
            b'&' => Ok(Token::BitAnd),
            b'|' => Ok(Token::BitOr),
            b'(' => Ok(Token::ParL),
            b')' => Ok(Token::ParR),
            b'{' => Ok(Token::CurlyL),
            b'}' => Ok(Token::CurlyR),
            b'[' => Ok(Token::SqurL),
            b']' => Ok(Token::SqurR),
            b';' => Ok(Token::SemiColon),
            b',' => Ok(Token::Comma),

            b':' => self.check_ahead(b':', Token::Colon, Token::DoubColon),
            b'/' => self.check_ahead(b'/', Token::Div, Token::Idiv),
            b'=' => self.check_ahead(b'=', Token::Equal, Token::Assign),
            b'~' => self.check_ahead(b'=', Token::BitNot, Token::NotEq),

            b'<' => self.check_ahead_multi_option(
                vec![b'=', b'<', b'>'],
                vec![Token::LesEq, Token::ShiftL, Token::Concat],
                Token::Less,
            ),

            b'>' => self.check_ahead_multi_option(
                vec![b'=', b'>'],
                vec![Token::GreEq, Token::ShiftR],
                Token::Greater,
            ),

            // ANCHOR Complex symbols
            b'.' => self.lex_number_or_dots(),
            b'-' => self.check_complex_ahead(vec![b'-', b'>'], Token::Sub, Lexer::lex_dash_symbol),

            // ANCHOR Strings
            b'\'' | b'"' => self.lex_string(byte_char.unwrap()),

            // ANCHOR - Numbers
            b'0'..=b'9' => self.lex_number(byte_char.unwrap()),
            b'A'..=b'Z' | b'a'..=b'z' | b'_' => Ok(self.lex_identifier_or_name(byte_char.unwrap())),

            // ANCHOR - Blank spaces
            b' ' | b'\r' | b'\t' => self.advance(), // Ignore spaces
            b'\n' => self.lex_next_line(),

            // ANCHOR - INVALID
            _ => Err(LexicalError {
                error: LexicalErrorType::UnexpectedToken {
                    token: byte_char.unwrap() as char,
                },

                line: self.current_line + 1,
            }),
        }
    }

    /// Advances the interator and return the next character from input stream (as byte)
    fn next_byte_char(&mut self) -> Option<u8> {
        self.input.next().map(|byte_result| {
            byte_result.unwrap_or_else(|error| {
                panic!("(lexer) failed to iterate through input stream. {}", error)
            })
        })
    }

    fn peek_byte_char(&mut self) -> u8 {
        match self.input.peek() {
            Some(Ok(byte_char)) => *byte_char,
            Some(_) => panic!("(lexer) failed to peek error"),
            None => b'\0',
        }
    }

    fn lex_next_line(&mut self) -> LexResult {
        self.current_line += 1;

        self.advance()
    }

    // TODO - add string interpolation
    fn lex_string(&mut self, quote_character: u8) -> LexResult {
        let mut buffer = String::new();

        loop {
            let next_byte = self.next_byte_char();

            if next_byte.is_none() {
                return Err(LexicalError {
                    error: LexicalErrorType::UnexpectedStringEnd,
                    line: self.current_line + 1,
                });
            }

            match next_byte.unwrap() {
                b'\\' => buffer.push(self.read_scape()? as char), // Push escape

                character if character == quote_character => break, // Close string
                character => buffer.push(character as char),        // Push character
            }
        }

        Ok(Token::String { value: buffer })
    }

    fn read_scape(&mut self) -> Result<u8, LexicalError> {
        let next_byte = self
            .next_byte_char()
            .unwrap_or_else(|| panic!("(lexer) failed to get next byte from string escape"));

        match next_byte {
            b'a' => Ok(0x07),
            b'b' => Ok(0x08),
            b'f' => Ok(0x0c),
            b'v' => Ok(0x0b),
            b'n' => Ok(b'\n'),
            b'r' => Ok(b'\r'),
            b't' => Ok(b'\t'),
            b'"' => Ok(b'"'),

            b'\\' => Ok(b'\\'),
            b'\'' => Ok(b'\''),

            b'x' => self.read_hexadecimal_escape(), // format: \xXX
            character @ b'0'..=b'9' => self.read_decimal_escape(character), // format: \d[d[d]]

            _ => Err(LexicalError {
                error: LexicalErrorType::BadStringEscape,
                line: self.current_line + 1,
            }),
        }
    }

    fn read_hexadecimal_escape(&mut self) -> Result<u8, LexicalError> {
        let hex_digit_1 =
            char::to_digit(self.next_byte_char().expect("invalid format") as char, 16)
                .expect("correct \\x format: first hex digit");
        let hex_digit_2 =
            char::to_digit(self.next_byte_char().expect("invalid format") as char, 16)
                .expect("correct \\x format: second hex digit");

        Ok((hex_digit_1 * 16 + hex_digit_2) as u8)
    }

    fn read_decimal_escape(&mut self, character: u8) -> Result<u8, LexicalError> {
        let mut decimal_value = char::to_digit(character as char, 10)
            .unwrap_or_else(|| panic!("(lexer) failed to convert char to digit: {:?}", character));

        if let Some(digit) = char::to_digit(self.peek_byte_char() as char, 10) {
            let _ = self.next_byte_char();

            decimal_value = decimal_value * 10 + digit;
            if let Some(digit) = char::to_digit(self.peek_byte_char() as char, 10) {
                let _ = self.next_byte_char();

                decimal_value = decimal_value * 10 + digit;
            }
        }

        Ok(u8::try_from(decimal_value).expect("decimal escape too large"))
    }

    fn lex_number(&mut self, current_byte: u8) -> LexResult {
        let next_byte = self.peek_byte_char();

        if current_byte == b'0' {
            // Hex
            if next_byte == b'x' || next_byte == b'X' {
                let _ = self.next_byte_char();
                let _ = self.next_byte_char();

                return self.lex_number_radix(16, "0x");
            }

            // Octal
            if next_byte == b'o' || next_byte == b'O' {
                let _ = self.next_byte_char();
                let _ = self.next_byte_char();

                return self.lex_number_radix(8, "0o");
            }

            if next_byte == b'b' || next_byte == b'B' {
                let _ = self.next_byte_char();
                let _ = self.next_byte_char();

                return self.lex_number_radix(2, "0b");
            }
        }

        self.lex_decimals_or_integers(current_byte)
    }

    fn lex_decimals_or_integers(&mut self, current_byte: u8) -> LexResult {
        let mut is_float = current_byte == b'.';
        let mut is_byte = false;

        let mut buffer = String::new();
        buffer.push(current_byte as char);

        loop {
            let next_byte = self.peek_byte_char() as char;

            match next_byte {
                '0'..='9' => buffer.push(next_byte),
                '.' | 'E' | 'e' | '+' | '-' => {
                    buffer.push(next_byte);

                    is_float = true;
                }
                'b' if !is_float && !self.peek_byte_char().is_ascii_digit() => is_byte = true,
                _ => break,
            }

            self.next_byte_char();
        }

        if is_float {
            let float_value = buffer.parse::<f64>().unwrap_or_else(|error| {
                panic!(
                    "(lexer) failed to parse value {:?} to float. {}",
                    buffer, error
                )
            });

            return Ok(Token::Float { value: float_value });
        }

        if is_byte {
            let byte_value = buffer.parse::<u8>().unwrap_or_else(|error| {
                panic!(
                    "(lexer) failed to parse value {:?} to byte. {}",
                    buffer, error
                )
            });

            return Ok(Token::Byte { value: byte_value });
        }

        let int_value = buffer.parse::<i32>().unwrap_or_else(|error| {
            panic!(
                "(lexer) failed to parse value {:?} to int. {}",
                buffer, error
            )
        });

        Ok(Token::Int { value: int_value })
    }

    // Lex a Hex/Octal/Binary number without a decimal point.
    fn lex_number_radix(&mut self, _radix: u32, _prefix: &str) -> LexResult {
        todo!() // TODO - Implement number radix ref: https://github.com/gleam-lang/gleam/blob/main/compiler-core/src/parse/lexer.rs#L554
    }

    fn lex_number_or_dots(&mut self) -> LexResult {
        let next_byte = self.peek_byte_char();

        if next_byte == b'.' {
            let _ = self.next_byte_char();

            return Ok(Token::Dots);
        }

        if !next_byte.is_ascii_digit() {
            return Ok(Token::Dot);
        }

        self.lex_number(b'.')
    }

    fn lex_dash_symbol(&mut self) -> LexResult {
        let next_byte = self.next_byte_char();
        if next_byte.is_none() {
            return self.advance();
        }

        if next_byte.unwrap() == b'>' {
            return Ok(Token::Arrow);
        }

        while let Some(byte_char) = self.next_byte_char() {
            if byte_char == b'\n' {
                self.current_line += 1;

                break;
            }
        }

        self.advance()
    }

    fn lex_identifier_or_name(&mut self, current_byte: u8) -> Token {
        let mut name = String::new();
        name.push(current_byte as char);

        loop {
            let character = self.peek_byte_char() as char;

            if !(character.is_alphanumeric() || character == '_') {
                break;
            }

            name.push(character);
            let _ = self.next_byte_char();
        }

        // TODO - optimize by hash
        match &name as &str {
            "mut" => Token::Mut,
            "require" => Token::Require,
            "and" => Token::And,
            "break" => Token::Break,
            "do" => Token::Do,
            "else" => Token::Else,
            "elseif" => Token::ElseIf,
            "end" => Token::End,
            "for" => Token::For,
            "function" => Token::Function,
            "if" => Token::If,
            "in" => Token::In,
            "nil" => Token::Nil,
            "not" => Token::Not,
            "or" => Token::Or,
            "repeat" => Token::Repeat,
            "return" => Token::Return,
            "then" => Token::Then,
            "true" => Token::Bool { value: true },
            "false" => Token::Bool { value: false },
            "until" => Token::Until,
            "while" => Token::While,
            _ => Token::Identifier(name),
        }
    }

    fn check_ahead(
        &mut self,
        ahead_char: u8,
        short_option: Token,
        long_option: Token,
    ) -> LexResult {
        if self.peek_byte_char() == ahead_char {
            let _ = self.next_byte_char();

            return Ok(long_option);
        }

        Ok(short_option)
    }

    fn check_ahead_multi_option(
        &mut self,
        ahead_chars: Vec<u8>,
        long_options: Vec<Token>,
        short_option: Token,
    ) -> LexResult {
        for i in 0..ahead_chars.len() {
            if self.peek_byte_char() == ahead_chars[i] {
                let _ = self.next_byte_char();

                return Ok(long_options[i].clone());
            }
        }

        Ok(short_option)
    }

    fn check_complex_ahead(
        &mut self,
        ahead_chars: Vec<u8>,
        short_option: Token,
        complex_token_reader: fn(&mut Lexer<R>) -> LexResult,
    ) -> LexResult {
        for ahead_char in ahead_chars {
            if self.peek_byte_char() == ahead_char {
                return complex_token_reader(self);
            }
        }

        Ok(short_option)
    }
}
