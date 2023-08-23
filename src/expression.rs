use std::rc::Rc;

use crate::token::Token;

#[derive(Debug, PartialEq)]
pub(crate) enum Expression {
    Nil, // there was no input
    Int(i64),
    Double(f64),
    Bool(bool),
    Ident(Rc<String>),
    Str(Rc<String>),
    PrefixExpression {
        operator: Token,
        expression: Box<Expression>,
    },
}

#[derive(Debug)]
pub(crate) enum ParseExpressionError {
    NothingToParse,
    UnexpectedEnd,
    ExpectedXFoundY {
        expected: &'static str,
        found: Token,
    },
}

pub(crate) enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix, // -x or !x
    Call,   // mufunc()
}

impl Precedence {
    pub(crate) fn value(&self) -> i32 {
        match self {
            Precedence::Lowest => 0,
            Precedence::Equals => 1,
            Precedence::LessGreater => 2,
            Precedence::Sum => 3,
            Precedence::Product => 4,
            Precedence::Prefix => 5,
            Precedence::Call => 6,
        }
    }
}
