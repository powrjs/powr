use crate::token::{keyword_token, Token};
use Token::*;

pub mod token;

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
            '.' => Dot,
            ';' => Semicolon,
            ':' => Colon,
            ',' => Comma,
            '<' => LessThan,
            '>' => MoreThan,
            '+' => Plus,
            '-' => Minus,
            '*' => Asterisk,
            '/' => Slash,
            '%' => Percentage,
            '&' => Ampersand,
            '|' => Vertical,
            '^' => Caret,
            '!' => Bang,
            '~' => Tilde,
            '=' => Assign,
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

        let expected = vec![identifier("1"), Plus, identifier("1")];

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
            Asterisk,
            identifier("c"),
            Asterisk,
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
            Plus,
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
}
