use crate::stringtokeniser::{split_eol, StrToken, tokenise_str};

#[test]
fn itertok_test1() {
    let str = "add 1 2\ndaa 2 1";
    let tmp = tokenise_str(str);
    println!("{:?}", tmp);
    assert_eq!(tmp,vec![
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

#[test]
fn spliteol_test() {
    let tmp = split_eol(&[
        StrToken::Generic("add".to_string()),
        StrToken::Generic("1".to_string()),
        StrToken::Generic("2".to_string()),
        StrToken::EOL,
        StrToken::Generic("sub".to_string()),
        StrToken::Generic("2".to_string()),
        StrToken::Generic("1".to_string()),
        StrToken::EOL,
        StrToken::EOF
    ]);
    println!("{:?}",tmp);
    assert_eq!(tmp,vec![
        vec![
            StrToken::Generic("add".to_string()),
            StrToken::Generic("1".to_string()),
            StrToken::Generic("2".to_string()),
        ],
        vec![
            StrToken::Generic("sub".to_string()),
            StrToken::Generic("2".to_string()),
            StrToken::Generic("1".to_string())
        ],
        vec![
            StrToken::EOF
        ]
    ])
}