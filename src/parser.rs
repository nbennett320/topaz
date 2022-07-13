use crate::opcode::Opcode;
use crate::token::{Token, TokenType};
use crate::value::Value;
use crate::vm::InterpretError;
use crate::Chunk;
use crate::Scanner;

pub struct Parser {
    current: Token,
    previous: Token,
    scanner: Scanner,
    chunk: Chunk,
    had_error: bool,
}

enum Precedence {
    None,
    Assignment, // =
    Or,         // ||
    And,        // &&
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

struct ParseRule {
    prefix: Option<fn(parser: &mut Parser)>,
    infix: Option<fn(parser: &mut Parser)>,
    precedence: Precedence,
}

const rules: [ParseRule; 40] = [
    ParseRule {
        prefix: Some(Parser::grouping),
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: Some(Parser::unary),
        infix: Some(Parser::binary),
        precedence: Precedence::Term,
    },
    ParseRule {
        prefix: None,
        infix: Some(Parser::binary),
        precedence: Precedence::Term,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: Some(Parser::binary),
        precedence: Precedence::Factor,
    },
    ParseRule {
        prefix: None,
        infix: Some(Parser::binary),
        precedence: Precedence::Factor,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: Some(Parser::number),
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
];

impl Parser {
    pub fn new(source: String) -> Parser {
        Parser {
            current: Token::new(TokenType::Error(String::from("current token")), 0, 0, 0),
            previous: Token::new(TokenType::Error(String::from("current token")), 0, 0, 0),
            scanner: Scanner::new(source),
            chunk: Chunk::new(),
            had_error: false,
        }
    }

    pub fn compile(mut self) -> Result<Chunk, InterpretError> {
        Ok(self.chunk)
    }

    fn expression(&mut self) {
        // self.parse_precedence(Precedence::Assignment);
    }

    pub fn advance(&mut self) {
        self.previous = self.current.clone();

        while let Some(tok) = self.scanner.next() {
            self.current = tok;
        }
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) {
        if !(self.current.token_type == token_type) {
            self.error_at_current(msg);
            return;
        }

        self.advance();
    }

    fn error(&mut self, msg: &str) {
        self.error_at(self.previous.clone(), msg);
    }

    fn error_at_current(&mut self, msg: &str) {
        self.error_at(self.current.clone(), msg);
    }

    fn error_at(&mut self, tok: Token, msg: &str) {
        println!("[line {}] Error: {}", tok.line, msg);
        self.had_error = true;
    }

    fn number(&mut self) {
        if let TokenType::Number(num) = self.previous.token_type {
            self.emit_constant(Value::Number(num));
        }
    }

    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write(byte, self.previous.line);
    }

    fn emit_bytes(&mut self, a: u8, b: u8) {
        self.emit_byte(a);
        self.emit_byte(b);
    }

    fn emit_op(&mut self, op: Opcode) {
        self.emit_byte(op as u8);
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value) as u8;
        self.emit_bytes(Opcode::Constant as u8, constant);
    }

    fn make_constant(&mut self, value: Value) -> usize {
        let constant = self.chunk.add_constant(value);
        if constant > std::u8::MAX as usize {
            self.error("Too many constants in this chunk");
            0
        } else {
            constant
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression");
    }

    fn unary(&mut self) {
        let operator = self.previous.token_type.clone();

        //self.parse_precedence(Precedence::Unary);

        match operator {
            TokenType::Minus => self.emit_op(Opcode::Negate),
            _ => (),
        }
    }

    fn binary(&mut self) {
        let operator = self.previous.token_type.clone();

        //self.parse_precedence();

        match operator {
            TokenType::Plus => self.emit_op(Opcode::Add),
            TokenType::Minus => self.emit_op(Opcode::Subtract),
            TokenType::Star => self.emit_op(Opcode::Multiply),
            TokenType::Slash => self.emit_op(Opcode::Divide),
            _ => (),
        }
    }
}
