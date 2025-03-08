use super::*;

#[test]
fn lexer_input_test_a(){
    let lexer = Lexer::new("add 1 2 x");

    assert!(lexer.eq([Token::Keyword(Keyword::Add),Token::FloatLiteral(1.0),
    Token::FloatLiteral(2.0),Token::Unidentified("x".to_string())]));   
}

#[test]
fn lexer_input_test_b(){
    let lexer = Lexer::new("add 1 2 x \n sub 1 3 x \n foo");

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