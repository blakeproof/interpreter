extern crate interpreter;
extern crate rustyline;


use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::cell::RefCell;


use interpreter::lexer::Lexer;
fn main() {
    let mut rl = Editor::<()>::new();

    println!("hello welcome");

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                rl.add_history_entry(&line);

                let mut parser = Parser::new(Lexer::new(&line));
            }
            Err(ReadlineError::Interrupted) => {
                println!("\nBye :");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err)
            }
        }
    }
}