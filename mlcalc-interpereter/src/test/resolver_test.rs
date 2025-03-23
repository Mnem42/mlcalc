use crate::{fileio::FilePos, lexer::{Keyword, Token}, resolver::{self, ResolvedToken, Var}};

#[test]
fn resolver_test_a(){
    let tmp =resolver::resolve_tokens(&[
        FilePos::new(&[
            Token::Keyword(Keyword::Add),
            Token::FloatLiteral(1.0),
            Token::FloatLiteral(1.0),
            Token::Unidentified("foo".to_string())
        ],1),
        FilePos::new(&[
            Token::Keyword(Keyword::Set),
            Token::FloatLiteral(1.0),
            Token::Unidentified("foo".to_string())
        ],4),
        FilePos::new(&[
            Token::EOF
        ],6)
    ]);
    assert_eq!(tmp,vec![
        Ok(ResolvedToken::Add(1.0,1.0,Var::new("foo".to_string()))),
        Ok(ResolvedToken::Set(1.0,Var::new("foo".to_string()))),
        Ok(ResolvedToken::EOF)
    ])
}
