use std::io::{self, BufRead, Write};
use tini::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut interpreter = Interpreter::new();

    loop {
        print!(">> ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        stdin
            .lock()
            .read_line(&mut input)
            .expect("Error while reading from stdin.");

        // ^D flushes the input, so no newline is present.
        if !input.contains('\n') {
            println!("\nLeaving ...");
            break;
        }

        let lexer = Lexer::new(&input);
        let parser = Parser::new(lexer);

        for expr in parser {
            match expr {
                Err(e) => {
                    eprintln!("Error: {}.", e);
                    break;
                }
                Ok(expr) => {
                    let value = match interpreter.eval(expr) {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("Error: {}.", e);
                            break;
                        }
                    };
                    println!(" < {:?}", value);
                }
            }
        }
    }
}
