#[derive(Debug)]
pub enum LexicalErrorType {
    BadStringEscape,
    UnexpectedStringEnd,
}

#[derive(Debug)]
pub struct LexicalError {
    pub error: LexicalErrorType,
    pub line: i32,
}

pub enum ParseError {
    LexError { error: LexicalError },
}
