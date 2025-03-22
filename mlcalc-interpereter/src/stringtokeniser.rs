use std::iter;
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

pub fn tokenise_str(input:&str) -> Vec<StrToken> {
    let mut out: Vec<StrToken> = vec![];
    let iterator = input.chars();
    let mut tmpstr = "".to_string();
    let size = iterator.size_hint();

    iterator.enumerate().for_each(|(i, x)| match x {
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
