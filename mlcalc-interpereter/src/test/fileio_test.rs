use std::path::Path;
use crate::fileio::InterpereterUnit;

#[test]
fn fileio_input_test_a() {
    let mut unit = InterpereterUnit::new();

    unit.open_file(Path::new("./test-files/a.txt"))
        .expect("IO error");
    assert_eq!(unit.contents, "hello world");
}

#[test]
fn fileio_io_input_test_b() {
    let mut unit = InterpereterUnit::new();

    unit.open_file(Path::new("./test-files/b.txt"))
        .expect("IO error");
    assert!(unit.contents == "hello\n world " || unit.contents == "hello\r\n world ");
}

#[test]
fn fileio_interface_test_copycontents() {
    let unit = InterpereterUnit::new();
    let str = unit.contents;

    assert_eq!(str, "");
}

#[test]
fn fileio_interface_test_mutcontents() {
    let mut unit = InterpereterUnit::new();
    let content = &mut unit.contents;

    assert_eq!(content, "");

    *content = "hello world".to_string();
    assert_eq!(content, "hello world");
}

#[test]
fn fileio_interface_test_getcontents() {
    let unit = InterpereterUnit::new();
    let str = unit.contents;

    assert_eq!(str, "");
}