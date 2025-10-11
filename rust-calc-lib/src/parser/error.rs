use crate::lexer::{error::LexerError, token::Token};
use crate::numeric::NumericValue;

#[derive(Debug)]
pub enum ParserError<N: NumericValue> {
    LexerError(LexerError),
    UnexpectedToken(Token<N>),
    UnexpectedEnd,
    InvalidAssignment,
}

impl<N: NumericValue> From<LexerError> for ParserError<N> {
    fn from(value: LexerError) -> Self {
        Self::LexerError(value)
    }
}
