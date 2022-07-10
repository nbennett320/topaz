use crate::token::{Token, TokenType};

struct Scanner {
    source: String,
    start: usize, // index of beginning of lexeme being scanned
    pos: usize,   // current character being looked at
    line: usize,
}

impl Scanner {
    fn new(source: String) -> Scanner {
        Scanner {
            source,
            start: 0,
            pos: 0,
            line: 1,
        }
    }

    fn scan_all(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(tok) = self.next() {
            tokens.push(tok);
        }
        tokens
    }

    fn next(&mut self) -> Option<Token> {
        // ignores whitespace between tokens
        self.skip_whitespace();

        self.start = self.pos;

        if self.eof() {
            return None;
        }

        let tok = match self.peek() {
            '(' => Some(self.make_token(TokenType::LeftParen)),
            ')' => Some(self.make_token(TokenType::RightParen)),
            '{' => Some(self.make_token(TokenType::LeftBrace)),
            '}' => Some(self.make_token(TokenType::RightBrace)),
            ';' => Some(self.make_token(TokenType::Semicolon)),
            ',' => Some(self.make_token(TokenType::Comma)),
            '.' => Some(self.make_token(TokenType::Dot)),
            '-' => Some(self.make_token(TokenType::Minus)),
            '+' => Some(self.make_token(TokenType::Plus)),
            '/' => Some(self.make_token(TokenType::Slash)),
            '*' => Some(self.make_token(TokenType::Star)),
            '!' => {
                let token_type = if self.matches('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                Some(self.make_token(token_type))
            }
            '=' => {
                let token_type = if self.matches('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                Some(self.make_token(token_type))
            }
            '<' => {
                let token_type = if self.matches('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                Some(self.make_token(token_type))
            }
            '>' => {
                let token_type = if self.matches('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                Some(self.make_token(token_type))
            }
            '\'' => Some(self.string()),
            _ => None,
        };

        tok
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        let len = self.pos - self.start;
        Token::new(token_type, self.line, self.start, len)
    }

    /// emits a syntax error token
    fn error(&self, msg: &str) -> Token {
        Token::new(
            TokenType::Error(String::from(msg)),
            self.line,
            self.start,
            msg.len(),
        )
    }

    fn peek(&self) -> char {
        self.source[self.pos..].chars().next().unwrap()
    }

    /// advances the scanner's position and returns the consumed character
    fn advance(&mut self) -> char {
        let c = self.peek();
        self.pos += 1;
        c
    }

    /// returns true if next character matches the expected character, consuming it
    fn matches(&mut self, expected: char) -> bool {
        if self.eof() {
            false
        } else if self.peek() != expected {
            false
        } else {
            self.pos += 1;
            true
        }
    }

    fn string(&mut self) -> Token {
        while self.peek() != '\'' && !self.eof() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.eof() {
            self.error("Unterminated string")
        } else {
            // consume closing '
            self.advance();

            let string = &self.source[self.start..self.pos];
            self.make_token(TokenType::String(String::from(string)))
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                    ()
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                    ()
                }
                '#' => {
                    while self.peek() != '\n' && !self.eof() {
                        self.advance();
                    }
                    ()
                }
                _ => break,
            }
        }
    }

    fn eof(&self) -> bool {
        self.pos >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_source_returns_empty_vector_of_tokens() {
        let mut scanner = Scanner::new(String::from(""));
        assert_eq!(scanner.scan_all().len(), 0);
    }

    #[test]
    fn scans_keyword_return() {
        let mut scanner = Scanner::new(String::from("return"));
        let tokens = scanner.scan_all();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::new(TokenType::Return, 1, 0, 6));
    }

    #[test]
    fn scans_list_of_keywords() {
        let mut scanner = Scanner::new(String::from("nil if fn"));
        let tokens = scanner.scan_all();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::new(TokenType::Nil, 1, 0, 3));
        assert_eq!(tokens[1], Token::new(TokenType::If, 1, 5, 2));
        assert_eq!(tokens[2], Token::new(TokenType::Fn, 1, 8, 2));
    }

    #[test]
    fn ignores_whitespace() {
        let mut scanner = Scanner::new(String::from("  \tnil"));
        let tokens = scanner.scan_all();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::new(TokenType::Nil, 1, 3, 3));
    }

    #[test]
    fn finds_string() {
        let mut scanner = Scanner::new(String::from("'hello world'"));
        let tokens = scanner.scan_all();
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens[0],
            Token::new(TokenType::String(String::from("hello world")), 1, 1, 13)
        );
    }
}
