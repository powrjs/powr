use crate::token::{keyword_token, Token};
use Token::*;

mod token;

struct Tokenizer {
    input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
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

    pub fn next(&mut self) -> Token {
        self.skip_whitespace();
        let token = self.get_token();
        self.read_char();
        token
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

        self.input[pos..self.position].to_vec()
    }

    fn read_number(&mut self) -> Vec<char> {
        let pos = self.position;

        while !self.is_eof() && self.is_number() {
            self.read_char();
        }

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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec_char(input: &str) -> Vec<char> {
        input.chars().collect::<Vec<char>>()
    }

    fn check(mut tokenizer: Tokenizer, expected: Vec<Token>) {
        let mut actual: Vec<Token> = Vec::new();
        let mut t = tokenizer.next();
        actual.push(t.clone());
        while t != EndOfFile {
            t = tokenizer.next();
            actual.push(t.clone());
        }

        assert_eq!(actual, expected);
    }

    #[test]
    fn sum() {
        let sum = vec_char("1 + 1");
        let tokenizer = Tokenizer::new(sum);

        let expected = vec![
            Identifier(vec!['1']),
            Plus,
            Identifier(vec!['1']),
            EndOfFile,
        ];

        check(tokenizer, expected);
    }

    #[test]
    fn einstein() {
        let input = vec_char("e = m * c * c");
        let tokenizer = Tokenizer::new(input);

        let expected = vec![
            Identifier(vec!['e']),
            Assign,
            Identifier(vec!['m']),
            Asterisk,
            Identifier(vec!['c']),
            Asterisk,
            Identifier(vec!['c']),
            EndOfFile,
        ];

        check(tokenizer, expected);
    }
}
