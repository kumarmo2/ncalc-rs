#![allow(dead_code, unused_variables)]
use std::rc::Rc;

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
    ExpectedExpressionXFoundY {
        expected: &'static str,
        found: Expression,
    },
    ParseExpressionError {
        error: ParseExpressionError,
    },
    UnExpectedTokenFound {
        token: Token,
    },
    MissMatchFunctionArguements {
        expected: u8,
        found: u8,
    },
    ReferenceNotFound(String),
    UnExpectedOperatorOperandFound {
        operator: Token,
        left: Object,
        right: Object,
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
        } => eval_infix_expression(operator.clone(), left, right, context),
        Expression::PrefixExpression {
            operator,
            expression,
        } => eval_prefix_expression(operator.clone(), expression, context),
        Expression::CallExpression {
            function,
            arguments,
        } => eval_fn_call(function, arguments.as_slice(), context),
        Expression::Ident(ident) => eval_ident(ident, context.clone()),
        _ => unimplemented!(),
    }
}

fn eval_fn_call(
    function_expression: &Expression,
    arguments: &[Box<Expression>],
    context: Context,
) -> Result<Object, EvalError> {
    let Expression::Ident(function_name) = function_expression else {
        return Err(EvalError::ExpectedExpressionXFoundY { expected: "function name", found: function_expression.clone() });
    };

    let _ = match function_name.as_str() {
        "if" => (),
        _ => unimplemented!(),
    };

    if arguments.len() != 3 {
        return Err(EvalError::MissMatchFunctionArguements {
            expected: 3,
            found: arguments.len() as u8,
        });
    }

    let condition_evaluated_val = eval(arguments[0].as_ref(), context.clone())?;
    let Object::Bool(is_true) = condition_evaluated_val else {
        return Err(EvalError::ExpectedObjectXFoundY { expected: "bool", found: condition_evaluated_val })
    };

    match is_true {
        true => eval(arguments[1].as_ref(), context.clone()),
        false => eval(arguments[2].as_ref(), context.clone()),
    }
}

fn eval_numeric_infix_expression(
    operator: Token,
    left: &Expression,
    right: &Expression,
    context: Context,
) -> Result<Object, EvalError> {
    let left = eval(left, context.clone())?;
    let right = eval(right, context.clone())?;
    let (left, right) = match operator.clone() {
        Token::Plus
        | Token::Minus
        | Token::Asterisk
        | Token::Slash
        | Token::LessThan
        | Token::LessThanEqualTo
        | Token::GreaterThan
        | Token::GreaterThanEqualTo
        | Token::Percent => match (&left, &right) {
            (Object::Int(left), Object::Int(right)) => (*left as f64, *right as f64),
            (Object::Double(left), Object::Double(right)) => (*left, *right),
            (Object::Double(left), Object::Int(right)) => (*left, *right as f64),
            (Object::Int(left), Object::Double(right)) => (*left as f64, *right),
            _ => {
                return Err(EvalError::UnExpectedOperatorOperandFound {
                    operator,
                    left,
                    right,
                })
            }
        },
        _ => unimplemented!(),
    };

    Ok(apply_operator_to_float_values(operator, left, right))
}

fn apply_operator_to_float_values(operator: Token, left: f64, right: f64) -> Object {
    match operator.clone() {
        Token::Plus => Object::Double(left + right),
        Token::Minus => Object::Double(left - right),
        Token::Asterisk => Object::Double(left * right),
        Token::Slash => Object::Double(left / right),
        Token::LessThan => Object::Bool(left < right),
        Token::LessThanEqualTo => Object::Bool(left <= right),
        Token::GreaterThan => Object::Bool(left > right),
        Token::GreaterThanEqualTo => Object::Bool(left >= right),
        Token::Equals => Object::Bool(left == right),
        Token::NotEquals => Object::Bool(left != right),
        Token::Percent => Object::Double(left % right),
        _ => unimplemented!(),
    }
}

fn eval_infix_expression_where_operand_can_be_numerics_or_bools(
    operator: Token,
    left: &Expression,
    right: &Expression,
    context: Context,
) -> Result<Object, EvalError> {
    let left = eval(left, context.clone())?;
    let right = eval(right, context.clone())?;
    match (&left, &right) {
        (Object::Int(left), Object::Int(right)) => {
            let left = *left as f64;
            let right = *right as f64;
            Ok(apply_operator_to_float_values(operator, left, right))
        }
        (Object::Double(left), Object::Double(right)) => {
            let left = *left;
            let right = *right;
            Ok(apply_operator_to_float_values(operator, left, right))
        }
        (Object::Double(left), Object::Int(right)) => {
            let left = *left;
            let right = *right as f64;
            Ok(apply_operator_to_float_values(operator, left, right))
        }
        (Object::Int(left), Object::Double(right)) => {
            let left = *left as f64;
            let right = *right;
            Ok(apply_operator_to_float_values(operator, left, right))
        }
        (Object::Bool(left), Object::Bool(right)) => match operator {
            Token::Equals => Ok(Object::Bool(left == right)),
            Token::NotEquals => Ok(Object::Bool(left != right)),
            Token::Or | Token::DoublePipe => Ok(Object::Bool(*left || *right)),
            _ => todo!(),
        },
        _ => {
            return Err(EvalError::UnExpectedOperatorOperandFound {
                operator,
                left,
                right,
            })
        }
    }
}

fn eval_infix_expression(
    operator: Token,
    left: &Expression,
    right: &Expression,
    context: Context,
) -> Result<Object, EvalError> {
    match operator.clone() {
        Token::Plus
        | Token::Minus
        | Token::Asterisk
        | Token::Slash
        | Token::LessThan
        | Token::LessThanEqualTo
        | Token::GreaterThan
        | Token::GreaterThanEqualTo
        | Token::Percent => eval_numeric_infix_expression(operator, left, right, context),
        Token::Equals | Token::NotEquals | Token::DoublePipe | Token::Or => {
            eval_infix_expression_where_operand_can_be_numerics_or_bools(
                operator, left, right, context,
            )
        }
        _ => Err(EvalError::UnExpectedTokenFound { token: operator }),
    }
}

fn eval_ident(ident: &Rc<String>, context: Context) -> Result<Object, EvalError> {
    match context.get(ident.as_ref()) {
        Some(object) => Ok(object),
        None => Err(EvalError::ReferenceNotFound(ident.as_ref().to_owned())),
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
