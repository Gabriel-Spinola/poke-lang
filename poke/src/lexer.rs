// LINK - https://github.com/WuBingzheng/build-lua-in-rust/blob/main/listing/ch09.closure/src/lex.rs

use core::panic;
use std::{
    io::{Bytes, Read},
    iter::Peekable,
    mem,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    And,
    Do,
    Then,
    If,
    Else,
    ElseIf,
    End,
    False,
    True,
    For,
    In,
    Function,
    Mut,
    Nil,
    Not,
    Or,
    While,
    Repeat,
    Return,
    Until,
    Require,
    Break,

    // Operations
    //   +     -   *    /    %    ^    #
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Len,
    //    &       ~       |       <<      >>     //
    BitAnd,
    BitOr,
    BitNot,
    ShiftL,
    ShiftR,
    Idiv,
    //   ==       ~=     <=      >=      <       >        =
    Equal,
    NotEq,
    LesEq,
    GreEq,
    Less,
    Greater,
    Assign,
    //    (       )       {       }       [       ]       ::
    ParL,
    ParR,
    CurlyL,
    CurlyR,
    SqurL,
    SqurR,
    DoubColon,
    //      ;        :       ,      .    <>     ..
    SemiColon,
    Colon,
    Comma,
    Dot,
    Concat,
    Dots,
    Arrow,

    // Data types
    Int,
    Float,
    String,
    Bool,
    Byte,

    Identifier,
    Name(String),
    Numbers,

    // End of line
    Error,
    EoS,
}
pub struct Lexer<R: Read> {
    input: Peekable<Bytes<R>>,
    ahead: Token,
    pub current_line: i32,
}

impl<R: Read> Lexer<R> {
    pub fn new(input: R) -> Self {
        return Lexer {
            input: input.bytes().peekable(),
            ahead: Token::EoS,
            current_line: 0,
        };
    }

    pub fn peek(&mut self) -> &Token {
        if self.ahead == Token::EoS {
            self.ahead = self.advance();
        }

        return &self.ahead;
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

        return match byte_char.unwrap() {
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
            b'-' => {
                self.check_complex_ahead(vec![b'-', b'>'], Token::Sub, Lexer::read_comment_or_arrow)
            }

            // ANCHOR Strings
            b'\'' | b'"' => todo!(), // TODO -

            // ANCHOR - Numbers
            b'0'..=b'9' => todo!(), // TODO -
            b'A'..=b'Z' | b'a'..=b'z' | b'_' => self.read_identifier_or_name(byte_char.unwrap()),

            // ANCHOR - Blank spaces
            b' ' | b'\r' | b'\t' => self.advance(), // Ignore spaces
            b'\n' => {
                self.current_line += 1;
                return self.advance();
            }

            // ANCHOR - INVALID
            _ => panic!("Unexpected Character: {:?}", byte_char.unwrap()),
        };
    }

    /// Advances the interator and return the next character from input stream (as byte)
    fn next_byte_char(&mut self) -> Option<u8> {
        return self.input.next().map(|byte_result| {
            return byte_result.unwrap_or_else(|error| {
                panic!("(lexer) failed to iterate through input stream. {}", error)
            });
        });
    }

    fn peek_byte_char(&mut self) -> u8 {
        return match self.input.peek() {
            Some(Ok(byte_char)) => *byte_char,
            Some(_) => panic!("(lexer) failed to peek error"),
            None => b'\0',
        };
    }

    fn read_comment_or_arrow(&mut self) -> Token {
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

        return self.advance();
    }

    fn read_identifier_or_name(&mut self, byte_char: u8) -> Token {
        let mut name = String::new();
        name.push(byte_char as char);

        loop {
            let character = self.peek_byte_char() as char;

            if !(character.is_alphanumeric() || character == '_') {
                break;
            }

            name.push(character);
            self.next_byte_char();
        }

        // TODO - optimize by hash
        return match &name as &str { 
            "int" => Token::Int,
            "float" => Token::Float,
            "string" => Token::String,
            "bool" => Token::Bool,

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
            _ => Token::Name(name),
        };
    }

    fn check_ahead(&mut self, ahead_char: u8, short_option: Token, long_option: Token) -> Token {
        if self.peek_byte_char() == ahead_char {
            self.next_byte_char();

            return long_option;
        }

        return short_option;
    }

    fn check_ahead_multi_option(
        &mut self,
        ahead_chars: Vec<u8>,
        long_options: Vec<Token>,
        short_option: Token,
    ) -> Token {
        for i in 0..ahead_chars.len() {
            if self.peek_byte_char() == ahead_chars[i] {
                self.next_byte_char();

                return long_options[i].clone();
            }
        }

        return short_option;
    }

    fn check_complex_ahead(
        &mut self,
        ahead_chars: Vec<u8>,
        short_option: Token,
        complex_token_reader: fn(&mut Lexer<R>) -> Token,
    ) -> Token {
        for i in 0..ahead_chars.len() {
            if self.peek_byte_char() == ahead_chars[i] {
                return complex_token_reader(self);
            }
        }

        return short_option;
    }
}
