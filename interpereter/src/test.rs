use std::path::Path;

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
    assert_eq!(unit.get_contents(), "hello world");
}

#[test]
fn fileio_io_input_test_b() {
    let mut unit = fileio::InterpereterUnit::new();

    unit.open_file(Path::new("./test-files/b.txt"))
        .expect("IO error");
    assert!(unit.get_contents() == "hello\n world " || unit.get_contents() == "hello\r\n world ");
}

#[test]
fn fileio_interface_test_copycontents() {
    let mut unit = fileio::InterpereterUnit::new();
    let str = unit.get_contents_copy();

    assert_eq!(str, "");
}

#[test]
fn fileio_interface_test_mutcontents() {
    let mut unit = fileio::InterpereterUnit::new();
    let str = unit.get_contents_mut();

    assert_eq!(str, "");

    *str = "hello world".to_string();
    assert_eq!(str, "hello world");
}

#[test]
fn fileio_interface_test_getcontents() {
    let mut unit = fileio::InterpereterUnit::new();
    let str = unit.get_contents();

    assert_eq!(str, "");
}
