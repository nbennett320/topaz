#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // 1 character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // 1 or 2 character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fn,
    If,
    Nil,
    Or,
    Puts,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // Misc tokens
    Error(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub col: usize,
    pub len: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, col: usize, len: usize) -> Token {
        Token {
            token_type,
            line,
            col,
            len,
        }
    }
}
