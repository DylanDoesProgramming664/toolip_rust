use std::{env, fs, process::exit};

mod ast;
mod evaluator;
mod lexer;
mod parser;
mod repl;
mod token;

#[quit::main]
fn main() {
    let mut args = env::args();
    match args.len() {
        1 => {
            repl::start_repl();
        }
        2 => {
            let filename = args.nth(1).unwrap();
            let contents = fs::read_to_string(filename)
                .unwrap()
                .chars()
                .collect::<Vec<char>>();
            let mut lexer = lexer::Lexer::new(contents);
            lexer.print_tokens();
        }
        _ => {
            println!("Too many arguments. Shutting down.");
            exit(1);
        }
    }
}
