mod chunk;
mod function;
mod opcode;
mod operator;
mod parse_rule;
mod parser;
mod precedence;
mod scanner;
mod token;
mod value;
mod vm;

use parser::Parser;
use scanner::Scanner;
use vm::Vm;

use std::{
    env, fs,
    io::{stdin, stdout, Write},
};

fn repl() {
    let mut vm = Vm::new();
    let mut line_num = 1;
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
            Ok(func) => {
                func.chunk
                    .disassemble(format!("repl line {}", line_num).as_str());
                let res = vm.run(func);
                match res {
                    Ok(value) => println!("{}", value),
                    _ => todo!("Handle runtime error"),
                }
            }
            Err(_) => println!("Compile error"),
        }

        line_num += 1;
    }
}

fn run_file(fname: &str) {
    let source =
        fs::read_to_string(fname).unwrap_or_else(|_| panic!("Unable to open file {}", fname));

    let mut vm = Vm::new();

    let res = Parser::new(source).compile();
    match res {
        Ok(func) => {
            func.chunk.disassemble(format!("script {}", fname).as_str());
            let _ = vm.run(func);
        }
        Err(_) => println!("Compile error"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => repl(),
        2 => run_file(&args[1]),
        _ => println!("Usage: topaz [path]"),
    }
}
