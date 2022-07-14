mod chunk;
mod opcode;
mod parser;
mod scanner;
mod token;
mod value;
mod vm;

use chunk::Chunk;
use opcode::Opcode;
use parser::Parser;
use scanner::Scanner;
use value::Value;
use vm::Vm;

use std::{
    env, fs,
    io::{stdin, stdout, Write},
};

fn repl() {
    loop {
        print!("> ");
        stdout().flush().ok();

        let mut line = String::new();
        stdin().read_line(&mut line).ok();

        if line == "exit\n" {
            break;
        }

        let res = Parser::new(line).compile();
        match res {
            Ok(chunk) => {
                Vm::new(chunk).run();
                ()
            }
            Err(_) => println!("Compile error"),
        }
    }
}

fn run_file(fname: &str) {
    println!("Running file {}", fname);
    let _source =
        fs::read_to_string(fname).unwrap_or_else(|_| panic!("Unable to open file {}", fname));

    // @TODO
    // let res = interpret(&_source);
}

fn main() {
    let mut chunk = Chunk::new();
    let mut constant = chunk.add_constant(Value::Number(1.2));
    chunk.write(Opcode::Constant as u8, 123);
    chunk.write(constant as u8, 123);

    constant = chunk.add_constant(Value::Number(3.4));
    chunk.write(Opcode::Constant as u8, 123);
    chunk.write(constant as u8, 123);
    chunk.write(Opcode::Add as u8, 123);

    constant = chunk.add_constant(Value::Number(5.6));
    chunk.write(Opcode::Constant as u8, 123);
    chunk.write(constant as u8, 123);
    chunk.write(Opcode::Divide as u8, 123);

    chunk.write(Opcode::Negate as u8, 123);
    chunk.write(Opcode::Return as u8, 123);

    chunk.disassemble("test chunk");

    let mut vm = Vm::new(chunk);
    let _ = vm.run();

    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => repl(),
        2 => run_file(&args[1]),
        _ => println!("Usage: emerald [path]"),
    }
}
