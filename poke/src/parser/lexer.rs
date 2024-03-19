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

pub struct Lexer<R: Read> {
    input: Peekable<Bytes<R>>,
    ahead: Token,
    pub current_line: i32,
}

impl<R: Read> Lexer<R> {
    pub fn new(input: R) -> Self {
        Lexer {
            input: input.bytes().peekable(),
            ahead: Token::EoS,
            current_line: 0,
        }
    }

    pub fn peek(&mut self) -> &Token {
        if self.ahead == Token::EoS {
            self.ahead = self.advance();
        }

        &self.ahead
    }

    pub fn expect(&mut self, expected_token: Token) {
        assert_eq!(self.advance(), expected_token);
    }

    pub fn advance(&mut self) -> Token {
        // If ahead is not Token::EoS, it means that the next token is already
        // stored in ahead, so it returns that token. Otherwise, it fetches the
        // next token from the input stream and returns it.
        if self.ahead != Token::EoS {
            return mem::replace(&mut self.ahead, Token::EoS);
        }

        let byte_char = self.next_byte_char();
        if byte_char.is_none() {
            return Token::EoS;
        }

        match byte_char.unwrap() {
            // ANCHOR - Symbols
            b'+' => Token::Add,
            b'*' => Token::Mul,
            b'%' => Token::Mod,
            b'^' => Token::Pow,
            b'#' => Token::Len,
            b'&' => Token::BitAnd,
            b'|' => Token::BitOr,
            b'(' => Token::ParL,
            b')' => Token::ParR,
            b'{' => Token::CurlyL,
            b'}' => Token::CurlyR,
            b'[' => Token::SqurL,
            b']' => Token::SqurR,
            b';' => Token::SemiColon,
            b',' => Token::Comma,

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

            b'.' => self.check_ahead(b'.', Token::Dot, Token::Dots), // TODO - This should be complex ahead for considering decimal numbers
            b'-' => self.check_complex_ahead(vec![b'-', b'>'], Token::Sub, Lexer::lex_dash_symbol),

            // ANCHOR Strings
            b'\'' | b'"' => todo!(), // TODO -

            // ANCHOR - Numbers
            b'0'..=b'9' => todo!(), // TODO -
            b'A'..=b'Z' | b'a'..=b'z' | b'_' => self.lex_identifier_or_name(byte_char.unwrap()),

            // ANCHOR - Blank spaces
            b' ' | b'\r' | b'\t' => self.advance(), // Ignore spaces
            b'\n' => {
                self.current_line += 1;

                self.advance()
            }

            // ANCHOR - INVALID
            _ => panic!("Unexpected Character: {:?}", byte_char.unwrap()),
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

    fn lex_string(&mut self, byte_char: u8) -> Token {
        todo!()
    }

    fn lex_number(&mut self, byte_char: u8) -> Token {
        let next_byte = self.peek_byte_char();

        if byte_char == b'0' {
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

        self.lex_decimals()
    }

    fn lex_decimals(&mut self) -> Token {
        todo!()
    }

    // Lex a Hex/Octal/Binary number without a decimal point.
    fn lex_number_radix(&mut self, radix: u32, prefix: &str) -> Token {
        todo!()
    }

    fn lex_dash_symbol(&mut self) -> Token {
        let next_byte = self.next_byte_char();
        if next_byte.is_none() {
            return self.advance();
        }

        if next_byte.unwrap() == b'>' {
            return Token::Arrow;
        }

        while let Some(byte_char) = self.next_byte_char() {
            if byte_char == b'\n' {
                self.current_line += 1;

                break;
            }
        }

        self.advance()
    }

    fn lex_identifier_or_name(&mut self, byte_char: u8) -> Token {
        let mut name = String::new();
        name.push(byte_char as char);

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
            "false" => Token::False,
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
            "true" => Token::True,
            "until" => Token::Until,
            "while" => Token::While,
            _ => Token::Identifier(name),
        }
    }

    fn check_ahead(&mut self, ahead_char: u8, short_option: Token, long_option: Token) -> Token {
        if self.peek_byte_char() == ahead_char {
            let _ = self.next_byte_char();

            return long_option;
        }

        short_option
    }

    fn check_ahead_multi_option(
        &mut self,
        ahead_chars: Vec<u8>,
        long_options: Vec<Token>,
        short_option: Token,
    ) -> Token {
        for i in 0..ahead_chars.len() {
            if self.peek_byte_char() == ahead_chars[i] {
                let _ = self.next_byte_char();

                return long_options[i].clone();
            }
        }

        short_option
    }

    fn check_complex_ahead(
        &mut self,
        ahead_chars: Vec<u8>,
        short_option: Token,
        complex_token_reader: fn(&mut Lexer<R>) -> Token,
    ) -> Token {
        for ahead_char in ahead_chars {
            if self.peek_byte_char() == ahead_char {
                return complex_token_reader(self);
            }
        }

        short_option
    }
}
