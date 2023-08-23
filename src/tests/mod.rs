#![cfg(test)]

mod expression;

mod lexer {
    use std::rc::Rc;

    use crate::{lexer::Lexer, token::Token};

    #[test]
    fn test_tokens() {
        let input = "()[]  abc 1234 xcvxcgf \"xyz\" \"\"  ";
        let mut lexer = Lexer::from_input(input);

        assert_eq!(Token::LParen, lexer.next().unwrap());
        assert_eq!(Token::RParen, lexer.next().unwrap());
        assert_eq!(Token::LBracket, lexer.next().unwrap());
        assert_eq!(Token::RBracket, lexer.next().unwrap());
        assert_eq!(
            Token::Ident(Rc::new("abc".to_string())),
            lexer.next().unwrap()
        );
        assert_eq!(Token::IntLiteral(1234), lexer.next().unwrap());
        assert_eq!(
            Token::Ident(Rc::new("xcvxcgf".to_string())),
            lexer.next().unwrap()
        );
        assert_eq!(
            Token::StringLiteral(Rc::new("xyz".to_string())),
            lexer.next().unwrap()
        );
        assert_eq!(
            Token::StringLiteral(Rc::new("".to_string())),
            lexer.next().unwrap()
        );
        assert_eq!(Some(Token::EOF), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_number_tokens() {
        let tests = vec![
            ("123.4", Token::DoubleLiteral(123.4)),
            ("1", Token::IntLiteral(1)),
            ("100", Token::IntLiteral(100)),
        ]
        .into_iter();

        for (input, expected) in tests {
            let mut lexer = Lexer::from_input(input);
            assert_eq!(expected, lexer.next().unwrap());
        }
    }

    #[test]
    fn test_empty() {
        let input = "";
        let mut lexer = Lexer::from_input(input);
        assert_eq!(Token::EOF, lexer.next().unwrap());
    }
}
