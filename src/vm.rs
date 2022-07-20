use crate::function::Function;
use crate::opcode::{from_u8, Opcode};
use crate::value::Value;

use std::collections::HashMap;

pub struct Vm {
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
    frames: Vec<CallFrame>,
}

pub enum InterpretError {
    CompileError,
    RuntimeError,
}

struct CallFrame {
    function: Function,
    ip: usize,   // ip of caller to return to
    base: usize, // index of base of stack
}

impl CallFrame {
    pub fn new(function: Function, base: usize) -> CallFrame {
        CallFrame {
            function,
            ip: 0,
            base,
        }
    }
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            stack: Vec::new(),
            globals: HashMap::new(),
            frames: Vec::new(),
        }
    }

    pub fn run(&mut self, function: Function) -> Result<(), InterpretError> {
        // push "stack frame" of top level script onto stack
        let cf = CallFrame::new(function, 0);
        self.frames.push(cf);

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
                    let result = self.pop();
                    self.frames.pop();

                    if self.frames.is_empty() {
                        return Ok(());
                    }

                    self.push(result);
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
                Opcode::Add => self.binary_op('+'),
                Opcode::Subtract => self.binary_op('-'),
                Opcode::Multiply => self.binary_op('*'),
                Opcode::Divide => self.binary_op('/'),
                Opcode::Mod => self.binary_op('%'),
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
                    } else if let Value::Function(f) = constant {
                        self.globals
                            .insert(f.name.clone(), Value::Function(f.clone()));
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
                    let base = self.frames.last_mut().unwrap().base;
                    let slot = self.read_byte() as usize;
                    self.push(self.stack[base + slot].clone());
                }
                Opcode::SetLocal => {
                    let base = self.frames.last_mut().unwrap().base;
                    let slot = self.read_byte() as usize;
                    self.stack[base + slot] = self.peek(0).clone();
                }
                Opcode::JumpIfFalse => {
                    let offset = self.read_short() as usize;
                    if self.peek(0).is_falsey() {
                        self.frames.last_mut().unwrap().ip += offset;
                    }
                }
                Opcode::Jump => {
                    let offset = self.read_short() as usize;
                    self.frames.last_mut().unwrap().ip += offset;
                }
                Opcode::Loop => {
                    let offset = self.read_short() as usize;
                    self.frames.last_mut().unwrap().ip -= offset;
                }
                Opcode::Call => {
                    let num_args = self.read_byte() as usize;
                    let function = self.peek(num_args);
                    let f = match function {
                        Value::Function(f) => f,
                        _ => {
                            return Err(InterpretError::RuntimeError);
                        }
                    };

                    let cf = CallFrame::new(f.clone(), self.stack.len() - num_args);
                    self.frames.push(cf);
                }
                _ => return Err(InterpretError::CompileError),
            };
        }
    }

    fn runtime_error(&mut self, msg: &str) -> InterpretError {
        let ip = self.frames.last_mut().unwrap().ip;
        let line = self.frames.last_mut().unwrap().function.chunk.lines[ip - 1];
        println!("{} [line {}]", msg, line);
        InterpretError::RuntimeError
    }

    fn read_byte(&mut self) -> u8 {
        let mut ip = self.frames.last_mut().unwrap().ip;
        let byte = self.frames.last_mut().unwrap().function.chunk.code[ip];
        self.frames.last_mut().unwrap().ip += 1;
        byte
    }

    fn read_short(&mut self) -> u16 {
        let ip = self.frames.last_mut().unwrap().ip;
        let rs = &self.frames.last_mut().unwrap().function.chunk.code[ip..=ip + 1];
        let short: u16 = ((rs[0] as u16) << 8) | rs[1] as u16;
        self.frames.last_mut().unwrap().ip += 2;
        short
    }

    fn read_constant(&mut self) -> Value {
        let byte = self.read_byte();
        self.frames.last_mut().unwrap().function.chunk.constants[byte as usize].clone()
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

        match (val1, val2) {
            (Value::Number(a), Value::Number(b)) => {
                let result = match op {
                    '+' => Value::Number(a + b),
                    '-' => Value::Number(a - b),
                    '*' => Value::Number(a * b),
                    '/' => Value::Number(a / b),
                    '%' => Value::Number(a % b),
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
                    'A' => Value::Bool(a != 0f64 && b != 0f64),
                    'O' => Value::Bool(a != 0f64 || b != 0f64),
                    _ => unreachable!("binary_op: invalid op {}", op),
                };

                self.push(result)
            }
            (Value::Bool(n), Value::Number(m)) => {
                let (a, b) = (1f64, m);

                let result = match op {
                    '+' | '-' | '*' | '/' | '%' | '>' | '<' => {
                        self.runtime_error("operands must be numbers");
                        Value::Nil
                    }
                    '&' => Value::Number((a as i64 & b.round() as i64) as f64),
                    '|' => Value::Number((a as i64 | b.round() as i64) as f64),
                    'A' => Value::Bool(n && b != 0f64),
                    'O' => Value::Bool(n || b != 0f64),
                    _ => unreachable!("binary_op: invalid op {}", op),
                };

                self.push(result)
            }
            (Value::Number(n), Value::Bool(m)) => {
                let (a, b) = (n, 1f64);

                let result = match op {
                    '+' | '-' | '*' | '/' | '%' | '>' | '<' => {
                        self.runtime_error("operands must be numbers");
                        Value::Nil
                    }
                    '&' => Value::Number((a.round() as i64 & b as i64) as f64),
                    '|' => Value::Number((a.round() as i64 | b as i64) as f64),
                    'A' => Value::Bool(a != 0f64 && m),
                    'O' => Value::Bool(a != 0f64 || m),
                    _ => unreachable!("binary_op: invalid op {}", op),
                };

                self.push(result)
            }
            (Value::Bool(n), Value::Bool(m)) => {
                let (a, b) = (1f64, 1f64);

                let result = match op {
                    '+' | '-' | '*' | '/' | '%' | '>' | '<' => {
                        self.runtime_error("operands must be numbers");
                        Value::Nil
                    }
                    '&' => Value::Number((a as i64 & b as i64) as f64),
                    '|' => Value::Number((a as i64 | b as i64) as f64),
                    'A' => Value::Bool(n && m),
                    'O' => Value::Bool(n || m),
                    _ => unreachable!("binary_op: invalid op {}", op),
                };

                self.push(result)
            }
            (Value::String(a), Value::String(b)) => {
                let result: Value = match op {
                    '+' => Value::String(format!("{}{}", a, b)),
                    '-' | '*' | '/' | '%' | '>' | '<' | '&' | '|' => {
                        let msg = format!("no {} operation on string '{}' and '{}'", op, a, b);
                        self.runtime_error(&msg);
                        Value::Nil
                    }
                    'A' => Value::Bool(a.len() > 0 && b.len() > 0),
                    'O' => Value::Bool(a.len() > 0 || b.len() > 0),
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
