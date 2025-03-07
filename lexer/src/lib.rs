use std::str::SplitWhitespace;
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
    data: SplitWhitespace<'a>,
    position: usize
}

impl Lexer<'_>{
    pub fn new(input: &str) -> Lexer{
        Lexer{
            data:input.split_whitespace(),
            position:0
        }
    }
}

impl Iterator for Lexer<'_>{
    type Item = Token;

    fn next(&mut self) -> Option<Token>{
        let slice = self.data.next();

        match slice{
            Some(x) =>{
				self.position+=x.chars().count();

				let float_opt = match x.parse::<f64>() {
					Ok(x)  => Some(Token::FloatLiteral(x)),
					Err(_) => None
				};

				if float_opt != None {
					return float_opt;
				}

				match x{
					"add" => Some(Token::Keyword(Keyword::Add)),
					"sub" => Some(Token::Keyword(Keyword::Sub)),
					_     => Some(Token::Unidentified(x.to_string()))
				}
            }
            None=>None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_input_test_a(){
        let mut lexer = Lexer::new("add 1 2 x");

        assert!(lexer.eq([Token::Keyword(Keyword::Add),Token::FloatLiteral(1.0),
            Token::FloatLiteral(2.0),Token::Unidentified("x".to_string())]));   
    }

    #[test]
    fn lexer_input_test_b(){
        let mut lexer = Lexer::new("add 1 2 x \n sub 1 3 x \n foo");

        assert!(lexer.eq([
            Token::Keyword(Keyword::Add),
            Token::FloatLiteral(1.0),
            Token::FloatLiteral(2.0),
            Token::Unidentified("x".to_string()),
            Token::Keyword(Keyword::Sub),
            Token::FloatLiteral(1.0),
            Token::FloatLiteral(3.0),
            Token::Unidentified("x".to_string()),
            Token::Unidentified("foo".to_string())
        ]));
    }
}
