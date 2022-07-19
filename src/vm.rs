use crate::chunk::Chunk;
use crate::opcode::{from_u8, Opcode};
use crate::value::Value;
use crate::operator::Operator;

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
                    print!("{} ", value);
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
                Opcode::Add => self.binary_op(Operator::Plus),
                Opcode::Subtract => self.binary_op(Operator::Minus),
                Opcode::Multiply => self.binary_op(Operator::Star),
                Opcode::Divide => self.binary_op(Operator::Slash),
                Opcode::Mod => self.binary_op(Operator::Mod),
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
                Opcode::Greater => self.binary_op(Operator::GreaterThan),
                Opcode::Less => self.binary_op(Operator::LessThan),
                Opcode::LogicalAnd => self.binary_op(Operator::AmpAmp),
                Opcode::LogicalOr => self.binary_op(Operator::PipePipe),
                Opcode::BitwiseAnd => self.binary_op(Operator::Amp),
                Opcode::BitwiseOr => self.binary_op(Operator::Pipe),
                Opcode::Print => {
                    print!("{}\n", self.pop());
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
                        unreachable!("Did not receive a String in GetGlobal")
                    }
                }
                Opcode::SetGlobal => {
                    let constant = self.read_constant();
                    if let Value::String(name) = constant {
                        self.globals.insert(name, self.peek(0).clone());
                        self.pop();
                    } else {
                        unreachable!("Did not receive a String in SetGlobal")
                    }
                }
                Opcode::GetLocal => {
                    let slot = self.read_byte() as usize;
                    self.push(self.stack[slot].clone());
                }
                Opcode::SetLocal => {
                    let slot = self.read_byte() as usize;
                    self.stack[slot] = self.peek(0).clone();
                }
                Opcode::JumpIfFalse => {
                    let offset = self.read_short() as usize;
                    if self.peek(0).is_falsey() {
                        self.ip += offset;
                    }
                }
                Opcode::Jump => {
                    let offset = self.read_short() as usize;
                    self.ip += offset;
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

    fn read_short(&mut self) -> u16 {
        let rs = &self.chunk.code[self.ip..=self.ip + 1];
        let short: u16 = ((rs[0] as u16) << 8) | rs[1] as u16;
        self.ip += 2;
        short
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

    fn binary_op(&mut self, op: Operator) {
        let val2 = self.pop();
        let val1 = self.pop();

        match (val1, val2) {
            (Value::Number(a), Value::Number(b)) => {
                let result = match op {
                    Operator::Plus => Value::Number(a + b),
                    Operator::Minus => Value::Number(a - b),
                    Operator::Star => Value::Number(a * b),
                    Operator::Slash => Value::Number(a / b),
                    Operator::Mod => Value::Number(a % b),
                    Operator::GreaterThan => Value::Bool(a > b),
                    Operator::LessThan => Value::Bool(a < b),
                    Operator::Amp => {
                        let a_diff = (a - a.round()).abs();
                        let b_diff = (b - b.round()).abs();

                        if a_diff > 0f64 || b_diff > 0f64 {
                            self.runtime_error("Cannot use fp operands for & operator");
                        }

                        Value::Number((a.round() as i64 & b.round() as i64) as f64)
                    }
                    Operator::Pipe => {
                        let a_diff = (a - a.round()).abs();
                        let b_diff = (b - b.round()).abs();

                        if a_diff > 0f64 || b_diff > 0f64 {
                            self.runtime_error("Cannot use fp operands for | operator");
                        }

                        Value::Number((a.round() as i64 | b.round() as i64) as f64)
                    }
                    Operator::AmpAmp => Value::Bool(a != 0f64 && b != 0f64),
                    Operator::PipePipe => Value::Bool(a != 0f64 || b != 0f64),
                    _ => unreachable!("binary_op: invalid op {}", op),
                };

                self.push(result)
            }
            (Value::Bool(n), Value::Number(m)) => {
                let (a, b) = (1f64, m);

                let result = match op {
                    Operator::Plus 
                    | Operator::Minus 
                    | Operator::Star
                    | Operator::Slash 
                    | Operator::Mod 
                    | Operator::GreaterThan 
                    | Operator::LessThan => {
                        self.runtime_error("operands must be numbers");
                        Value::Nil
                    }
                    Operator::Amp => Value::Number((a as i64 & b.round() as i64) as f64),
                    Operator::Pipe => Value::Number((a as i64 | b.round() as i64) as f64),
                    Operator::AmpAmp => Value::Bool(n && b != 0f64),
                    Operator::PipePipe => Value::Bool(n || b != 0f64),
                    _ => unreachable!("binary_op: invalid op {}", op),
                };

                self.push(result)
            }
            (Value::Number(n), Value::Bool(m)) => {
                let (a, b) = (n, 1f64);

                let result = match op {
                    Operator::Plus 
                    | Operator::Minus 
                    | Operator::Star
                    | Operator::Slash 
                    | Operator::Mod 
                    | Operator::GreaterThan 
                    | Operator::LessThan => {
                        self.runtime_error("operands must be numbers");
                        Value::Nil
                    }
                    Operator::Amp => Value::Number((a.round() as i64 & b as i64) as f64),
                    Operator::Pipe => Value::Number((a.round() as i64 | b as i64) as f64),
                    Operator::AmpAmp => Value::Bool(a != 0f64 && m),
                    Operator::PipePipe => Value::Bool(a != 0f64 || m),
                    _ => unreachable!("binary_op: invalid op {}", op),
                };

                self.push(result)
            }
            (Value::Bool(n), Value::Bool(m)) => {
                let (a, b) = (1f64, 1f64);

                let result = match op {
                    Operator::Plus 
                    | Operator::Minus 
                    | Operator::Star
                    | Operator::Slash 
                    | Operator::Mod 
                    | Operator::GreaterThan 
                    | Operator::LessThan => {
                        self.runtime_error("operands must be numbers");
                        Value::Nil
                    }
                    Operator::Amp => Value::Number((a as i64 & b as i64) as f64),
                    Operator::Pipe => Value::Number((a as i64 | b as i64) as f64),
                    Operator::AmpAmp => Value::Bool(n && m),
                    Operator::PipePipe => Value::Bool(n || m),
                    _ => unreachable!("binary_op: invalid op {}", op),
                };

                self.push(result)
            }
            (Value::String(a), Value::String(b)) => {
                let result: Value = match op {
                    Operator::Plus => Value::String(format!("{}{}", a, b)),
                    Operator::Minus 
                    | Operator::Star
                    | Operator::Slash 
                    | Operator::Mod 
                    | Operator::GreaterThan 
                    | Operator::LessThan => {
                        let msg = format!("no {} operation on string '{}' and '{}'", op, a, b);
                        self.runtime_error(&msg);
                        Value::Nil
                    }
                    Operator::AmpAmp => Value::Bool(a.len() > 0 && b.len() > 0),
                    Operator::PipePipe => Value::Bool(a.len() > 0 || b.len() > 0),
                    _ => unreachable!("binary_op: invalid op {}", op),
                };

                self.push(result)
            }
            _ => {
                unreachable!("binary_op: invalid op {}", op);
            }
        }
    }
}
