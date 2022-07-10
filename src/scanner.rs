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
		self.start = self.pos;

		None
	}

	fn make_token(&self, token_type: TokenType) -> Token {
		let len = self.pos - self.start;
		Token::new(token_type, self.line, self.start, len)
	}
	fn skip_whitespace(&mut self) {
		loop {
			match self.peek() {
				' ' | '\r' | '\t' => {
					self.advance();
					()
				},
				'\n' => {
					self.line += 1;
					self.advance();
					()
				},
				'#' => {
					while self.peek() != '\n' && !self.eof() {
						self.advance();
					}
					()
				},
				_ => break
			}
		}
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
}
