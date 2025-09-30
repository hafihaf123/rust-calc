use crate::{Numeric, parser::error::ParserError};

#[derive(Debug)]
pub enum EvaluatorError<N: Numeric> {
    ParserError(ParserError<N>),
    UnexpectedError,
    OperationFailed(String),
    UndefinedVariable(String),
}
