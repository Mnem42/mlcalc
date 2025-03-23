use crate::stringtokeniser::StrToken;
use crate::fileio::FilePos;
use crate::lexer::{Keyword, Token};
pub type MergedLine = Vec<StrToken>;

#[derive(Debug,PartialEq)]
pub enum ResolvedToken {
    Add(f64, f64, Var),
    Sub(f64, f64, Var),
    Mul(f64, f64, Var),
    Div(f64, f64, Var),
    Set(f64, Var),
    EOL,
    EOF
}

#[derive(Debug,PartialEq)]
pub struct Var{
    name: String
}

impl Var{
    pub fn new(name: String) -> Var{
        Var{
            name
        }
    }
}

#[derive(Debug,PartialEq)]
pub enum Error {
    UnexpectedToken()
}

pub fn resolve_tokens(lines: &[FilePos<&[Token]>]) -> Vec<Result<ResolvedToken,Error>>{
    let lines = lines.iter();

    lines.map(|x|{
        match x.clone().get_contents() {
            [Token::Keyword(keyword),
             Token::FloatLiteral(x),
             Token::FloatLiteral(y),
             Token::Unidentified(var)] => {
                match keyword {
                    Keyword::Add => Ok(ResolvedToken::Add(*x, *y, Var{name:var.clone()})),
                    Keyword::Sub => Ok(ResolvedToken::Sub(*x, *y, Var{name:var.clone()})),
                    Keyword::Mul => Ok(ResolvedToken::Mul(*x, *y, Var{name:var.clone()})),
                    Keyword::Div => Ok(ResolvedToken::Div(*x, *y, Var{name:var.clone()})),
                    _ => todo!("Implement keyword {:?}", keyword)
                }
            },
            [Token::Keyword(keyword),
             Token::FloatLiteral(x),
             Token::Unidentified(var)] => {
                match keyword {
                    Keyword::Set => Ok(ResolvedToken::Set(*x, Var{name:var.clone()})),
                    _ => todo!("Implement keyword {:?}", keyword)
                }
            }
            [Token::EOF] => Ok(ResolvedToken::EOF),
            _  => Err(Error::UnexpectedToken())
        }
    }).collect()
}