use crate::opcode::{from_u8, Opcode};
use crate::value::Value;

pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset]);
        }

        let instruction = self.code[offset];
        match from_u8(instruction) {
            Opcode::Return => self.simple_instruction("Return", offset),
            Opcode::Constant => self.constant_instruction("Constant", offset),
            Opcode::Negate => self.simple_instruction("Negate", offset),
            Opcode::Add => self.simple_instruction("Add", offset),
            Opcode::Subtract => self.simple_instruction("Subtract", offset),
            Opcode::Multiply => self.simple_instruction("Multiply", offset),
            Opcode::Divide => self.simple_instruction("Divide", offset),
            Opcode::Nil => self.simple_instruction("Nil", offset),
            Opcode::True => self.simple_instruction("True", offset),
            Opcode::False => self.simple_instruction("False", offset),
            Opcode::Not => self.simple_instruction("Not", offset),
            Opcode::Equal => self.simple_instruction("Equal", offset),
            Opcode::Greater => self.simple_instruction("Greater", offset),
            Opcode::Less => self.simple_instruction("Less", offset),
            _ => {
                println!("Unknown opcode: {}", instruction);
                offset + 1
            }
        }
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant = self.code[offset + 1] as usize;
        print!("{} {} ", name, constant);
        self.constants[constant].print();
        print!("\n");
        return offset + 2;
    }
}
