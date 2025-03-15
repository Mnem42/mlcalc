use crate::fileio;
use crate::stringtokeniser;
use crate::stringtokeniser::StrToken;
use crate::stringtokeniser::StrTokeniser;
use std::collections::binary_heap::Iter;
use std::str::SplitWhitespace;
use std::slice;

#[derive(Debug, Clone, PartialEq, Eq)]

/// Enumeration for keywords.
pub enum Keyword {
    Add,
    Sub,
    Mul,
    Div,
    Set,
}

/// Enumeration for lexer tokens.

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier,
    Comment,
    Keyword(Keyword),
    FloatLiteral(f64),
    Punctuator(char),
    Unidentified(String),
    Space,
    EOL,
    EOF
}

#[derive(Debug, Clone)]
/// A lexer struct. This is implemented as an `Iterator` of tokens.
pub struct Lexer<'a> {
    contained: std::slice::Iter<'a, StrToken>,
    position: usize,
}

impl <'a>Lexer<'a> {
    pub fn new_tokenarr(input: &[StrToken]) -> Lexer {
        Lexer {
            contained: input.iter(),
            position: 0,
        }
    }

    pub fn new(input: std::slice::Iter<'a, StrToken>) -> Lexer<'a> {
        Lexer {
            contained: input,
            position: 0,
        }
    }
}

// Implements iterator so that it behaves like one
impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        match self.contained.next()? {
            StrToken::Generic(x) =>{
                if let Ok(float_opt) = x.parse::<f64>() {
                    Some(Token::FloatLiteral(float_opt))
                }
                else{
                    match x.as_str() {
                        "add" => Some(Token::Keyword(Keyword::Add)),
                        "sub" => Some(Token::Keyword(Keyword::Sub)),
                        "mul" => Some(Token::Keyword(Keyword::Mul)),
                        "div" => Some(Token::Keyword(Keyword::Div)),
                        "set" => Some(Token::Keyword(Keyword::Set)),
                        _ => Some(Token::Unidentified(x.to_string())),
                    }
                }
            }
            StrToken::Space => Some(Token::Space),
            StrToken::EOL   => Some(Token::EOL),
            StrToken::EOF   => Some(Token::EOF)
        }
    }
}
