use super::tokens::Token;

#[derive(Debug)]
pub enum LexicalErrorType {
    BadStringEscape,
    UnexpectedStringEnd,
    UnexpectedToken { token: char },
}

#[derive(Debug)]
pub struct LexicalError {
    pub error: LexicalErrorType,
    pub line: i32,
}

#[derive(Debug)]
pub enum ParseErrorType {
    LexError { error: LexicalError },
    UnexpectedToken { token: Token },
}

#[derive(Debug)]
pub struct ParseError {
    pub error: ParseErrorType,
    pub line: i32,
}
