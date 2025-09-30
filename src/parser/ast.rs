use std::marker::PhantomData;

use crate::Numeric;
use crate::lexer::token::Operator;
use crate::parser::error::ParserError;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression<N: Numeric> {
    Number(N),
    Variable(String),
    Unary(UnaryOp<N>, Box<Expression<N>>),
    Binary(Box<Expression<N>>, Operator, Box<Expression<N>>),
    Call(String, Box<Expression<N>>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp<N: Numeric> {
    Negative,
    Positive,
    _Marker(PhantomData<N>),
}

impl<N: Numeric> UnaryOp<N> {
    pub fn apply(&self, a: N) -> N {
        match self {
            UnaryOp::Negative => N::zero() - a,
            UnaryOp::Positive => a,
            UnaryOp::_Marker(_) => unreachable!(),
        }
    }
}

impl<N: Numeric> TryFrom<Operator> for UnaryOp<N> {
    type Error = ParserError<N>;
    fn try_from(value: Operator) -> Result<UnaryOp<N>, ParserError<N>> {
        match value {
            Operator::Plus => Ok(Self::Positive),
            Operator::Minus => Ok(Self::Negative),
            _ => Err(ParserError::UnexpectedToken(
                crate::lexer::token::Token::<N>::Operator(value),
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement<N: Numeric> {
    Assignment(String, Expression<N>),
    Expression(Expression<N>),
    Empty,
}
