#![allow(unused_imports, dead_code)]
use crate::ast;
use crate::lexer::{self, Lexer};
use crate::token::{self, Token};

pub struct Parser {
    input: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { input: tokens }
    }
}
