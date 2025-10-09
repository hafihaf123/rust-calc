use std::marker::PhantomData;
use std::str::Chars;

use crate::lexer::token::{Operator, Punctuation};
use crate::numeric::NumericValue;

use super::error::LexerError;
use super::token::Token;

#[derive(Debug)]
pub struct FSMContext<'a> {
    input: Chars<'a>,
    current_char: Option<char>,
    // state: State,
    position: usize,
    buffer: String,
}

impl<'a> FSMContext<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current_char = chars.next();
        Self {
            input: chars,
            // state: State::Start,
            current_char,
            position: 0,
            buffer: String::new(),
        }
    }

    pub fn advance(&mut self) {
        if let Some(current) = self.current_char {
            self.position += current.len_utf8();
            self.current_char = self.input.next();
        }
    }
}

#[derive(Debug)]
pub struct LexerFSM<'a, State, N: NumericValue> {
    // ctx: &'a mut FSMContext<'a>,
    ctx: FSMContext<'a>,
    _state: std::marker::PhantomData<(State, N)>,
}

// impl<'a, State> LexerFSM<'a, State> {
//     fn into_state<S>(&mut self) -> &mut LexerFSM<'a, S> {
//         unsafe { &mut *(self as *mut LexerFSM<'a, State> as *mut LexerFSM<'a, S>) }
//     }
// }

impl<'a, State, N: NumericValue> LexerFSM<'a, State, N> {
    fn into_state<S>(self) -> LexerFSM<'a, S, N> {
        LexerFSM::<S, N> {
            ctx: self.ctx,
            _state: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct Start;
#[derive(Debug)]
pub struct IntegerPart;
#[derive(Debug)]
pub struct DecimalPart;
#[derive(Debug)]
pub struct InIdentifier;

impl<'a, N: NumericValue> LexerFSM<'a, Start, N> {
    pub fn new(input: &'a str) -> Self {
        Self {
            ctx: FSMContext::new(input),
            _state: PhantomData,
        }
    }

    pub fn next_token(mut self) -> Result<(Token<N>, LexerFSM<'a, Start, N>), LexerError> {
        while let Some(c) = self.ctx.current_char {
            if c.is_whitespace() {
                self.ctx.advance();
                continue;
            }
            if c.is_ascii_digit() {
                return self
                    .into_state::<IntegerPart>()
                    .collect()
                    .map(|(token, fsm)| (token, fsm.into_state()));
            }
            if c.is_ascii_alphabetic() {
                let (token, fsm) = self.into_state::<InIdentifier>().collect();
                return Ok((token, fsm.into_state()));
            }
            if let Some(op) = Operator::get(c) {
                self.ctx.advance();
                return Ok((Token::Operator(op), self));
            }
            if let Some(punc) = Punctuation::get(c) {
                self.ctx.advance();
                return Ok((Token::Punctuation(punc), self));
            }
            return Err(LexerError::UnexpectedChar(c, self.ctx.position));
        }
        Ok((Token::Eof, self))
    }
}

impl<'a, N: NumericValue> LexerFSM<'a, IntegerPart, N> {
    pub fn collect(mut self) -> Result<(Token<N>, LexerFSM<'a, IntegerPart, N>), LexerError> {
        self.ctx.buffer.clear();
        while let Some(c) = self.ctx.current_char {
            if c.is_ascii_digit() {
                self.ctx.buffer.push(c);
                self.ctx.advance();
                continue;
            }
            if c == '.' {
                self.ctx.buffer.push(c);
                self.ctx.advance(); // consume the decimal separator (a dot - '.')
                return self
                    .into_state::<DecimalPart>()
                    .collect()
                    .map(|(token, fsm)| (token, fsm.into_state()));
            }
            break;
        }
        Ok((
            Token::Number(N::from_str_radix(&self.ctx.buffer, 10).map_err(|_| {
                LexerError::InvalidNumber(self.ctx.buffer.clone(), self.ctx.position)
            })?),
            self,
        ))
    }
}

impl<'a, N: NumericValue> LexerFSM<'a, DecimalPart, N> {
    pub fn collect(mut self) -> Result<(Token<N>, LexerFSM<'a, DecimalPart, N>), LexerError> {
        let initial_len = self.ctx.buffer.len();
        while let Some(c) = self.ctx.current_char {
            if !c.is_ascii_digit() {
                break;
            }
            self.ctx.buffer.push(c);
            self.ctx.advance();
        }
        // Check if at least one digit was added after the decimal point
        if self.ctx.buffer.len() == initial_len {
            return Err(LexerError::InvalidNumber(
                self.ctx.buffer.clone(),
                self.ctx.position,
            ));
        }
        Ok((
            Token::Number(N::from_str_radix(&self.ctx.buffer, 10).map_err(|_| {
                LexerError::InvalidNumber(self.ctx.buffer.clone(), self.ctx.position)
            })?),
            self,
        ))
    }
}

impl<'a, N: NumericValue> LexerFSM<'a, InIdentifier, N> {
    pub fn collect(mut self) -> (Token<N>, LexerFSM<'a, InIdentifier, N>) {
        self.ctx.buffer.clear();
        while let Some(c) = self.ctx.current_char {
            if !c.is_ascii_alphabetic() && !c.is_ascii_digit() && c != '_' {
                break;
            }
            self.ctx.buffer.push(c);
            self.ctx.advance();
        }
        (Token::Identifier(self.ctx.buffer.clone()), self)
    }
}
