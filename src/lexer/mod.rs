pub mod error;
mod fsm;
#[cfg(test)]
mod tests;
pub mod token;

use error::LexerError;
use token::Token;

use crate::{
    lexer::fsm::{LexerFSM, Start},
    numeric::Numeric,
};

#[derive(Debug)]
pub struct Lexer<'a, N: Numeric> {
    fsm: Option<LexerFSM<'a, Start, N>>,
}

impl<'a, N: Numeric> Lexer<'a, N> {
    pub fn new(input: &'a str) -> Self {
        Self {
            fsm: Some(LexerFSM::new(input)),
        }
    }
}

impl<'a, N: Numeric> Iterator for Lexer<'a, N> {
    type Item = Result<Token<N>, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        let fsm = self.fsm.take()?;
        match fsm.next_token() {
            Ok((token, new_fsm)) => {
                if token == Token::Eof {
                    return None;
                }
                self.fsm = Some(new_fsm);
                Some(Ok(token))
            }
            Err(e) => Some(Err(e)),
        }
    }
}
