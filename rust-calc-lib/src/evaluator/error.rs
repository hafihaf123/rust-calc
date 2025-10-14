use crate::numeric::NumericValue;
use crate::parser::error::ParserError;

#[derive(Debug)]
pub enum EvaluatorError<N: NumericValue> {
    ParserError(ParserError<N>),
    UnexpectedError,
    OperationFailed(String),
    UndefinedVariable(String),
    UnknownFunction(String),
    InvalidAssignment(String),
}
