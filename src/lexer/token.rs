#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    Assignment,
}

impl Operator {
    pub fn get(c: char) -> Option<Self> {
        match c {
            '+' => Some(Self::Plus),
            '-' => Some(Self::Minus),
            '*' => Some(Self::Star),
            '/' => Some(Self::Slash),
            '^' => Some(Self::Caret),
            '=' => Some(Self::Assignment),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Punctuation {
    LeftParenthesis,
    RightParenthesis,
    Semicolon,
}

impl Punctuation {
    pub fn get(c: char) -> Option<Self> {
        match c {
            '(' => Some(Self::LeftParenthesis),
            ')' => Some(Self::RightParenthesis),
            ';' => Some(Self::Semicolon),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Identifier {
    Variable(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Identifier(Identifier),
    Operator(Operator),
    Punctuation(Punctuation),
    Eof,
}
