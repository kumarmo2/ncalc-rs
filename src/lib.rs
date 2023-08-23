#![allow(dead_code)]

pub(crate) mod expression;
pub(crate) mod lexer;
pub(crate) mod parser;
pub(crate) mod tests;
pub(crate) mod token;

pub struct MathematicalExpression {
    source: String,
}

impl MathematicalExpression {
    pub fn new(input: &str) -> Self {
        Self {
            source: input.to_string(),
        }
    }

    pub(crate) fn get_source(&self) -> &str {
        &self.source
    }
}
