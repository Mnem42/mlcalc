use std::string::{String};
use std::str::Chars;

#[derive(Debug)]
enum CharToken{
    Generic(char),
    EOL,
    EOF
}

#[derive(Debug, Copy)]
pub enum StrToken{
    Generic(String),
    EOL,
    EOF
}

struct CharTokeniser<'a>{
    contained: Chars<'a>,
    position: usize
}

pub struct StrTokeniser{
    contained: Vec<CharToken>,
    position: usize
}

fn strcollect<'a>(input: &[StrToken]) -> Vec<StrToken>{
    let tmp: Vec<StrToken> = input.to_vec();

    tmp.iter().enumerate().map(|i| {
        match i {
            (index,StrToken::Generic(x)) => {
                let tmp = &tmp[..index];

                let pos = tmp.iter().position(|x| {
                    match x {
                        StrToken::Generic(_) => false,
                        _ => true
                    }
                });

                let pos: usize = match pos{
                    Some(x) => x, 
                    _ => input.len()
                };

                let mut str = String::new();
                &tmp[index..pos].iter().map(|x| match x {StrToken::Generic(x) => x.clone(), _ => "".to_string()})
                    .for_each(|x| str.push_str(&x));

                tmp
            },
            _ => &[StrToken::EOF]
        }
    }).map(|x| x[0]).collect()
}

impl <'a>Iterator for CharTokeniser<'a>{
    type Item = CharToken;

    fn next(&mut self) -> Option<CharToken> {
        self.position += 1;

        match self.contained.next(){
            Some('\n') => Some(CharToken::EOL),
            None => None,
            x => Some(CharToken::Generic(x?))
        }
    }
}

impl StrTokeniser{
    pub fn new(input: String) -> StrTokeniser{
        let chartok = CharTokeniser{contained: input.chars(),position:0}.collect::<Vec<_>>();
        StrTokeniser{
            contained: chartok, position:0
        }
    }
}

/*
impl Iterator for StrTokeniser{
    type Item = StrToken;

    fn next(&mut self)  -> Option<StrToken> {
        let mut iter= self.contained.iter();
        println!("aa");
        match iter.next(){
            Some(CharToken::EOL) => Some(StrToken::EOL),
            Some(CharToken::Generic(_)) => Some(StrToken::Generic(strcollect(iter.collect()))),
            None => Some(StrToken::EOF),
            _ => todo!("Something isn't implemented")
        }
    }
}
    */