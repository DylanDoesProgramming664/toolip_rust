#![allow(dead_code, unused_imports)]
use crate::lexer::{self, Lexer};
use crate::token;
use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};

pub fn start() {
    println!("Welcome to the Toolip Programming Language! Enter some code below and hit Enter to execute.");
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::new(
        DefaultPromptSegment::Basic("toolip".to_owned()),
        DefaultPromptSegment::Empty,
    );

    loop {
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) => {
                let line = buffer.chars().collect::<Vec<char>>();
                let mut lex = Lexer::new(line);
                lex.print_tokens();
            }
            Ok(Signal::CtrlD | Signal::CtrlC) => {
                println!("\nAborted!");
                break;
            }
            x => {
                println!("Event: {x:?}");
            }
        }
    }
}
