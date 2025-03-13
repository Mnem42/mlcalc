use std::path::Iter;

use crate::lexer::{Token, Lexer};

#[derive(Debug, Clone, PartialEq)]
pub enum ResolvedToken{
    Add(f64,f64),
    Sub(f64,f64),
    Mul(f64,f64),
    Div(f64,f64),
    Var(f64)
}

struct Resolver{
    contents: Vec<ResolvedToken>,
    position: usize
}

impl Resolver{
    pub fn new<'a>(input_vec: Vec<ResolvedToken>) -> Resolver{
        Resolver{
            contents:input_vec,
            position:0
        }
    }
}

impl Iterator for Resolver{
    type Item = ResolvedToken;

    fn next(&mut self) -> Option<ResolvedToken>{
        if self.contents.len() < self.position {
            Some(self.contents[self.position].clone())
        } else { None }
    }
}