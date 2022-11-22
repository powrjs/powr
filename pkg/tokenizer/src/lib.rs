use crate::token::{keyword_token, Token};

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

        let token = match self.ch {
            '+' => Token::Plus,
            _ => {
                if self.is_letter() {
                    let id = self.read_identifier();

                    match keyword_token(&id) {
                        Ok(token) => token,
                        Err(_) => Token::Identifier(id),
                    }
                } else if self.is_number() {
                    let id = self.read_number();
                    Token::Identifier(id)
                } else {
                    Token::EndOfFile
                }
            }
        };

        self.read_char();
        token
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

    #[test]
    fn sum() {
        let sum = "1 + 1".chars().collect();
        let mut tokenizer = Tokenizer::new(sum);

        let expected = vec![
            Token::Identifier(vec!['1']),
            Token::Plus,
            Token::Identifier(vec!['1']),
            Token::EndOfFile,
        ];

        let mut actual: Vec<Token> = Vec::new();
        let mut t = tokenizer.next();
        actual.push(t.clone());
        while t != Token::EndOfFile {
            t = tokenizer.next();
            actual.push(t.clone());
        }

        assert_eq!(actual, expected);
    }
}
