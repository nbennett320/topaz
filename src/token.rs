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

pub fn to_usize(token_type: TokenType) -> usize {
    match token_type {
        TokenType::LeftParen => 0,
        TokenType::RightParen => 1,
        TokenType::LeftBrace => 2,
        TokenType::RightBrace => 3,
        TokenType::Comma => 4,
        TokenType::Dot => 5,
        TokenType::Minus => 6,
        TokenType::Plus => 7,
        TokenType::Semicolon => 8,
        TokenType::Slash => 9,
        TokenType::Star => 10,
        TokenType::Bang => 11,
        TokenType::BangEqual => 12,
        TokenType::Equal => 13,
        TokenType::EqualEqual => 14,
        TokenType::Greater => 15,
        TokenType::GreaterEqual => 16,
        TokenType::Less => 17,
        TokenType::LessEqual => 18,
        TokenType::Identifier(_) => 19,
        TokenType::String(_) => 20,
        TokenType::Number(_) => 21,
        TokenType::And => 22,
        TokenType::Class => 23,
        TokenType::Else => 24,
        TokenType::False => 25,
        TokenType::For => 26,
        TokenType::Fn => 27,
        TokenType::If => 28,
        TokenType::Nil => 29,
        TokenType::Or => 30,
        TokenType::Puts => 31,
        TokenType::Return => 32,
        TokenType::Super => 33,
        TokenType::This => 34,
        TokenType::True => 35,
        TokenType::Var => 36,
        TokenType::While => 37,
        TokenType::Error(_) => 38,
    }
}
