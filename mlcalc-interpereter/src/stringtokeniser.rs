use std::str::Chars;
use std::string::String;

#[derive(Debug, PartialEq, Clone)]
pub enum StrToken {
    Generic(String),
    Space,
    EOL,
    EOF,
}

pub fn split_eol(x:&[StrToken]) -> Vec<Vec<StrToken>>{
    x.split(|x| *x==StrToken::EOL).map(|x|x.to_owned()).collect()
}

#[derive(Clone)]
pub struct StrTokeniser<'a> {
    contained: Chars<'a>,
    position: usize,
}

impl<'a> StrTokeniser<'a> {
    pub fn new(input: &'a str) -> StrTokeniser<'a> {
        let tmp = input.chars();
        StrTokeniser {
            contained: tmp,
            position: 0,
        }
    }

    pub fn collect(self) -> Vec<StrToken> {
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
                let size = size.1.unwrap_or(0);
                if i > size {
                    out.push(StrToken::Generic(tmpstr.clone()));
                }
                tmpstr.push(x);
            }
        });

        out.push(StrToken::Generic(tmpstr.clone()));
        out.push(StrToken::EOF);

        out
    }
}
