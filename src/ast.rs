#![allow(unused_imports, dead_code)]
use crate::token::{self, Token};

pub trait Node {
    fn token_value(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn token_value(&self) -> String {
        if !self.statements.is_empty() {
            return self.statements[0].token_value();
        }
        "".to_owned()
    }
}

pub struct Identifier {
    token: Token,
    value: String,
}

impl Identifier {
    pub fn expression_node(&self) {}

    pub fn token_value(&self) -> String {
        self.token.value.clone()
    }
}

pub struct Int32Statement {
    token: Token,
    name: Identifier,
    value: Box<dyn Expression>,
}

pub struct Float32Statement {
    token: Token,
    name: Identifier,
    value: Box<dyn Expression>,
}
