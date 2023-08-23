#![allow(dead_code)]

use crate::token::Token;

pub(crate) struct Lexer {
    source: Vec<u8>,
    read_position: Option<usize>,
    last_position: usize,
    peek_position: Option<usize>,
}

impl Lexer {
    pub(crate) fn from_input(input: &str) -> Self {
        let source: Vec<u8> = input.bytes().collect();
        let last_position = source.len() - 1;
        let peek_position = match source.len() {
            0 | 1 => None,
            _ => Some(1),
        };
        let read_position = match source.len() {
            0 => None,
            _ => Some(0),
        };
        Self {
            source,
            read_position,
            last_position,
            peek_position,
        }
    }

    fn read_char(&mut self) {
        let Some(peek_position) = self.peek_position else {
            self.read_position = None;
            return;
        };
        std::mem::swap(&mut self.read_position, &mut self.peek_position);
        if peek_position >= self.last_position {
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
        return Token::Int(
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
        Token::Double(
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
            return None;
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
                    _ => Token::Ident(identifier),
                }
            }
            b'0'..=b'9' => self.read_number(read_position),
            _ => Token::Illegal,
        };
        self.read_char();
        Some(token)
    }
}
