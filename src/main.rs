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
            repl::start();
        }
        2 => {
            let filename = args.nth(1).map_or_else(
                || {
                    println!("Could not read file.");
                    exit(1);
                },
                |filename| filename,
            );
            if &filename[filename.len() - 5..] != ".tool" {
                println!("Not a Toolip file.");
                exit(1);
            }
            let contents = fs::read_to_string(&filename).map_or_else(
                |_| {
                    println!("Error reading file: {}", &filename);
                    exit(1);
                },
                |contents| contents.chars().collect::<Vec<char>>(),
            );
            let mut lexer = lexer::Lexer::new(contents);
            lexer.print_tokens();
        }
        _ => {
            println!("Too many arguments. Shutting down.");
            exit(1);
        }
    }
}
