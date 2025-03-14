use std::{f64::consts::PI, path::Path};

use crate::stringtokeniser::StrTokeniser;

use super::*;
use lexer::{Keyword, Lexer, Token};

#[test]
fn lexer_input_test_a() {
    let lexer = Lexer::new_str("add 1 2 x");

    assert!(lexer.eq([
        Token::Keyword(Keyword::Add),
        Token::FloatLiteral(1.0),
        Token::FloatLiteral(2.0),
        Token::Unidentified("x".to_string())
    ]));
}

#[test]
fn lexer_input_test_b() {
    let lexer = Lexer::new_str("add 1 2 x \n sub 1.0 3.14 x \n foo");

    println!("{:?}", lexer.clone().collect::<Vec<_>>());
    assert!(lexer.eq([
        Token::Keyword(Keyword::Add),
        Token::FloatLiteral(1.0),
        Token::FloatLiteral(2.0),
        Token::Unidentified("x".to_string()),
        Token::Keyword(Keyword::Sub),
        Token::FloatLiteral(1.0),
        Token::FloatLiteral(3.14),
        Token::Unidentified("x".to_string()),
        Token::Unidentified("foo".to_string())
    ]));
}

#[test]
fn fileio_input_test_a() {
    let mut unit = fileio::InterpereterUnit::new();

    unit.open_file(Path::new("./test-files/a.txt"))
        .expect("IO error");
    assert_eq!(unit.contents, "hello world");
}

#[test]
fn fileio_io_input_test_b() {
    let mut unit = fileio::InterpereterUnit::new();

    unit.open_file(Path::new("./test-files/b.txt"))
        .expect("IO error");
    assert!(unit.contents == "hello\n world " || unit.contents == "hello\r\n world ");
}

#[test]
fn fileio_interface_test_copycontents() {
    let unit = fileio::InterpereterUnit::new();
    let str = unit.contents;

    assert_eq!(str, "");
}

#[test]
fn fileio_interface_test_mutcontents() {
    let mut unit = fileio::InterpereterUnit::new();
    let content = &mut unit.contents;

    assert_eq!(content, "");

    *content = "hello world".to_string();
    assert_eq!(content, "hello world");
}

#[test]
fn fileio_interface_test_getcontents() {
    let unit = fileio::InterpereterUnit::new();
    let str = unit.contents;

    assert_eq!(str, "");
}

#[test]
fn itertok_test1() {
    use stringtokeniser::StrToken;

    let str = "add 1 2\ndaa 2 1".to_string();
    let tmp = StrTokeniser::new(&str);
    println!("{:?}", tmp.clone().collect());
    assert_eq!(tmp.collect(),vec![
        StrToken::Generic("add".to_string()),
        StrToken::Space,
        StrToken::Generic("1".to_string()),
        StrToken::Space,
        StrToken::Generic("2".to_string()),
        StrToken::EOL,
        StrToken::Generic("daa".to_string()),
        StrToken::Space,
        StrToken::Generic("2".to_string()),
        StrToken::Space,
        StrToken::Generic("1".to_string()),
        StrToken::EOF

    ])
}
