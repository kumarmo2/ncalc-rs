use std::rc::Rc;

use crate::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
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
    InfixExpression {
        operator: Token,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    CallExpression {
        function: Box<Expression>, // For ncalc, this would always be a Function name.
        // Had it been a case of full-fledged language, it could have been an inline function.
        arguments: Vec<Box<Expression>>,
    },
}

#[derive(Debug)]
pub enum ParseExpressionError {
    NothingToParse,
    UnexpectedEnd,
    ExpectedXFoundY {
        expected: &'static str,
        found: Token,
    },
    UnexpectedToken {
        token: Token,
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
