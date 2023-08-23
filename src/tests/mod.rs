#![cfg(test)]

mod lexer {
    use crate::{lexer::Lexer, token::Token};

    #[test]
    fn test_tokens() {
        let input = "()[]  abc 1234 xcvxcgf \"xyz\" \"\"  ";
        let mut lexer = Lexer::from_input(input);

        assert_eq!(Token::LParen, lexer.next().unwrap());
        assert_eq!(Token::RParen, lexer.next().unwrap());
        assert_eq!(Token::LBracket, lexer.next().unwrap());
        assert_eq!(Token::RBracket, lexer.next().unwrap());
        assert_eq!(Token::Ident("abc".to_string()), lexer.next().unwrap());
        assert_eq!(Token::Int(1234), lexer.next().unwrap());
        assert_eq!(Token::Ident("xcvxcgf".to_string()), lexer.next().unwrap());
        assert_eq!(Token::Str("xyz".to_string()), lexer.next().unwrap());
        assert_eq!(Token::Str("".to_string()), lexer.next().unwrap());
        assert_eq!(Some(Token::EOF), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn test_number_tokens() {
        let tests = vec![
            ("123.4", Token::Double(123.4)),
            ("1", Token::Int(1)),
            ("100", Token::Int(100)),
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
