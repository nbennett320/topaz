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
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => repl(),
        2 => run_file(&args[1]),
        _ => println!("Usage: emerald [path]"),
    }
}
