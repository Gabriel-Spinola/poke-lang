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

pub enum ParseErrorType {
    LexError { error: LexicalError },
}

pub struct ParseError {
    pub error: ParseErrorType,
    pub line: i32,
}
