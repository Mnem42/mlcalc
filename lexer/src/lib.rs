
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword{
    Add,
    Sub
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token{
    Identifier,
    Space,
    Comment,
    Keyword(Keyword),
    FloatLiteral(f64),
    Punctuator(char),
    Unidentified(String),
    EOF
}

#[derive(Debug)]
struct Lexer<'a>{
    data: Vec<&'a str>,
    position: usize
}

impl Lexer<'_>{
    pub fn new(input: &str) -> Lexer{
        Lexer{
            data:input.split_whitespace().collect(),
            position:0
        }
    }
}

impl Iterator for Lexer<'_>{
    type Item = Token;

    fn next(&mut self) -> Option<Token>{
        self.position+=1;

        if self.position>self.data.len() {
            return None;
        }

        let data=self.data[self.position-1];

        if data.parse::<f64>().is_ok() {
            Some(Token::FloatLiteral(data.parse::<f64>().expect("???")))
        }
        else{
            match data{
                "add" => Some(Token::Keyword(Keyword::Add)),
                "sub" => Some(Token::Keyword(Keyword::Sub)),
                x     => Some(Token::Unidentified(data.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_test(){
        let lexer = Lexer::new("add 1 2 x");
        println!("{:?}",lexer);
        assert!(lexer.eq([Token::Keyword(Keyword::Add),Token::FloatLiteral(1.0),
            Token::FloatLiteral(2.0),Token::Unidentified("x".to_string())]));   

        /*assert_eq!(lexer.next(),Some(Token::Keyword(Keyword::Add)));
        assert_eq!(lexer.next(),Some(Token::FloatLiteral(1.0)));
        assert_eq!(lexer.next(),Some(Token::FloatLiteral(2.0)));
        assert_eq!(lexer.next(),None);*/
    }

    #[test]
    fn lexer_test_map(){
        let mut lexer = Lexer::new("add 1 2 x \n sub 1 3 x");
        lexer.for_each(|x| println!("{:?}",x));
    }
}
