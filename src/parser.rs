use crate::{
    expression::{Expression, Precedence},
    lexer::Lexer,
};

pub(crate) struct Parser {
    _lexer: Lexer,
}

impl Parser {
    pub(crate) fn new(lexer: Lexer) -> Self {
        Self { _lexer: lexer }
    }

    pub(crate) fn parse(&mut self) -> Expression {
        todo!()
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Expression {
        todo!()
    }
}
