use crate::stringtokeniser::{StrToken,StrTokeniser};

#[test]
fn itertok_test1() {
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
