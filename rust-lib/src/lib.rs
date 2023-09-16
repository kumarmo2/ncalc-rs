#![allow(dead_code)]
#![forbid(unsafe_code, unstable_features)]
#![warn(clippy::unwrap_used)]

use object::{Context, Object};

pub mod evaluator;
pub(crate) mod expression;
pub(crate) mod lexer;
pub mod object;
pub(crate) mod parser;
pub(crate) mod tests;
pub(crate) mod token;

// TODO: once the integration with 'c' is done, make this &str,
pub struct MathematicalExpression {
    source: String,
}

impl MathematicalExpression {
    pub fn new(input: &str) -> Self {
        Self {
            source: input.to_string(),
        }
    }

    pub fn eval(&mut self) -> Result<Object, String> {
        evaluator::eval_input(&self.source, Context::default()).map_err(|e| format!("{:?}", e))
    }
}
