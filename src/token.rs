#[derive(Debug, PartialEq)]
pub enum TokenType {
    // 1 character tokens
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // 1 or 2 character tokens
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals
    IDENTIFIER(String),
    STRING(String),
    NUMBER(f64),

    // Keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FOR,
    FN,
    IF,
    NIL,
    OR,
    PUTS,
    RETURN,
    SUPER,
    SELF,
    TRUE,
    VAR,
    WHILE,

    ERROR,
    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    line: usize,
    col: usize,
    len: usize,
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
