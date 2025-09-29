use crate::lexer::{error::LexerError, token::Token};

#[derive(Debug)]
pub enum ParserError {
    LexerError(LexerError),
    UnexpectedToken(Token),
    UnexpectedEnd,
    InvalidAssignment,
}

impl From<LexerError> for ParserError {
    fn from(value: LexerError) -> Self {
        Self::LexerError(value)
    }
}
