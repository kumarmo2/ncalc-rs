use std::rc::Rc;

use crate::{
    expression::{Expression, ParseExpressionError, Precedence},
    lexer::Lexer,
    token::Token,
};

pub(crate) struct Parser {
    _lexer: Lexer,
    curr_token: Token,
    next_token: Token,
}

impl Parser {
    pub(crate) fn new(mut lexer: Lexer) -> Result<Self, ParseExpressionError> {
        if lexer.peek_char().is_none() {
            return Err(ParseExpressionError::NothingToParse);
        }
        // This means at the very least, we will have one user defined token
        // and one EOF token. That means 2 times doing next should be fine.
        let curr_token = lexer.next().unwrap();
        let next_token = lexer.next().unwrap();
        Ok(Self {
            _lexer: lexer,
            curr_token,
            next_token,
        })
    }

    pub(crate) fn parse(&mut self) -> Result<Expression, ParseExpressionError> {
        self.parse_expression(Precedence::Lowest)
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, ParseExpressionError> {
        let expression = self.parse_expression(Precedence::Lowest)?;
        self.advance_token()?;
        match &self.curr_token {
            Token::RParen => Ok(expression),
            _ => Err(ParseExpressionError::ExpectedXFoundY {
                expected: "Token::RParen",
                found: self.curr_token.clone(),
            }),
        }
    }

    fn advance_token(&mut self) -> Result<(), ParseExpressionError> {
        self.curr_token = self.next_token.clone();
        self.next_token = match self._lexer.next() {
            Some(token) => token,
            None => return Err(ParseExpressionError::UnexpectedEnd),
        };
        Ok(())
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParseExpressionError> {
        let prefix_operator = self.curr_token.clone();
        let _ = self.advance_token()?;
        let right_expression = self.parse_expression(prefix_operator.get_precedence())?;
        Ok(Expression::PrefixExpression {
            operator: prefix_operator,
            expression: Box::new(right_expression),
        })
    }

    fn parse_bracket_ident_expression(&mut self) -> Result<Expression, ParseExpressionError> {
        let _l_bracket_token = self.curr_token.clone();
        let _ = self.advance_token()?;
        match &self.curr_token {
            Token::Ident(ident) => Ok(Expression::Ident(ident.clone())),
            _ => Err(ParseExpressionError::ExpectedXFoundY {
                expected: "Identifier",
                found: self.curr_token.clone(),
            }),
        }
    }

    fn parse_expression(
        &mut self,
        precedence: Precedence,
    ) -> Result<Expression, ParseExpressionError> {
        let left_expression = match self.curr_token.clone() {
            Token::IntLiteral(int) => Expression::Int(int),
            Token::DoubleLiteral(double) => Expression::Double(double),
            Token::True => Expression::Bool(true),
            Token::False => Expression::Bool(false),
            Token::LBracket => self.parse_bracket_ident_expression()?,
            Token::LParen => self.parse_grouped_expression()?,
            Token::Minus | Token::Not | Token::Bang => self.parse_prefix_expression()?,
            _ => todo!(),
        };
        Ok(left_expression)
    }
}
