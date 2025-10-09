pub mod error;

use std::collections::HashMap;

use crate::evaluator::error::EvaluatorError;
use crate::numeric::{BuiltinFn, NumericValue};
use crate::parser::ast::{Expression, Statement};
use crate::parser::Parser;

pub struct Evaluator<N: NumericValue, F: BuiltinFn<N>> {
    env: HashMap<String, N>,
    builtins: F,
}

impl<N: NumericValue, F: BuiltinFn<N>> Evaluator<N, F> {
    pub fn new(builtins: F) -> Self {
        Self {
            env: HashMap::new(),
            builtins,
        }
    }

    pub fn parse(&mut self, input: &str) -> Result<Option<N>, EvaluatorError<N>> {
        let mut parser = Parser::new(input);
        let mut res = Err(EvaluatorError::UnexpectedError);
        for statement in parser
            .parse_program()
            .map_err(EvaluatorError::ParserError)?
        {
            res = self.eval_statement(statement)
        }
        res
    }

    fn eval_statement(&mut self, statement: Statement<N>) -> Result<Option<N>, EvaluatorError<N>> {
        match statement {
            Statement::Assignment(var_name, expression) => {
                let expr_res = self.eval_expression(expression)?;
                self.env.insert(var_name, expr_res);
                Ok(None)
            }
            Statement::Expression(expression) => self.eval_expression(expression).map(Some),
            Statement::Empty => Ok(None),
        }
    }

    fn eval_expression(&mut self, expression: Expression<N>) -> Result<N, EvaluatorError<N>> {
        match expression {
            Expression::Number(n) => Ok(n),
            Expression::Variable(var) => self
                .env
                .get(&var)
                .ok_or(EvaluatorError::UndefinedVariable(var))
                .cloned(),
            Expression::Unary(unary_op, expression) => {
                Ok(unary_op.apply(self.eval_expression(*expression)?))
            }
            Expression::Binary(expression, operator, expression1) => operator
                .apply(
                    self.eval_expression(*expression)?,
                    self.eval_expression(*expression1)?,
                )
                .map_err(EvaluatorError::OperationFailed),
            Expression::Call(func_name, expression) => {
                let argument = self.eval_expression(*expression)?;
                self.builtins
                    .call(&func_name, argument)
                    .ok_or_else(|| EvaluatorError::UnknownFunction(func_name))
            }
        }
    }
}
