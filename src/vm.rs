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
                }
                print!(" ]");
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
}
