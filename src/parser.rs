#[allow(unused_imports)]
use crate::{
    ast::{Expr, Stmt, AST},
    lexer::Token,
};

#[derive(Debug)]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

pub struct Parser {
    pub tokens: Vec<(Token, Position)>,
}

impl Parser {
    pub fn run(&self) {}
}
