#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Plus,
    Identifier(Vec<char>),
    Function,
    EndOfFile,
}

pub fn keyword_token(chars: &[char]) -> Result<Token, String> {
    let id: String = chars.iter().collect();

    match id.as_ref() {
        "function" => Ok(Token::Function),
        _ => Err("keyword could not be found".to_string()),
    }
}
