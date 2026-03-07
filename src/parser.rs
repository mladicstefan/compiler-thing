#[allow(unused_imports)]
use crate::{
    ast::{Expr, Stmt, AST},
    lexer::Token,
};

#[derive(Clone, Debug)]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

type Lexme = (Token, Position);

#[derive(Clone, Debug)]
pub struct Parser {
    pub tokens: Vec<Lexme>,
    pos: usize,
}

#[allow(dead_code)]
impl Parser {
    pub fn new(tokens: Vec<Lexme>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Lexme> {
        self.tokens.get(self.pos)
    }
    fn consume(&mut self) -> Option<Lexme> {
        let tok = self.tokens.get(self.pos);
        self.pos += 1;
        tok.cloned()
    }
    // pub fn run(&mut self) -> Result<AST, Box<dyn std::error::Error>> {}
}
