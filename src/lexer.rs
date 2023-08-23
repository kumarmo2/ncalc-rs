#![allow(dead_code)]

use std::rc::Rc;

use crate::token::Token;

#[derive(Debug)]
pub(crate) struct Lexer {
    source: Vec<u8>,
    read_position: Option<usize>,
    last_position: Option<usize>,
    peek_position: Option<usize>,
    eof_returned: bool,
}

impl Lexer {
    pub(crate) fn from_input(input: &str) -> Self {
        let source: Vec<u8> = input.bytes().collect();
        let last_position = match source.len() {
            n if n > 0 => Some(n - 1),
            _ => None,
        };
        let peek_position = match source.len() {
            0 | 1 => None,
            _ => Some(1),
        };
        let read_position = match source.len() {
            0 => None,
            _ => Some(0),
        };
        let to_return = Self {
            source,
            read_position,
            last_position,
            peek_position,
            eof_returned: false,
        };
        to_return
    }

    pub(crate) fn peek_char(&mut self) -> Option<u8> {
        match self.read_position {
            Some(read_position) => Some(self.source[read_position]),
            None => None,
        }
    }
    fn read_char(&mut self) {
        let Some(peek_position) = self.peek_position else {
            self.read_position = None;
            return;
        };
        std::mem::swap(&mut self.read_position, &mut self.peek_position);
        // this unwrap should be safe.
        if peek_position >= self.last_position.unwrap() {
            self.peek_position = None;
        } else {
            self.peek_position = Some(peek_position + 1)
        }
    }

    fn skip_whitespaces(&mut self) {
        loop {
            let Some(read_position) = self.read_position else {
                return;
            };
            match self.source[read_position] {
                b'\t' | b' ' | b'\n' | b'\r' => self.read_char(),
                _ => return,
            }
        }
    }

    fn get_integer_token(slice: &[u8]) -> Token {
        return Token::IntLiteral(
            i64::from_str_radix(
                String::from_utf8(slice.iter().map(|x| *x).collect())
                    .unwrap()
                    .as_str(),
                10,
            )
            .unwrap(),
        );
    }
    fn get_double_token(slice: &[u8]) -> Token {
        Token::DoubleLiteral(
            String::from_utf8(slice.iter().map(|x| *x).collect())
                .unwrap()
                .parse::<f64>()
                .unwrap(),
        )
    }

    fn read_number(&mut self, read_position: usize) -> Token {
        let mut has_fractional = false;
        loop {
            // TODO: remove unwraps.
            match self.peek_position {
                None => match has_fractional {
                    false => return Lexer::get_integer_token(&self.source[read_position..]),
                    true => return Lexer::get_double_token(&self.source[read_position..]),
                },
                Some(peek_position) => match self.source[peek_position] {
                    b'.' => {
                        has_fractional = true;
                        self.read_char();
                        continue;
                    }
                    b'0'..=b'9' => {
                        self.read_char();
                        continue;
                    }
                    _ => match has_fractional {
                        false => {
                            return Lexer::get_integer_token(
                                &self.source[read_position..peek_position],
                            )
                        }
                        true => {
                            return Lexer::get_double_token(
                                &self.source[read_position..peek_position],
                            )
                        }
                    },
                },
            }
        }
    }

    fn read_string(&mut self) -> Token {
        let mut chars: Vec<u8> = vec![];
        let token = loop {
            self.read_char();
            let Some(read_position) = self.read_position else {
                        return Token::Illegal
            };

            let ch = self.source[read_position];
            if ch == b'"' {
                break Token::StringLiteral(Rc::new(String::from_utf8(chars).unwrap()));
            }

            if ch == b'\\' {
                let Some(peek_position) = self.peek_position else {
                            break Token::Illegal
                };
                let next_char = self.source[peek_position];
                let next_char = match next_char {
                    b'"' => b'"',
                    b'n' => b'\n',
                    b't' => b'\x09',
                    b'r' => b'\x0d',
                    b'\\' => b'\x5c',
                    _ => todo!("expected valid escape sequence, but found {}", next_char),
                };
                chars.push(next_char);
                self.read_char();
                continue;
            }
            chars.push(ch);
        };
        token
    }

    fn read_identifier(&mut self, read_position: usize) -> String {
        loop {
            match self.peek_position {
                None => {
                    return String::from_utf8(
                        self.source[read_position..].iter().map(|x| *x).collect(),
                    )
                    .unwrap();
                }
                Some(peek_position) => match self.source[peek_position] {
                    b'a'..=b'z' | b'A'..=b'Z' => {
                        self.read_char();
                        continue;
                    }
                    _ => {
                        return String::from_utf8(
                            self.source[read_position..peek_position]
                                .iter()
                                .map(|x| *x)
                                .collect(),
                        )
                        .unwrap()
                    }
                },
            }
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespaces();
        let Some(read_position) = self.read_position else {
            match self.eof_returned {
                false => {
                    self.eof_returned = true;
                    return Some(Token::EOF);
                },
                    true => {
                    return None;
                }
            }
        };

        let token = match self.source[read_position] {
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'[' => Token::LBracket,
            b']' => Token::RBracket,
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'*' => Token::Asterisk,
            b'/' => Token::Slash,
            b'%' => Token::Percent,
            b'=' => match self.peek_position {
                // There is no assign operator i.e '=' in expressions.
                None => Token::Illegal,
                Some(peek_position) => {
                    if b'=' == self.source[peek_position] {
                        self.read_char();
                        Token::Equals
                    } else {
                        Token::Illegal
                    }
                }
            },
            b'!' => match self.peek_position {
                None => Token::Bang,
                Some(peek_position) => {
                    if b'=' == self.source[peek_position] {
                        self.read_char();
                        Token::NotEquals
                    } else {
                        Token::Bang
                    }
                }
            },
            b'<' => match self.peek_position {
                None => Token::LessThan,
                Some(peek_position) => match self.source[peek_position] {
                    b'=' => {
                        self.read_char();
                        Token::LessThanEqualTo
                    }
                    b'>' => {
                        self.read_char();
                        Token::NotEqualsAngleBrackets
                    }
                    _ => Token::LessThan,
                },
            },
            b'>' => match self.peek_position {
                None => Token::GreaterThan,
                Some(peek_position) => match self.source[peek_position] {
                    b'=' => {
                        self.read_char();
                        Token::GreaterThanEqualTo
                    }
                    _ => Token::GreaterThan,
                },
            },
            b'&' => match self.peek_position {
                None => Token::Illegal,
                Some(peek_position) => match self.source[peek_position] {
                    b'&' => {
                        self.read_char();
                        Token::DoubleAmpersand
                    }
                    _ => Token::Illegal,
                },
            },
            b'|' => match self.peek_position {
                None => Token::Illegal,
                Some(peek_position) => match self.source[peek_position] {
                    b'|' => {
                        self.read_char();
                        Token::DoublePipe
                    }
                    _ => Token::Illegal,
                },
            },
            b'a'..=b'z' | b'A'..=b'Z' => {
                let identifier = self.read_identifier(read_position);
                match identifier.as_str() {
                    "not" => Token::Not,
                    "and" => Token::And,
                    "or" => Token::Or,
                    "true" => Token::True,
                    "false" => Token::False,
                    _ => Token::Ident(Rc::new(identifier)),
                }
            }
            b'0'..=b'9' => self.read_number(read_position),
            b'"' => self.read_string(),
            // TODO: handle datetimes
            _ => Token::Illegal,
        };
        self.read_char();
        Some(token)
    }
}
