
#[derive(Debug, Clone, PartialEq, Eq)]
enum Keyword{
    Add,
    Sub
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token{
    Identifier,
    Space,
    Comment,
    Keyword(Keyword),
    FloatConstant(f64),
    Punctuator(char),
    EOF
}

pub fn parse(input: &str) -> Vec<Option<Token>>{
    input.split_whitespace().map(|x| {
            if x.parse::<f64>().is_ok() {
                Some(Token::FloatConstant(x.parse::<f64>().expect("???")))
            }
            else{
                match x{
                    "add" => Some(Token::Keyword(Keyword::Add)),
                    "sub" => Some(Token::Keyword(Keyword::Sub)),
                    x     => None
                }}
            }
        ).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_test() {
        //Move along, nothing to see here...

        println!("{:?}",parse("add 1 2  \n sub 3 2 \n erap "));
        assert_eq!(parse("add 1 2  \n sub 3 2 \n erap "), 
        vec![Some(Token::Keyword(Keyword::Add)),Some(Token::FloatConstant(1.0)),Some(Token::FloatConstant(2.0))
        ,Some(Token::Keyword(Keyword::Sub)),Some(Token::FloatConstant(3.0)),Some(Token::FloatConstant(2.0)),None]);
    }
}
