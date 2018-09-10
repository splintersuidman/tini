use std::env;
use std::fs;
use std::process;
use tini::prelude::*;

fn main() {
    let mut args = env::args().skip(1);
    let filename = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Error: too few arguments. Expected one.");
            println!("Usage: tinii <file>");
            process::exit(1);
        }
    };

    let input = match fs::read_to_string(filename) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("error while reading file {}.", e);
            process::exit(1)
        }
    };

    let lexer = Lexer::new(&input);
    let parser = Parser::new(lexer);
    let mut interpreter = Interpreter::new();

    for expr in parser {
        let expr = match expr {
            Ok(expr) => expr,
            Err(e) => {
                eprintln!("error while parsing: {}.", e);
                process::exit(1);
            }
        };
        match interpreter.eval(expr) {
            Err(e) => {
                eprintln!("error while running interpreter: {}.", e);
                process::exit(1)
            }
            _ => {}
        }
    }
}
