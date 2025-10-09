#![allow(unused_imports)]
use crate::lexer::token::Operator;
use crate::parser::{Expression, Statement, ast::UnaryOp};

/// Macro to generate lexer tests
macro_rules! lexer_test {
    ($name:ident, $input:expr, [$($expected:expr),* $(,)?]) => {
        #[test]
        fn $name() {

            let mut parser = crate::parser::Parser::<f64>::new($input);
            let statements = parser.parse_program().unwrap();

            let expected_statements = vec![
                $($expected,)*
            ];

            assert_eq!(
                statements,
                expected_statements,
                "Statement mismatch in test '{}'\ninput: {}\nactual: {:?}\nexpected: {:?}",
                stringify!($name),
                $input,
                statements,
                expected_statements
            );
        }
    };
}

lexer_test!(
    number_literal,
    "4;",
    [Statement::Expression(Expression::Number(4f64)),]
);

lexer_test!(
    variable_expression,
    "x;",
    [Statement::Expression(Expression::Variable("x".to_string())),]
);

lexer_test!(
    unary_negative,
    "-4;",
    [Statement::Expression(Expression::Unary(
        UnaryOp::Negative,
        Box::new(Expression::Number(4f64))
    )),]
);

lexer_test!(
    unary_positive,
    "+5;",
    [Statement::Expression(Expression::Unary(
        UnaryOp::Positive,
        Box::new(Expression::Number(5f64))
    )),]
);

lexer_test!(
    binary_addition,
    "2 + 3;",
    [Statement::Expression(Expression::Binary(
        Box::new(Expression::Number(2f64)),
        Operator::Plus,
        Box::new(Expression::Number(3f64))
    )),]
);

lexer_test!(
    binary_multiplication,
    "4 * 5;",
    [Statement::Expression(Expression::Binary(
        Box::new(Expression::Number(4f64)),
        Operator::Star,
        Box::new(Expression::Number(5f64))
    )),]
);

lexer_test!(
    precedence_test,
    "2 + 3 * 4;",
    [Statement::Expression(Expression::Binary(
        Box::new(Expression::Number(2f64)),
        Operator::Plus,
        Box::new(Expression::Binary(
            Box::new(Expression::Number(3f64)),
            Operator::Star,
            Box::new(Expression::Number(4f64))
        ))
    )),]
);

lexer_test!(
    parenthesis_test,
    "(2 + 3) * 4;",
    [Statement::Expression(Expression::Binary(
        Box::new(Expression::Binary(
            Box::new(Expression::Number(2f64)),
            Operator::Plus,
            Box::new(Expression::Number(3f64))
        )),
        Operator::Star,
        Box::new(Expression::Number(4f64))
    )),]
);

lexer_test!(
    assignment_statement,
    "x = 42;",
    [Statement::Assignment(
        "x".to_string(),
        Expression::Number(42f64)
    ),]
);

lexer_test!(
    chained_add_sub,
    "1 + 2 - 3;",
    [Statement::Expression(Expression::Binary(
        Box::new(Expression::Binary(
            Box::new(Expression::Number(1f64)),
            Operator::Plus,
            Box::new(Expression::Number(2f64))
        )),
        Operator::Minus,
        Box::new(Expression::Number(3f64))
    )),]
);

lexer_test!(
    function_call_simple,
    "square(2);",
    [Statement::Expression(Expression::Call(
        "square".to_string(),
        Box::new(Expression::Number(2f64))
    )),]
);

lexer_test!(
    nested_function_call,
    "f(g(1));",
    [Statement::Expression(Expression::Call(
        "f".to_string(),
        Box::new(Expression::Call(
            "g".to_string(),
            Box::new(Expression::Number(1f64))
        ))
    )),]
);

lexer_test!(empty_statement, ";", [Statement::Empty,]);

lexer_test!(
    multiple_statements,
    "x = 1; y = x + 2;",
    [
        Statement::Assignment("x".to_string(), Expression::Number(1f64)),
        Statement::Assignment(
            "y".to_string(),
            Expression::Binary(
                Box::new(Expression::Variable("x".to_string())),
                Operator::Plus,
                Box::new(Expression::Number(2f64))
            )
        ),
    ]
);

lexer_test!(
    function_call_with_expression,
    "sqrt(2 + 3);",
    [Statement::Expression(Expression::Call(
        "sqrt".to_string(),
        Box::new(Expression::Binary(
            Box::new(Expression::Number(2f64)),
            Operator::Plus,
            Box::new(Expression::Number(3f64))
        ))
    )),]
);
