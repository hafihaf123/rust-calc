use crate::{lexer::token::Operator, parser::error::ParserError};

#[derive(Debug)]
pub enum Expression {
    Number(f64),
    Variable(String),
    Unary(UnaryOp, Box<Expression>),
    Binary(Box<Expression>, Operator, Box<Expression>),
    Call(String, Box<Expression>),
}

#[derive(Debug)]
pub enum UnaryOp {
    Negative,
    Positive,
}

impl TryFrom<Operator> for UnaryOp {
    type Error = ParserError;
    fn try_from(value: Operator) -> Result<UnaryOp, ParserError> {
        match value {
            Operator::Plus => Ok(Self::Positive),
            Operator::Minus => Ok(Self::Negative),
            _ => Err(ParserError::UnexpectedToken(
                crate::lexer::token::Token::Operator(value),
            )),
        }
    }
}

#[derive(Debug)]
pub enum Statement {
    Assignment(String, Expression),
    Expression(Expression),
    Empty,
}
