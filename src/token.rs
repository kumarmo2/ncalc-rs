#![allow(dead_code)]

use std::rc::Rc;

use crate::expression::Precedence;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Token {
    // Data Types
    IntLiteral(i64),
    DoubleLiteral(f64),
    True,
    False,
    Str(Rc<String>),
    // TODO: this also should be wrapped in Rc.
    DateTime(String),

    // Identifier
    Ident(Rc<String>),

    // Grouping Operator
    LParen, // '('
    RParen, // ')'

    LBracket, // '['
    RBracket, // ']'

    // Logical Negation
    Bang, // !
    Not,  // not

    // Multiplicative Operators
    Asterisk, // *
    Slash,    // '/'
    Percent,  // '%'

    // Additive operators
    Plus,
    Minus,

    // Relational
    LessThan, // '<'
    GreaterThan,
    LessThanEqualTo, // '<='
    GreaterThanEqualTo,

    Equals,                 // '=='
    NotEquals,              // '!='
    NotEqualsAngleBrackets, // '<>'

    And,             // 'and'
    DoubleAmpersand, // '&&'

    Or,         // 'or'
    DoublePipe, // '||'
    EOF,
    Illegal,
}

impl Token {
    pub(crate) fn get_precedence(&self) -> Precedence {
        match self {
            Token::Equals | Token::NotEquals => Precedence::Equals,
            Token::LessThan
            | Self::LessThanEqualTo
            | Token::GreaterThanEqualTo
            | Token::GreaterThan => Precedence::LessGreater,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Slash | Token::Asterisk => Precedence::Product,
            Token::LParen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }
}
