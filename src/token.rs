#[derive(Debug, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    Assignment,
}

#[derive(Debug, Clone)]
pub enum Punctuation {
    LeftParenthesis,
    RightParenthesis,
    Semicolon,
}

#[derive(Debug, Clone)]
pub enum Identifier {
    Variable(String),
}

#[derive(Debug, Clone)]
pub enum Token {
    Number(f64),
    Identifier(Identifier),
    Operator(Operator),
    Punctuation(Punctuation),
    Eof,
}
