mod chunk;
mod opcode;
mod scanner;
mod token;
mod value;
use chunk::Chunk;
use opcode::Opcode;
use value::Value;

use scanner::Scanner;

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

        let mut s = Scanner::new(line);
        let _ = s.scan_all();

        // @TODO
        // let res = interpret(&line);
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
    let constant = chunk.add_constant(Value::Number(1.2));
    chunk.write(Opcode::Constant as u8, 123);
    chunk.write(constant as u8, 123);
    chunk.write(Opcode::Return as u8, 123);

    chunk.disassemble("test chunk");

    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => repl(),
        2 => run_file(&args[1]),
        _ => println!("Usage: emerald [path]"),
    }
}
