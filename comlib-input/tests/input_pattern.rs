use comlib_input::*;

#[test]
fn test_last_pattern() {
    assert_eq!(input_pattern!(usize, "-").parse_all("12-"), Some(12));
    assert_eq!(input_pattern!(usize, "-").parse_all("12- "), None);
    assert_eq!(input_pattern!(usize, "-").parse_all("12"), None);

    assert_eq!(input_pattern!(usize, "-"?).parse_all("12-"), Some(12));
    assert_eq!(input_pattern!(usize, "-"?).parse_all("12- "), None);
    assert_eq!(input_pattern!(usize, "-"?).parse_all("12"), Some(12));
}

#[test]
fn test_input_pattern() {
    assert_eq!(
        input_pattern!(usize, "-", usize, " ", char, ": ", String).parse_all("1-2 c: a b"),
        Some((1, 2, 'c', "a b".to_string()))
    );
    assert_eq!(
        input_pattern!(String, ' ', String, ' ', String).parse_all("word1 word2 word3"),
        Some((
            "word1".to_string(),
            "word2".to_string(),
            "word3".to_string()
        ))
    );

    assert_eq!(
        input_pattern!(String, ' ', usize, ' ', String).parse_all("word1 word2 word3"),
        None
    );

    assert_eq!(
        input_pattern!(String, ' ', usize, ' ', String).parse_all("word1 123word2 word3"),
        None
    );
    assert_eq!(
        input_pattern!(String, ' ', usize, String, ' ', String).parse_all("word1 123word2 word3"),
        Some((
            "word1".to_string(),
            123,
            "word2".to_string(),
            "word3".to_string()
        ))
    );

    assert_eq!(
        input_pattern!(usize, String).parse_all("123asd"),
        Some((123, "asd".to_string()))
    );

    assert_eq!(
        input_pattern!(">", usize, String).parse_all(">123asd"),
        Some((123, "asd".to_string()))
    );

    assert_eq!(input_pattern!(">", usize, String).parse_all("123asd"), None);

    assert_eq!(
        input_pattern!([usize, " ", String, " "], String).parse_all("12 a 13 b c"),
        Some((
            vec![(12, "a".to_string()), (13, "b".to_string())],
            "c".to_string()
        ))
    );

    assert_eq!(
        input_pattern!([usize, " "?]).parse_all("1 2 3 4"),
        Some(vec![1, 2, 3, 4])
    );

    assert_eq!(
        input_pattern!([usize, " "?]).parse_all("1 2 3 4 "),
        Some(vec![1, 2, 3, 4])
    );
    assert_eq!(input_pattern!([usize, " "?]).parse_all("1 2 3 4 a"), None);
    assert_eq!(
        input_pattern!([usize, " "?], String).parse_all("1 2 3 4 a"),
        Some((vec![1, 2, 3, 4], "a".to_string()))
    );
    assert_eq!(
        input_pattern!([usize, " "?], String).parse_all("1 2 3 4a"),
        Some((vec![1, 2, 3], "4a".to_string()))
    );

    assert_eq!(
        input_pattern!([String, ":", String, " "?]).parse_all("key:value key2:value2"),
        Some(vec![
            ("key".to_string(), "value".to_string()),
            ("key2".to_string(), "value2".to_string())
        ])
    );
}
