#![warn(rust_2018_idioms, missing_debug_implementations)]

use crate::token::{keyword_token, Token};
use Token::*;

pub mod token;

#[derive(Debug, Clone, PartialEq)]
pub struct Tokenizer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.skip_whitespace();
        let token = self.get_token();
        self.read_char();

        match token {
            EndOfFile => None,
            rest => Some(rest),
        }
    }
}

#[allow(dead_code)]
impl Tokenizer {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input,
            position: 0,
            read_position: 0,
            ch: ' ',
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = ' ';
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn get_token(&mut self) -> Token {
        match self.ch {
            '[' => LeftBracket,
            ']' => RightBracket,
            '(' => LeftParenthesis,
            ')' => RightParenthesis,
            '{' => LeftBrace,
            '}' => RightBrace,
            '.' => {
                let is_spread =
                    self.look_ahead() == Some('.') && self.look_ahead_by(2) == Some('.');
                if is_spread {
                    self.skip_next_chars_by(2);
                    Spread
                } else {
                    Dot
                }
            }
            ';' => Semicolon,
            ':' => Colon,
            ',' => Comma,
            '<' => match self.look_ahead() {
                Some('=') => {
                    self.skip_next_char();
                    LessEquals
                }
                Some('<') => match self.look_ahead_by(2usize) {
                    Some('=') => {
                        self.skip_next_chars_by(2usize);
                        LeftShiftAssign
                    }
                    _ => {
                        self.skip_next_char();
                        LeftShift
                    }
                },
                _ => LessThan,
            },
            '>' => match self.look_ahead() {
                Some('=') => {
                    self.skip_next_char();
                    MoreEquals
                }
                Some('>') => match self.look_ahead_by(2usize) {
                    Some('=') => {
                        self.skip_next_chars_by(2usize);
                        RightShiftAssign
                    }
                    Some('>') => match self.look_ahead_by(3usize) {
                        Some('=') => {
                            self.skip_next_chars_by(3usize);
                            UnsignedRightShiftAssign
                        }
                        _ => {
                            self.skip_next_chars_by(2usize);
                            UnsignedRightShift
                        }
                    },
                    _ => {
                        self.skip_next_char();
                        RightShift
                    }
                },
                _ => MoreThan,
            },
            '+' => match self.look_ahead() {
                Some('=') => {
                    self.skip_next_char();
                    AdditionAssign
                }
                _ => Addition,
            },
            '-' => match self.look_ahead() {
                Some('=') => {
                    self.skip_next_char();
                    SubtractionAssign
                }
                _ => Subtraction,
            },
            '*' => match self.look_ahead() {
                Some('=') => {
                    self.skip_next_char();
                    MultiplicationAssign
                }
                _ => Multiplication,
            },
            '/' => match self.look_ahead() {
                Some('=') => {
                    self.skip_next_char();
                    DivisionAssign
                }
                _ => Division,
            },
            '%' => match self.look_ahead() {
                Some('=') => {
                    self.skip_next_char();
                    ModulusAssign
                }
                _ => Modulus,
            },
            '&' => match self.look_ahead() {
                Some('&') => {
                    self.skip_next_char();
                    LogicalAnd
                }
                _ => BitwiseAnd,
            },
            '|' => match self.look_ahead() {
                Some('|') => {
                    self.skip_next_char();
                    LogicalOr
                }
                _ => BitwiseOr,
            },
            '^' => match self.look_ahead() {
                Some('=') => {
                    self.skip_next_char();
                    BitwiseXORAssign
                }
                _ => BitwiseXOR,
            },
            '!' => match self.look_ahead() {
                Some('=') => {
                    self.skip_next_char();
                    NotEquals
                }
                _ => LogicalNot,
            },
            '~' => match self.look_ahead() {
                Some('=') => {
                    self.skip_next_char();
                    BitwiseNotAssign
                }
                _ => BitwiseNot,
            },
            '=' => match self.look_ahead() {
                Some('=') => match self.look_ahead_by(2usize) {
                    Some('=') => {
                        self.skip_next_chars_by(2usize);
                        StrictEquals
                    }
                    _ => {
                        self.skip_next_char();
                        Equals
                    }
                },
                Some('>') => {
                    self.skip_next_char();
                    Arrow
                }
                _ => Assign,
            },
            _ => {
                if self.is_letter() {
                    let id = self.read_identifier();

                    match keyword_token(&id) {
                        Ok(token) => token,
                        Err(_) => Identifier(id),
                    }
                } else if self.is_number() {
                    let id = self.read_number();

                    Identifier(id)
                } else {
                    EndOfFile
                }
            }
        }
    }

    /// Returns the next char, based on the struct state.
    fn look_ahead(&mut self) -> Option<char> {
        self.look_ahead_by(1)
    }

    /// Returns the next `x` char, based on the struct state.
    fn look_ahead_by(&mut self, x: usize) -> Option<char> {
        let next_position = self.position + x;

        if next_position > self.input.len() || self.input.get(next_position).is_none() {
            None
        } else {
            self.input.get(next_position).map(|c| *c)
        }
    }

    fn skip_next_char(&mut self) {
        self.skip_next_chars_by(1);
    }

    fn skip_next_chars_by(&mut self, x: usize) {
        self.position += x;
        self.read_position += x;
    }

    fn read_identifier(&mut self) -> Vec<char> {
        let pos = self.position;

        while !self.is_eof() && self.is_letter() {
            self.read_char();
        }
        self.back();

        self.input[pos..self.position].to_vec()
    }

    fn read_number(&mut self) -> Vec<char> {
        let pos = self.position;

        while !self.is_eof() && self.is_number() {
            self.read_char();
        }
        self.back();

        self.input[pos..self.position].to_vec()
    }

    fn is_eof(&self) -> bool {
        self.read_position > self.input.len()
    }

    fn is_letter(&self) -> bool {
        ('a' <= self.ch && 'z' > self.ch) || ('A' <= self.ch && 'Z' > self.ch) || ('_' == self.ch)
    }

    fn is_number(&self) -> bool {
        '0' <= self.ch && '9' > self.ch
    }

    fn skip_whitespace(&mut self) {
        match self.ch {
            ' ' | '\t' | '\n' | '\r' => self.read_char(),
            _ => {}
        }
    }

    fn back(&mut self) {
        self.read_position -= 1;
        self.ch = self.input[self.read_position - 1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec_char(input: &str) -> Vec<char> {
        input.chars().collect::<Vec<char>>()
    }

    fn check(tokenizer: Tokenizer, expected: Vec<Token>) {
        let actual: Vec<Token> = tokenizer.collect();
        assert_eq!(actual, expected);
    }

    fn identifier(name: &str) -> Token {
        Identifier(name.chars().collect::<Vec<char>>())
    }

    #[test]
    fn sum() {
        let sum = vec_char("1 + 1");
        let tokenizer = Tokenizer::new(sum);

        let expected = vec![identifier("1"), Addition, identifier("1")];

        check(tokenizer, expected);
    }

    #[test]
    fn einstein() {
        let input = vec_char("e = m * c * c");
        let tokenizer = Tokenizer::new(input);

        let expected = vec![
            identifier("e"),
            Assign,
            identifier("m"),
            Multiplication,
            identifier("c"),
            Multiplication,
            identifier("c"),
        ];

        check(tokenizer, expected);
    }

    #[test]
    fn function() {
        let input = vec_char("function sum(a, b) { return a + b }");
        let tokenizer = Tokenizer::new(input);

        let sum = identifier("sum");
        let a = identifier("a");
        let b = identifier("b");
        let expected = vec![
            Function,
            sum,
            LeftParenthesis,
            a.clone(),
            Comma,
            b.clone(),
            RightParenthesis,
            LeftBrace,
            Return,
            a,
            Addition,
            b,
            RightBrace,
        ];

        check(tokenizer, expected);
    }

    #[test]
    fn symbol_after_keyword() {
        let input = vec_char("await (this.wait(200))");
        let tokenizer = Tokenizer::new(input);

        let expected = vec![
            Await,
            LeftParenthesis,
            This,
            Dot,
            identifier("wait"),
            LeftParenthesis,
            identifier("200"),
            RightParenthesis,
            RightParenthesis,
        ];

        check(tokenizer, expected);
    }

    #[test]
    fn different_symbols() {
        let input = vec_char("a = (b != c) == d");
        let tokenizer = Tokenizer::new(input);

        let expected = vec![
            identifier("a"),
            Assign,
            LeftParenthesis,
            identifier("b"),
            NotEquals,
            identifier("c"),
            RightParenthesis,
            Equals,
            identifier("d"),
        ];

        check(tokenizer, expected);
    }

    #[test]
    fn longer_symbols() {
        let input = vec_char("b >>>= c");
        let tokenizer = Tokenizer::new(input);

        let expected = vec![identifier("b"), UnsignedRightShiftAssign, identifier("c")];

        check(tokenizer, expected);
    }

    #[test]
    fn keywords() {
        let input = vec_char("if (a) { return b } else { return c }");
        let tokenizer = Tokenizer::new(input);

        let expected = vec![
            If,
            LeftParenthesis,
            identifier("a"),
            RightParenthesis,
            LeftBrace,
            Return,
            identifier("b"),
            RightBrace,
            Else,
            LeftBrace,
            Return,
            identifier("c"),
            RightBrace,
        ];

        check(tokenizer, expected);
    }
    
    #[test]
    fn bitwise() {
        let input = vec_char("z ^= a & b | c ^ d");
        let tokenizer = Tokenizer::new(input);

        let expected = vec![
            identifier("z"),
            BitwiseXORAssign,
            identifier("a"),
            BitwiseAnd,
            identifier("b"),
            BitwiseOr,
            identifier("c"),
            BitwiseXOR,
            identifier("d"),
        ];

        check(tokenizer, expected);
    }
}
