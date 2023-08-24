#![allow(dead_code, unused_variables)]
use crate::{
    expression::{Expression, ParseExpressionError},
    lexer::Lexer,
    object::{Context, Object},
    parser::Parser,
    token::Token,
};

#[derive(Debug)]
pub(crate) enum EvalError {
    ExpectedObjectXFoundY {
        expected: &'static str,
        found: Object,
    },
    ExpectedTokenXFoundY {
        expected: &'static str,
        found: Token,
    },
    ParseExpressionError {
        error: ParseExpressionError,
    },
}

pub(crate) fn eval_input(input: String, context: Context) -> Result<Object, EvalError> {
    let mut parser = Parser::new(Lexer::from_input(input.as_str()))
        .map_err(|err| EvalError::ParseExpressionError { error: err })?;

    let expression = parser
        .parse()
        .map_err(|err| EvalError::ParseExpressionError { error: err })?;

    eval(&expression, context)
}

fn eval(expression: &Expression, context: Context) -> Result<Object, EvalError> {
    match &expression {
        Expression::Int(val) => Ok(Object::Int(*val)),
        Expression::Double(val) => Ok(Object::Double(*val)),
        Expression::Str(val) => Ok(Object::Str(val.clone())),
        Expression::Bool(val) => Ok(Object::Bool(*val)),
        Expression::InfixExpression {
            operator,
            left,
            right,
        } => todo!(),
        Expression::PrefixExpression {
            operator,
            expression,
        } => eval_prefix_expression(operator.clone(), expression, context),
        _ => todo!("sdfsdf"),
    }
}

fn eval_prefix_expression(
    operator: Token,
    expression: &Expression,
    context: Context,
) -> Result<Object, EvalError> {
    let value = eval(expression, context)?;
    match operator {
        Token::Minus => match value {
            Object::Int(val) => Ok(Object::Int(-1 * val)),
            Object::Double(val) => Ok(Object::Double(-1 as f64 * val)),
            _ => Err(EvalError::ExpectedObjectXFoundY {
                expected: "number",
                found: value,
            }),
        },
        Token::Bang | Token::Not => match value {
            Object::Bool(val) => Ok(Object::Bool(!val)),
            _ => Err(EvalError::ExpectedObjectXFoundY {
                expected: "bool",
                found: value,
            }),
        },
        _ => Err(EvalError::ExpectedTokenXFoundY {
            expected: "prefix operator",
            found: operator,
        }),
    }
}

fn eval_infix_exprression(
    operator: Token,
    left: &Expression,
    right: &Expression,
    context: Context,
) -> Result<Object, EvalError> {
    todo!()
}
fn eval_expression(expression: &Expression, context: Context) -> Result<Object, EvalError> {
    todo!()
}
