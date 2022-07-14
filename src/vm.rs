use crate::chunk::Chunk;
use crate::opcode::{from_u8, Opcode};
use crate::value::Value;

use std::collections::HashMap;

pub struct Vm {
    ip: usize,
    chunk: Chunk,
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
}

pub enum InterpretError {
    CompileError,
    RuntimeError,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            ip: 0,
            chunk: Chunk::new(),
            stack: Vec::new(),
            globals: HashMap::new(),
        }
    }

    pub fn run(&mut self, chunk: Chunk) -> Result<(), InterpretError> {
        self.chunk = chunk;
        loop {
            // debug information
            if cfg!(debug_assertions) {
                print!("stack:          ");
                print!("[ ");
                for value in &mut self.stack {
                    value.print();
                    print!(" ");
                }
                print!("]");
                println!();
            }

            let instruction = self.read_byte();
            match from_u8(instruction) {
                Opcode::Return => {
                    //self.pop().print();
                    //println!();
                    break;
                }
                Opcode::Constant => {
                    let constant = self.read_constant();
                    self.push(constant);
                }
                Opcode::Negate => {
                    let value = self.pop();
                    let negated_value = match value {
                        Value::Number(num) => Value::Number(-num),
                        _ => return Err(self.runtime_error("Operand must be a number")),
                    };
                    self.push(negated_value)
                }
                Opcode::Add => {
                    if let (Value::String(mut a), Value::String(b)) =
                        (self.peek(1).clone(), self.peek(0))
                    {
                        a.push_str(b);
                        self.push(Value::String(a))
                    } else {
                        self.binary_op('+')
                    }
                }
                Opcode::Subtract => self.binary_op('-'),
                Opcode::Multiply => self.binary_op('*'),
                Opcode::Divide => self.binary_op('/'),
                Opcode::Nil => self.push(Value::Nil),
                Opcode::True => self.push(Value::Bool(true)),
                Opcode::False => self.push(Value::Bool(false)),
                Opcode::Not => {
                    let value = self.pop().is_falsey();
                    self.push(Value::Bool(value))
                }
                Opcode::Equal => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Bool(a.eq(&b)));
                }
                Opcode::Greater => self.binary_op('>'),
                Opcode::Less => self.binary_op('<'),
                Opcode::LogicalAnd => self.binary_op('A'),
                Opcode::LogicalOr => self.binary_op('O'),
                Opcode::BitwiseAnd => self.binary_op('&'),
                Opcode::BitwiseOr => self.binary_op('|'),
                Opcode::Print => {
                    self.pop().print();
                    println!();
                }
                Opcode::Pop => {
                    self.pop();
                }
                Opcode::DefineGlobal => {
                    let constant = self.read_constant();
                    if let Value::String(name) = constant {
                        self.globals.insert(name, self.peek(0).clone());
                        self.pop();
                    } else {
                        unreachable!("Did not receive a String in DefineGlobal")
                    }
                }
                Opcode::GetGlobal => {
                    let constant = self.read_constant();
                    if let Value::String(name) = constant {
                        match self.globals.get(&name) {
                            Some(val) => self.push(val.clone()),
                            None => {
                                self.runtime_error(
                                    format!("Undefined variable {}", &name).as_str(),
                                );
                                return Err(InterpretError::RuntimeError);
                            }
                        }
                    } else {
                        unreachable!("Did not receive a String in DefineGlobal")
                    }
                }
                _ => return Err(InterpretError::CompileError),
            };
        }

        self.ip = 0;
        Ok(())
    }

    fn runtime_error(&mut self, msg: &str) -> InterpretError {
        let line = self.chunk.lines[self.ip - 1];
        println!("{} [line {}]", msg, line);
        InterpretError::RuntimeError
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.chunk.code[self.ip];
        self.ip += 1;
        byte
    }

    fn read_constant(&mut self) -> Value {
        let byte = self.read_byte();
        self.chunk.constants[byte as usize].clone()
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value)
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    fn peek(&self, offset: usize) -> &Value {
        let len = self.stack.len();
        &self.stack[len - 1 - offset]
    }

    fn binary_op(&mut self, op: char) {
        let val2 = self.pop();
        let val1 = self.pop();

        let (a, b) = if let (Value::Number(a), Value::Number(b)) = (val1.clone(), val2) {
            (a, b)
        } else {
            self.runtime_error("Operands must be numbers");
            self.push(val1);
            return;
        };

        let result = match op {
            '+' => Value::Number(a + b),
            '-' => Value::Number(a - b),
            '*' => Value::Number(a * b),
            '/' => Value::Number(a / b),
            '>' => Value::Bool(a > b),
            '<' => Value::Bool(a < b),
            '&' => {
                let a_diff = (a - a.round()).abs();
                let b_diff = (b - b.round()).abs();

                if a_diff > 0f64 || b_diff > 0f64 {
                    self.runtime_error("Cannot use fp operands for & operator");
                }

                Value::Number((a.round() as i64 & b.round() as i64) as f64)
            }
            '|' => {
                let a_diff = (a - a.round()).abs();
                let b_diff = (b - b.round()).abs();

                if a_diff > 0f64 || b_diff > 0f64 {
                    self.runtime_error("Cannot use fp operands for | operator");
                }

                Value::Number((a.round() as i64 | b.round() as i64) as f64)
            }
            // 'A' => Value::Bool(a as bool && b),
            // 'O' => Value::Bool(a || b),
            _ => unreachable!("binary_op: invalid op {}", op),
        };

        self.push(result)
    }
}
