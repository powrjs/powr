use Token::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal,
    Identifier(Vec<char>),

    // punctuators
    // https://tc39.es/ecma262/#sec-punctuators
    LeftBracket,      // [
    RightBracket,     // ]
    LeftParenthesis,  // (
    RightParenthesis, // )
    LeftBrace,        // {
    RightBrace,       // }
    Dot,              // .
    Semicolon,        // ;
    Colon,            // :
    Comma,            // ,
    LessThan,         // <
    MoreThan,         // >
    Plus,             // +
    Minus,            // -
    Asterisk,         // *
    Slash,            // /
    Percentage,       // %
    Ampersand,        // &
    Vertical,         // |
    Caret,            // ^
    Bang,             // !
    Tilde,            // ~
    Assign,           // =

    // reserved words
    Await,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Enum,
    Export,
    Extends,
    False,
    Finally,
    For,
    Function,
    If,
    Import,
    In,
    InstanceOf,
    New,
    Null,
    Return,
    Super,
    Switch,
    This,
    Throw,
    True,
    Try,
    TypeOf,
    Var,
    Void,
    While,
    With,
    Yield,

    // contextually disallowed as identifiers in strict mode
    Let,
    Static,
    Implements,
    Interface,
    Package,
    Private,
    Protected,
    Public,

    // always allowed as identifiers
    As,
    Async,
    From,
    Get,
    Meta,
    Of,
    Set,
    Target,

    EndOfFile,
}

pub fn keyword_token(chars: &[char]) -> Result<Token, String> {
    let id: String = chars.iter().collect();

    match id.as_ref() {
        "await" => Ok(Await),
        "break" => Ok(Break),
        "case" => Ok(Case),
        "catch" => Ok(Catch),
        "class" => Ok(Class),
        "const" => Ok(Const),
        "continue" => Ok(Continue),
        "debugger" => Ok(Debugger),
        "default" => Ok(Default),
        "delete" => Ok(Delete),
        "do" => Ok(Do),
        "else" => Ok(Else),
        "enum" => Ok(Enum),
        "export" => Ok(Export),
        "extends" => Ok(Extends),
        "false" => Ok(False),
        "finally" => Ok(Finally),
        "for" => Ok(For),
        "function" => Ok(Function),
        "if" => Ok(If),
        "import" => Ok(Import),
        "in" => Ok(In),
        "instanceof" => Ok(InstanceOf),
        "new" => Ok(New),
        "null" => Ok(Null),
        "return" => Ok(Return),
        "super" => Ok(Super),
        "switch" => Ok(Switch),
        "this" => Ok(This),
        "throw" => Ok(Throw),
        "true" => Ok(True),
        "try" => Ok(Try),
        "typeof" => Ok(TypeOf),
        "var" => Ok(Var),
        "void" => Ok(Void),
        "while" => Ok(While),
        "with" => Ok(With),
        "yield" => Ok(Yield),
        "let" => Ok(Let),
        "static" => Ok(Static),
        "implements" => Ok(Implements),
        "interface" => Ok(Interface),
        "package" => Ok(Package),
        "private" => Ok(Private),
        "protected" => Ok(Protected),
        "public" => Ok(Public),
        "as" => Ok(As),
        "async" => Ok(Async),
        "from" => Ok(From),
        "get" => Ok(Get),
        "meta" => Ok(Meta),
        "of" => Ok(Of),
        "set" => Ok(Set),
        "target" => Ok(Target),
        _ => Err("keyword could not be found".to_string()),
    }
}
