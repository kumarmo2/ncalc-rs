use std::rc::Rc;

use crate::{expression::Expression, lexer::Lexer, parser::Parser, token::Token};

#[test]
fn test_basic_expressions() {
    let tests = vec![
        ("123", Expression::Int(123)),
        ("12.3", Expression::Double(12.3)),
        (
            "-12",
            Expression::PrefixExpression {
                operator: Token::Minus,
                expression: Box::new(Expression::Int(12)),
            },
        ),
        (
            "\"m o_hit2\"",
            Expression::Str(Rc::new("m o_hit2".to_string())),
        ),
        ("(4)", Expression::Int(4)),
    ]
    .into_iter();
    for (input, expected) in tests {
        let mut parser = Parser::new(Lexer::from_input(input)).unwrap();
        let expression = parser.parse().unwrap();
        assert_eq!(expected, expression);
    }
}

#[test]
fn test_basic_infix_expressions() {
    let tests = vec![
        (
            "1+2",
            Expression::InfixExpression {
                operator: Token::Plus,
                left: Box::new(Expression::Int(1)),
                right: Box::new(Expression::Int(2)),
            },
        ),
        (
            "1-2",
            Expression::InfixExpression {
                operator: Token::Minus,
                left: Box::new(Expression::Int(1)),
                right: Box::new(Expression::Int(2)),
            },
        ),
        (
            "1*2",
            Expression::InfixExpression {
                operator: Token::Asterisk,
                left: Box::new(Expression::Int(1)),
                right: Box::new(Expression::Int(2)),
            },
        ),
        (
            "1 + (2 - 3)",
            Expression::InfixExpression {
                operator: Token::Plus,
                left: Box::new(Expression::Int(1)),
                right: Box::new(Expression::InfixExpression {
                    operator: Token::Minus,
                    left: Box::new(Expression::Int(2)),
                    right: Box::new(Expression::Int(3)),
                }),
            },
        ),
        (
            "1.23 + (2 * 3)",
            Expression::InfixExpression {
                operator: Token::Plus,
                left: Box::new(Expression::Double(1.23)),
                right: Box::new(Expression::InfixExpression {
                    operator: Token::Asterisk,
                    left: Box::new(Expression::Int(2)),
                    right: Box::new(Expression::Int(3)),
                }),
            },
        ),
    ]
    .into_iter();

    for (test, expected) in tests {
        let mut parser = Parser::new(Lexer::from_input(test)).unwrap();
        let expression = parser.parse().unwrap();
        assert_eq!(expected, expression);
    }
}
