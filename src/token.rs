#![allow(dead_code)]

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Token {
    // Data Types
    Int(i64),
    Double(f64),
    True,
    False,
    Str(String),
    DateTime(String),

    // Identifier
    Ident(String),

    // Grouping Operator
    LParen, // '('
    RParen, // ')'

    LBracket, // '['
    RBracket, // ']'
    //
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
    Illegal,
}
