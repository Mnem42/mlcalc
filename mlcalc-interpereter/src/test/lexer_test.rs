use crate::{lexer::clean_tokenarr, stringtokeniser::{tokenise_str, StrToken}};

use crate::lexer::{Keyword, Lexer, Token};

#[test]
fn lexer_input_test_a() {
    let arr = [
        StrToken::Generic("add".to_string()),
        StrToken::Generic("1".to_string()),
        StrToken::Generic("2".to_string()),
        StrToken::Generic("x".to_string())
    ];
    let lexer = Lexer::new_tokenarr(arr.as_slice());

    assert!(lexer.eq([
        Token::Keyword(Keyword::Add),
        Token::FloatLiteral(1.0),
        Token::FloatLiteral(2.0),
        Token::Unidentified("x".to_string())
    ]));
}

#[test]
fn lexer_input_test_b() {
    let tokens = tokenise_str("add 1 2 x\nsub 1 3.14 x\nfoo");
    let lexer = Lexer::new(tokens.iter());
    let tmp = clean_tokenarr(lexer.collect::<Vec<_>>().as_slice());

    assert_eq!(tmp, vec![
        Token::Keyword(Keyword::Add),
        Token::FloatLiteral(1.0),
        Token::FloatLiteral(2.0),
        Token::Unidentified("x".to_string()),
        Token::EOL,
        Token::Keyword(Keyword::Sub),
        Token::FloatLiteral(1.0),
        Token::FloatLiteral(3.14),
        Token::Unidentified("x".to_string()),
        Token::EOL,
        Token::Unidentified("foo".to_string()),
        Token::EOF
    ]);
}

#[test]
fn lexer_input_test_comment(){
    let tokens = tokenise_str("add 1 2 x # A comment \n# This one spans the entire line \n sub");
    let lexer = Lexer::new(tokens.iter());
    let tmp = clean_tokenarr(lexer.collect::<Vec<_>>().as_slice());

    assert_eq!(tmp,vec![
        Token::Keyword(Keyword::Add),
        Token::FloatLiteral(1.0),
        Token::FloatLiteral(2.0),
        Token::Unidentified("x".to_string()),
        Token::Comment("A comment".to_string()),
        Token::Comment("This one spans the entire line".to_string()),
        Token::Keyword(Keyword::Sub),
        Token::EOF
    ]);
}