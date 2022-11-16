#[derive(Debug, PartialEq)]
pub struct Token {
    pub _type: String,
    pub value: String,
}

pub fn tokenize_char(_type: &str, value: &str) -> Token {
    Token {
        _type: _type.to_string(),
        value: value.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = tokenize_char("keyword", "function");
        assert_eq!(
            res,
            Token {
                _type: "keyword".to_string(),
                value: "function".to_string()
            }
        );
    }
}
