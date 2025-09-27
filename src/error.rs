#[derive(Debug, Clone)]
pub enum LexerError {
    UnexpectedChar(char, usize),
    InvalidNumber(String, usize),
}
