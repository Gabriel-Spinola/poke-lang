use std::{io::Read, string::ParseError};

pub type ParseResult = Result<(), ParseError>;

pub fn load(input: impl Read) -> ParseResult {
    Ok(())
}
