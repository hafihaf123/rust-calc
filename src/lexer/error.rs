#[derive(Debug, Clone, PartialEq)]
pub enum LexerError {
    UnexpectedChar(char, usize),
    InvalidNumber(String, usize),
}
