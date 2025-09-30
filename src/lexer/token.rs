use crate::numeric::Numeric;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Associativity {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
}

impl Operator {
    pub fn get(c: char) -> Option<Self> {
        match c {
            '+' => Some(Self::Plus),
            '-' => Some(Self::Minus),
            '*' => Some(Self::Star),
            '/' => Some(Self::Slash),
            '^' => Some(Self::Caret),
            _ => None,
        }
    }

    pub fn priority(&self) -> u8 {
        match self {
            Operator::Plus => 1,
            Operator::Minus => 1,
            Operator::Star => 2,
            Operator::Slash => 2,
            Operator::Caret => 3,
        }
    }

    pub fn associativity(&self) -> Associativity {
        match self {
            Operator::Caret => Associativity::Right,
            _ => Associativity::Left,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Punctuation {
    LeftParenthesis,
    RightParenthesis,
    Semicolon,
    Assignment,
}

impl Punctuation {
    pub fn get(c: char) -> Option<Self> {
        match c {
            '(' => Some(Self::LeftParenthesis),
            ')' => Some(Self::RightParenthesis),
            ';' => Some(Self::Semicolon),
            '=' => Some(Self::Assignment),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token<N: Numeric> {
    Number(N),
    Identifier(String),
    Operator(Operator),
    Punctuation(Punctuation),
    Eof,
}
