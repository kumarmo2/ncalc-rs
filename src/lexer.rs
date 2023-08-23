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
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
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
                // There is no assign operator i.e '='
                None => Token::Illegal,
                Some(peek_position) => {
                    if b'=' == self.source[peek_position] {
                        Token::Equals
                    } else {
                        Token::Illegal
                    }
                }
            },
            _ => Token::Illegal,
        };
        self.read_char();
        Some(token)
    }
}
