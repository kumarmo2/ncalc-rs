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
    ]
    .into_iter();
    for (input, expected) in tests {
        let mut parser = Parser::new(Lexer::from_input(input)).unwrap();
        let expression = parser.parse().unwrap();
        assert_eq!(expected, expression);
    }
}
