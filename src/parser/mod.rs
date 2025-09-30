pub mod ast;
pub mod error;
pub mod tests;

use crate::Numeric;
use crate::lexer::token::{Associativity, Operator, Punctuation};
use crate::lexer::{Lexer, token::Token};
use crate::parser::ast::{Expression, Statement};
use crate::parser::error::ParserError;

use std::iter::Peekable;

pub struct Parser<'a, N: Numeric> {
    lexer: Peekable<Lexer<'a, N>>,
    // current: Option<Token>,
}

impl<'a, N: Numeric> Parser<'a, N> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
            // current: None,
        }
    }

    fn peek(&mut self) -> Result<Option<&Token<N>>, ParserError<N>> {
        self.lexer
            .peek()
            .map(Result::as_ref)
            .transpose()
            .map_err(|e| e.clone().into())
    }

    fn advance(&mut self) -> Result<Token<N>, ParserError<N>> {
        self.lexer
            .next()
            .ok_or(ParserError::UnexpectedEnd)?
            .map_err(Into::into)
    }

    fn expect(&mut self, token: &Token<N>) -> Result<(), ParserError<N>> {
        let next_token = self.advance()?;
        if &next_token == token {
            Ok(())
        } else {
            Err(ParserError::UnexpectedToken(next_token))
        }
    }

    pub fn parse_program(&mut self) -> Result<Vec<Statement<N>>, ParserError<N>> {
        let mut statements = Vec::new();
        while self.peek()?.is_some() {
            let statement = self.parse_statement()?;
            if statement != Statement::Empty {
                self.expect(&Token::Punctuation(Punctuation::Semicolon))?;
            }
            statements.push(statement);
        }
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement<N>, ParserError<N>> {
        match self.advance()? {
            Token::Identifier(var)
                if matches!(
                    self.peek()?,
                    Some(&Token::Punctuation(Punctuation::Assignment))
                ) =>
            {
                self.parse_assignment(var)
            }
            Token::Punctuation(Punctuation::Semicolon) => Ok(Statement::Empty),
            token => self.parse_expression(token, 0).map(Statement::Expression),
        }
    }

    fn parse_assignment(&mut self, var_name: String) -> Result<Statement<N>, ParserError<N>> {
        self.expect(&Token::Punctuation(Punctuation::Assignment))?;
        let first_expression_token = self.advance()?;
        Ok(Statement::Assignment(
            var_name.clone(),
            self.parse_expression(first_expression_token, 0)?,
        ))
    }

    fn parse_expression(
        &mut self,
        first: Token<N>,
        min_precedence: u8,
    ) -> Result<Expression<N>, ParserError<N>> {
        let mut primary = self.parse_primary(first)?;
        loop {
            match self.peek()? {
                Some(&Token::Operator(operator)) => {
                    if operator.priority() < min_precedence {
                        break;
                    }
                    self.advance()?; // consume the operator
                    let next_min_prec = if operator.associativity() == Associativity::Left {
                        operator.priority() + 1
                    } else {
                        operator.priority()
                    };
                    let token_after_operator = self.advance()?;
                    let after_operator =
                        self.parse_expression(token_after_operator, next_min_prec)?;
                    primary =
                        Expression::Binary(Box::new(primary), operator, Box::new(after_operator));
                }
                Some(&Token::Punctuation(Punctuation::Semicolon)) => break,
                Some(&Token::Punctuation(Punctuation::RightParenthesis)) => break,
                None => break,
                Some(token) => return Err(ParserError::UnexpectedToken(token.clone())),
            }
        }
        Ok(primary)
    }

    fn parse_primary(&mut self, first: Token<N>) -> Result<Expression<N>, ParserError<N>> {
        match first {
            Token::Number(num) => Ok(Expression::Number(num)),
            Token::Identifier(var_name) => match self.peek()? {
                Some(&Token::Punctuation(Punctuation::LeftParenthesis)) => {
                    let left_parenthesis = self.advance()?;
                    let argument = self.parse_primary(left_parenthesis)?;
                    Ok(Expression::Call(var_name, Box::new(argument)))
                }
                _ => Ok(Expression::Variable(var_name)),
            },
            Token::Punctuation(Punctuation::LeftParenthesis) => {
                let next_token = self.advance()?;
                let result = self.parse_expression(next_token, 0)?;
                self.expect(&Token::Punctuation(Punctuation::RightParenthesis))?;
                Ok(result)
            }
            Token::Operator(operator @ (Operator::Plus | Operator::Minus)) => {
                let next_token = self.advance()?;
                let operand = self.parse_primary(next_token)?;
                Ok(Expression::Unary(operator.try_into()?, Box::new(operand)))
            }
            token => Err(ParserError::UnexpectedToken(token)),
        }
    }
}
