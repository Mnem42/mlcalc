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
    Comment(String),
    Keyword(Keyword),
    FloatLiteral(f64),
    Punctuator(char),
    Unidentified(String),
    Space,
    EOL,
    EOF,
    Empty
}

#[derive(Debug, Clone)]
/// A lexer struct. This is implemented as an `Iterator` of tokens.
pub struct Lexer<'a> {
    contained: std::slice::Iter<'a, StrToken>,
    comment_str: String,
    in_comment: bool
}

impl <'a>Lexer<'a> {
    pub fn new_tokenarr(input: &[StrToken]) -> Lexer {
        Lexer {
            contained: input.iter(),
            in_comment: false,
            comment_str: String::new()
        }
    }

    pub fn new(input: std::slice::Iter<'a, StrToken>) -> Lexer<'a> {
        Lexer {
            contained: input,
            in_comment: false,
            comment_str: String::new()
        }
    }
}

// Implements iterator so that it behaves like one
impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        match self.contained.next()? {
            StrToken::Generic(x) =>{
                if *x == "".to_string(){
                    return Some(Token::Empty)
                }

                if self.in_comment {
                    self.comment_str.push_str(x);
                    Some(Token::Empty)
                }
                else if let Ok(float_opt) = x.parse::<f64>() {
                    Some(Token::FloatLiteral(float_opt))
                }
                else{
                    match x.as_str() {
                        "add" => Some(Token::Keyword(Keyword::Add)),
                        "sub" => Some(Token::Keyword(Keyword::Sub)),
                        "mul" => Some(Token::Keyword(Keyword::Mul)),
                        "div" => Some(Token::Keyword(Keyword::Div)),
                        "set" => Some(Token::Keyword(Keyword::Set)),
                        "#"   => {self.in_comment=true;Some(Token::Empty)},
                        _ => Some(Token::Unidentified(x.to_string())),
                    }
                }
            }
            StrToken::Space => {
                if self.in_comment {
                    self.comment_str.push(' ');
                    Some(Token::Empty)
                }
                else{
                    Some(Token::Space)
                }
            },
            StrToken::EOL => {
                if !self.comment_str.is_empty(){
                    let tmp = Some(Token::Comment(self.comment_str.clone().trim().to_string()));
                    self.comment_str.clear();
                    self.in_comment=false;

                    tmp
                }
                else{
                    Some(Token::EOL)
                }
            },
            StrToken::EOF => Some(Token::EOF)
        }
    }
}

/// Removes `Token::Empty` and `Token::Space`
pub fn clean_tokenarr(x: &[Token]) -> Vec<Token> {
    x.iter().clone()
     .filter(|x| **x!=Token::Empty && **x!=Token::Space)
     .map(|x|x.clone()).collect()
}