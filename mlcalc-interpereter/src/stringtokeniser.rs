use std::str::Chars;
use std::string::String;

#[derive(Debug, PartialEq, Clone)]
pub enum StrToken {
    Generic(String),
    Space,
    EOL,
    EOF,
}

#[derive(Clone)]
pub struct StrTokeniser<'a> {
    contained: Chars<'a>,
    position: usize,
}

impl<'a> StrTokeniser<'a> {
    pub fn new(input: &'a String) -> StrTokeniser<'a> {
        let tmp = input.chars();
        StrTokeniser {
            contained: tmp,
            position: 0,
        }
    }

    pub fn collect(mut self) -> Vec<StrToken> {
        let mut out: Vec<StrToken> = vec![];
        let mut tmpstr = "".to_string();
        let size = self.contained.size_hint();

        self.contained.enumerate().for_each(|(i, x)| match x {
            ' ' => {
                out.push(StrToken::Generic(tmpstr.clone()));
                out.push(StrToken::Space);
                
                tmpstr.clear();
            }
            '\n' => {
                out.push(StrToken::Generic(tmpstr.clone()));
                out.push(StrToken::EOL);

                tmpstr.clear();
            }
            x => {
                let size = match size.1 {
                    Some(x) => x,
                    None => 0,
                };
                if i > size - 4 {
                    out.push(StrToken::Generic(tmpstr.clone()));
                }
                tmpstr.push(x);
            }
        });

        out.push(StrToken::Generic(tmpstr.clone()));
        out.push(StrToken::EOF);

        // Remove empty tokens. There's probably a way to avoid them, but idk what it is
        out.retain(|x| *x != StrToken::Generic("".to_string()));

        out
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
