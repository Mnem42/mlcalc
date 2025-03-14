use crate::fileio;
use std::str::SplitWhitespace;

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
}
/*struct Token {
    pos: file::FilePos,
    item: TokenItem
}*/

#[derive(Debug)]
/// A lexer struct. This is implemented as an `Iterator` of tokens.
pub struct Lexer<'a> {
    data: SplitWhitespace<'a>,
    position: usize,
}

impl Lexer<'_> {
    pub fn new_str(input: &str) -> Lexer {
        Lexer {
            data: input.split_whitespace(),
            position: 0,
        }
    }

    pub fn new(input: &fileio::InterpereterUnit) -> Lexer {
        Lexer {
            data: input.str_tokenise(),
            position: 0,
        }
    }
}

// Implements iterator so that it behaves like one
impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let x = self.data.next()?;

        self.position += x.chars().count();

        if let Ok(float_opt) = x.parse() {
            return Some(Token::FloatLiteral(float_opt));
        }

        match x {
            "add" => Some(Token::Keyword(Keyword::Add)),
            "sub" => Some(Token::Keyword(Keyword::Sub)),
            "mul" => Some(Token::Keyword(Keyword::Mul)),
            "div" => Some(Token::Keyword(Keyword::Div)),
            "set" => Some(Token::Keyword(Keyword::Set)),
            _ => Some(Token::Unidentified(x.to_string())),
        }
    }
}
