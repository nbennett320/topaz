use crate::chunk::Chunk;
use crate::opcode::{from_u8, Opcode};
use crate::value::{print, Value};

pub struct Vm {
    ip: usize,
    chunk: Chunk,
    stack: Vec<Value>,
}

pub enum InterpretError {
    CompileError,
    RuntimeError,
}

impl Vm {
    pub fn new(chunk: Chunk) -> Vm {
        Vm {
            ip: 0,
            chunk,
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), InterpretError> {
        loop {
            // debug information
            if cfg!(debug_assertions) {
                print!("stack:          ");
                print!("[ ");
                for value in &mut self.stack {
                    print(&value);
                    print!(" ");
                }
                print!("]");
                print!("\n");
            }

            let instruction = self.read_byte();
            match from_u8(instruction) {
                Opcode::Return => {
                    print(&self.pop());
                    print!("\n");
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
                    };
                    self.push(negated_value)
                }
                Opcode::Add => self.binary_op('+'),
                Opcode::Subtract => self.binary_op('-'),
                Opcode::Multiply => self.binary_op('*'),
                Opcode::Divide => self.binary_op('/'),
                _ => return Err(InterpretError::CompileError),
            };
        }

        Ok(())
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.chunk.code[self.ip];
        self.ip += 1;
        byte
    }

    fn read_constant(&mut self) -> Value {
        let byte = self.read_byte();
        self.chunk.constants[byte as usize]
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value)
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    fn binary_op(&mut self, op: char) {
        let val2 = self.pop();
        let val1 = self.pop();

        let (a, b) = if let (Value::Number(a), Value::Number(b)) = (val1, val2) {
            (a, b)
        } else {
            println!("binary_op: val1 or val2 aren't numbers");
            self.push(val1);
            return;
        };

        let result = match op {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            '/' => a / b,
            _ => {
                println!("binary_op: invalid op {}", op);
                0.0
            }
        };

        self.push(Value::Number(result))
    }
}
