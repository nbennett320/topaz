use crate::opcode::Opcode;
use crate::precedence::Precedence;
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
    locals: Vec<Local>,
    had_error: bool,
    end_flag: bool,
    local_count: usize,
    scope_depth: usize,
}

/// represents a local variable
struct Local {
    name: String,
    depth: usize,
}

impl Parser {
    pub fn new(source: String) -> Parser {
        Parser {
            current: Token::new(TokenType::Error(String::from("current token")), 0, 0, 0),
            previous: Token::new(TokenType::Error(String::from("current token")), 0, 0, 0),
            scanner: Scanner::new(source),
            chunk: Chunk::new(),
            locals: Vec::new(),
            had_error: false,
            end_flag: false,
            local_count: 0,
            scope_depth: 0,
        }
    }

    pub fn compile(mut self) -> Result<Chunk, InterpretError> {
        self.advance();

        while !self.end_flag {
            self.declaration();
        }

        self.emit_op(Opcode::Return);
        Ok(self.chunk)
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn declaration(&mut self) {
        if let TokenType::Identifier(name) = self.current.token_type.clone() {
            self.advance();
            if self.matches(TokenType::Equal) {
                self.variable_definition(name);
                return;
            }
        }

        self.statement();
    }

    fn statement(&mut self) {
        match self.current.token_type.clone() {
            TokenType::Print => {
                self.advance();
                self.print_statement();
            }
            TokenType::LeftBrace => {
                self.advance();
                self.begin_scope();
                self.block();
                self.end_scope();
            }
            TokenType::If => {
                self.advance();
                self.if_statement();
            }
            _ => self.expression_statement(),
        }
    }

    fn expression_statement(&mut self) {
        self.expression();
        self.emit_op(Opcode::Pop);
    }

    fn block(&mut self) {
        while self.current.token_type != TokenType::RightBrace {
            self.declaration()
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block");
    }

    fn if_statement(&mut self) {
        self.expression();
        let if_offset = self.emit_jump(Opcode::JumpIfFalse);
        self.emit_op(Opcode::Pop);
        self.statement();

        let else_offset = self.emit_jump(Opcode::Jump);
        self.emit_op(Opcode::Pop);

        self.patch_jump(if_offset);

        // compile optional else clause
        if self.matches(TokenType::Else) {
            self.statement();
        }

        self.patch_jump(else_offset);
    }

    pub fn conditional(&mut self, val: bool) {
        println!("executing conditional: {}", val);
        println!(
            "token_type: {}, current: {:?}",
            TokenType::If,
            self.current.token_type
        );
        self.consume(
            TokenType::LeftParen,
            "Expect expression to be evaluated as boolean",
        );
        self.expression();
        self.consume(
            TokenType::RightParen,
            "Expect expression to be evaluated as boolean",
        );
        let then_jump = self.emit_jump(Opcode::JumpIfFalse);
        self.statement();
        self.patch_jump(then_jump);
    }

    pub fn advance(&mut self) {
        self.previous = self.current.clone();

        if let Some(tok) = self.scanner.next() {
            self.current = tok;
        } else {
            self.end_flag = true;
        }
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) {
        if !(self.current.token_type == token_type) {
            self.error_at_current(msg);
        }

        self.advance();
    }

    pub fn string(&mut self, _can_assign: bool) {
        match &self.previous.token_type {
            TokenType::String(s) => self.emit_constant(Value::String(s.to_string())),
            _ => unreachable!("No string"),
        }
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

    pub fn number(&mut self, _can_assign: bool) {
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

    fn emit_ops(&mut self, op1: Opcode, op2: Opcode) {
        self.emit_byte(op1 as u8);
        self.emit_byte(op2 as u8);
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value) as u8;
        self.emit_bytes(Opcode::Constant as u8, constant);
    }

    fn emit_jump(&mut self, op: Opcode) -> usize {
        self.emit_byte(op as u8);
        self.emit_bytes(0xff, 0xff);
        self.chunk.code.len() - 2
    }

    fn patch_jump(&mut self, offset: usize) {
        let jump = self.chunk.code.len() - offset - 2;

        if jump > std::i16::MAX as usize {
            self.error("Jump is out of bounds");
        }

        self.chunk.code[offset] = ((jump >> 8) & 0xff) as u8;
        self.chunk.code[offset + 1] = (jump & 0xff) as u8;
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

    pub fn grouping(&mut self, _can_assign: bool) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression");
    }

    pub fn unary(&mut self, _can_assign: bool) {
        let operator = self.previous.token_type.clone();
        self.parse_precedence(Precedence::Unary);

        match operator {
            TokenType::Minus => self.emit_op(Opcode::Negate),
            TokenType::Bang => self.emit_op(Opcode::Not),
            _ => unreachable!("Impossible unary operator"),
        }
    }

    pub fn binary(&mut self, _can_assign: bool) {
        let operator = self.previous.token_type.clone();
        let rule = operator.rule();
        let precedence = Precedence::from(rule.precedence as usize + 1);
        self.parse_precedence(precedence);

        match operator {
            TokenType::Plus => self.emit_op(Opcode::Add),
            TokenType::Minus => self.emit_op(Opcode::Subtract),
            TokenType::Star => self.emit_op(Opcode::Multiply),
            TokenType::Slash => self.emit_op(Opcode::Divide),
            TokenType::Mod => self.emit_op(Opcode::Mod),
            TokenType::BangEqual => self.emit_ops(Opcode::Equal, Opcode::Not),
            TokenType::EqualEqual => self.emit_op(Opcode::Equal),
            TokenType::Greater => self.emit_op(Opcode::Greater),
            TokenType::GreaterEqual => self.emit_ops(Opcode::Less, Opcode::Not),
            TokenType::Less => self.emit_op(Opcode::Less),
            TokenType::LessEqual => self.emit_ops(Opcode::Greater, Opcode::Not),
            TokenType::BitwiseAnd => self.emit_op(Opcode::BitwiseAnd),
            TokenType::BitwiseOr => self.emit_op(Opcode::BitwiseOr),
            TokenType::LogicalAnd => self.emit_op(Opcode::LogicalAnd),
            TokenType::LogicalOr => self.emit_op(Opcode::LogicalOr),
            _ => (),
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let rule = self.previous.token_type.rule();

        if let Some(prefix_rule) = rule.prefix {
            let can_assign = precedence as usize <= Precedence::Assignment as usize;
            prefix_rule(self, can_assign);

            let prec_u8 = precedence as u8;
            while prec_u8 <= self.current.token_type.rule().precedence as u8 {
                self.advance();
                if let Some(infix_rule) = self.previous.token_type.rule().infix {
                    infix_rule(self, can_assign);
                }
            }

            return;
        }

        self.error("Expected expression");
    }

    pub fn literal(&mut self, _can_assign: bool) {
        let token_type = self.previous.token_type.clone();
        match token_type {
            TokenType::False => self.emit_op(Opcode::False),
            TokenType::Nil => self.emit_op(Opcode::Nil),
            TokenType::True => self.emit_op(Opcode::True),
            _ => unreachable!("Impossible TokenType in literal"),
        }
    }

    fn print_statement(&mut self) {
        self.expression();
        self.emit_op(Opcode::Print);
    }

    fn matches(&mut self, token_type: TokenType) -> bool {
        if self.current.token_type == token_type {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn variable(&mut self, can_assign: bool) {
        if let TokenType::Identifier(name) = self.previous.token_type.clone() {
            let (get_op, set_op, var) = match self.resolve_local(&self.previous) {
                Ok(id) => (Opcode::GetLocal, Opcode::SetLocal, id),
                Err(_) => (
                    Opcode::GetGlobal,
                    Opcode::SetGlobal,
                    self.make_constant(Value::String(name)),
                ),
            };
            if can_assign && self.matches(TokenType::Equal) {
                self.expression();
                self.emit_op(set_op);
            } else {
                self.emit_op(get_op);
            }
            self.emit_byte(var as u8);
        }
    }

    fn variable_definition(&mut self, name: String) {
        self.expression();

        // local variable
        if self.scope_depth > 0 {
            self.add_local(name);
        }
        // global variable
        else {
            let global = self.make_constant(Value::String(name));
            self.emit_op(Opcode::DefineGlobal);
            self.emit_byte(global as u8);
        }
    }

    fn add_local(&mut self, name: String) {
        // no more than 255 local variables
        if self.local_count == u8::MAX as usize {
            self.error("Too many local variables");
            return;
        }

        let local = Local {
            name,
            depth: self.scope_depth,
        };

        self.local_count += 1;
        self.locals.push(local);
    }

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;

        // pop local variables introduced in this scope off  the stack
        while self.local_count > 0 && self.locals[self.local_count - 1].depth > self.scope_depth {
            self.emit_op(Opcode::Pop);
            self.local_count -= 1;
        }
    }

    fn resolve_local(&self, name: &Token) -> Result<usize, ()> {
        if self.locals.is_empty() {
            return Err(());
        }

        let mut local_count = self.local_count - 1;
        let identifier = match &name.token_type {
            TokenType::Identifier(id) => id,
            _ => unreachable!("Was not given an identifier to resolve_local"),
        };

        loop {
            let local = &self.locals[local_count];
            if local.name == *identifier {
                return Ok(local_count);
            }

            if local_count == 0 {
                break;
            }

            local_count -= 1;
        }

        Err(())
    }
}
