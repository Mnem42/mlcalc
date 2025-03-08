use std::str::SplitWhitespace;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword{
    Add,
    Sub
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token{
    Identifier,
    Comment,
    Keyword(Keyword),
    FloatLiteral(f64),
    Punctuator(char),
    Unidentified(String)
}

/*struct Token {
    pos: file::FilePos,
    item: TokenItem
}*/

#[derive(Debug)]
pub struct Lexer<'a>{
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

    pub fn new(input: &InterpereterUnit) -> Lexer{
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
