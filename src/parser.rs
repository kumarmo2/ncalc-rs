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

    fn peek_next(&self) -> &Token {
        &self.next_token
    }

    pub(crate) fn parse(&mut self) -> Result<Expression, ParseExpressionError> {
        self.parse_expression(Precedence::Lowest)
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, ParseExpressionError> {
        self.advance_token()?;
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
            // None => return Err(ParseExpressionError::UnexpectedEnd),
            None => Token::EOF,
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
        let x = match &self.curr_token {
            Token::Ident(ident) => Ok(Expression::Ident(ident.clone()))?,
            _ => {
                return Err(ParseExpressionError::ExpectedXFoundY {
                    expected: "Identifier",
                    found: self.curr_token.clone(),
                })
            }
        };
        self.advance_token()?;
        Ok(x)
    }

    fn parse_infix_expression(
        &mut self,
        left: Expression,
    ) -> Result<Expression, ParseExpressionError> {
        let operator = self.curr_token.clone();
        self.advance_token()?;

        let right = self.parse_expression(operator.get_precedence())?;
        Ok(Expression::InfixExpression {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    fn parse_function_call_expression(
        &mut self,
        function: Expression,
    ) -> Result<Expression, ParseExpressionError> {
        self.advance_token()?; // curr_token is Token::LParen.
        let mut args = vec![];
        while self.curr_token != Token::RParen {
            let arg = self.parse_expression(Precedence::Lowest)?;
            args.push(Box::new(arg));
            self.advance_token()?;
            if let Token::Comma = self.curr_token.clone() {
                self.advance_token()?;
            }
        }
        self.advance_token()?;
        Ok(Expression::CallExpression {
            function: Box::new(function),
            arguments: args,
        })
    }

    fn parse_expression(
        &mut self,
        precedence: Precedence,
    ) -> Result<Expression, ParseExpressionError> {
        let mut left_expression = match self.curr_token.clone() {
            Token::IntLiteral(int) => Expression::Int(int),
            Token::DoubleLiteral(double) => Expression::Double(double),
            Token::True => Expression::Bool(true),
            Token::False => Expression::Bool(false),
            Token::LBracket => self.parse_bracket_ident_expression()?,
            Token::LParen => self.parse_grouped_expression()?,
            Token::Ident(ident) => Expression::Ident(ident.clone()),
            Token::StringLiteral(string) => Expression::Str(string.clone()),
            Token::Minus | Token::Not | Token::Bang => self.parse_prefix_expression()?,
            _ => {
                return Err(ParseExpressionError::UnexpectedToken {
                    token: self.curr_token.clone(),
                })
            }
        };

        while *self.peek_next() != Token::EOF
            && self.peek_next().get_precedence().value() > precedence.value()
        {
            self.advance_token()?;
            left_expression = match self.curr_token.clone() {
                Token::Plus
                | Token::Minus
                | Token::Asterisk
                | Token::Slash
                | Token::Percent
                | Token::Equals
                | Token::NotEquals
                | Token::NotEqualsAngleBrackets
                | Token::LessThan
                | Token::LessThanEqualTo
                | Token::GreaterThan
                | Token::GreaterThanEqualTo
                | Token::Or
                | Token::DoublePipe => self.parse_infix_expression(left_expression)?,
                Token::LParen => self.parse_function_call_expression(left_expression)?,
                _ => unimplemented!(),
            }
        }
        Ok(left_expression)
    }
}
