use crate::{
    Numeric,
    lexer::{error::LexerError, token::Token},
};

#[derive(Debug)]
pub enum ParserError<N: Numeric> {
    LexerError(LexerError),
    UnexpectedToken(Token<N>),
    UnexpectedEnd,
    InvalidAssignment,
}

impl<N: Numeric> From<LexerError> for ParserError<N> {
    fn from(value: LexerError) -> Self {
        Self::LexerError(value)
    }
}
