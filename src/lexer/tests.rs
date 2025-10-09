use crate::lexer::{
    error::LexerError,
    token::{Operator, Punctuation, Token},
};

/// Macro to generate lexer tests
macro_rules! lexer_test {
    ($name:ident, $input:expr, [$($expected:expr),* $(,)?]) => {
        #[test]
        fn $name() {
            let lexer = crate::lexer::Lexer::<f64>::new($input);
            let tokens: Vec<_> = lexer.collect();

            let expected_tokens = vec![
                $($expected,)*
            ];

            assert_eq!(
                tokens,
                expected_tokens,
                "Token mismatch in test '{}'\ninput: {}\nactual: {:?}\nexpected: {:?}",
                stringify!($name),
                $input,
                tokens,
                expected_tokens
            );
        }
    };
}

// ======== Example usage ========

// Simple numbers
lexer_test!(
    numbers,
    "42 6.954 0.001",
    [
        Ok(Token::Number(42.0)),
        Ok(Token::Number(6.954)),
        Ok(Token::Number(0.001))
    ]
);

// Operators
lexer_test!(
    operators,
    "+ - * /",
    [
        Ok(Token::Operator(Operator::Plus)),
        Ok(Token::Operator(Operator::Minus)),
        Ok(Token::Operator(Operator::Star)),
        Ok(Token::Operator(Operator::Slash)),
    ]
);

// Punctuation
lexer_test!(
    punctuation,
    "( );  =",
    [
        Ok(Token::Punctuation(Punctuation::LeftParenthesis)),
        Ok(Token::Punctuation(Punctuation::RightParenthesis)),
        Ok(Token::Punctuation(Punctuation::Semicolon)),
        Ok(Token::Punctuation(Punctuation::Assignment))
    ]
);

// Identifiers
lexer_test!(
    identifiers,
    "x y1 variable_name",
    [
        Ok(Token::Identifier("x".into())),
        Ok(Token::Identifier("y1".into())),
        Ok(Token::Identifier("variable_name".into()))
    ]
);

// Mixed expression
lexer_test!(
    mixed_expression,
    "x = 3 + 4.5 * (y - 2);",
    [
        Ok(Token::Identifier("x".into())),
        Ok(Token::Punctuation(Punctuation::Assignment)),
        Ok(Token::Number(3.0)),
        Ok(Token::Operator(Operator::Plus)),
        Ok(Token::Number(4.5)),
        Ok(Token::Operator(Operator::Star)),
        Ok(Token::Punctuation(Punctuation::LeftParenthesis)),
        Ok(Token::Identifier("y".into())),
        Ok(Token::Operator(Operator::Minus)),
        Ok(Token::Number(2.0)),
        Ok(Token::Punctuation(Punctuation::RightParenthesis)),
        Ok(Token::Punctuation(Punctuation::Semicolon))
    ]
);

// Invalid character test
lexer_test!(
    invalid_character,
    "42 &",
    [
        Ok(Token::Number(42.0)),
        Err(LexerError::UnexpectedChar('&', 3))
    ]
);

// Invalid number ending with decimal point (no digits after dot)
lexer_test!(
    invalid_number_trailing_dot,
    "5.",
    [
        Err(LexerError::InvalidNumber("5.".to_string(), 2))
    ]
);
