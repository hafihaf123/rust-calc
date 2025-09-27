use std::{iter::Peekable, str::Chars};

use crate::{error::LexerError, token::Token};

#[derive(Debug, Clone, Copy)]
enum LexerState {
    Start,
    Integer,
    Decimal,
    Identifier,
    Done,
    Error,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    current_char: Option<char>,
    state: LexerState,
    position: usize,
    buffer: String,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars().peekable();
        let current_char = chars.peek().cloned();
        Lexer {
            input: chars,
            state: LexerState::Start,
            current_char,
            position: 0,
            buffer: String::new(),
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        unimplemented!();
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        unimplemented!();
    }

    fn advance(&mut self) {
        if let Some(current) = self.current_char {
            self.position += current.len_utf8();
            self.current_char = self.input.next();
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(Token::Eof) => None,
            result => Some(result),
        }
    }
}
