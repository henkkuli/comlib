use comlib_io::*;
use std::io::Cursor;

#[test]
fn test_parse_line() {
    let input = "1\nasd\n3";
    let mut input = Input::from(Cursor::new(input));
    assert_eq!(input.parse_line_opt::<usize>(), Some(1));
    assert_eq!(input.parse_line_opt::<usize>(), None);
    assert_eq!(input.parse_line_opt::<String>(), Some("asd".into()));
    assert_eq!(input.parse_line_opt::<usize>(), Some(3));
    assert_eq!(input.parse_line_opt::<usize>(), None);
}

#[test]
fn test_match_line() {
    let input = "1\nasd\n3";
    let mut input = Input::from(Cursor::new(input));
    assert_eq!(input.match_line_opt(input_pattern!(usize)), Some(1));
    assert_eq!(input.match_line_opt(input_pattern!(usize)), None);
    assert_eq!(
        input.match_line_opt(input_pattern!(String)),
        Some("asd".into())
    );
    assert_eq!(input.match_line_opt(input_pattern!(usize)), Some(3));
    assert_eq!(input.match_line_opt(input_pattern!(usize)), None);
}

#[test]
fn test_match_lines() {
    let input = "1\n2\n3\n4\n5\n6";
    assert_eq!(
        Input::from(Cursor::new(input)).match_lines(input_pattern!(usize), ..=1),
        &[1]
    );
    assert_eq!(
        Input::from(Cursor::new(input)).match_lines(input_pattern!(usize), 1..=4),
        &[1, 2, 3, 4]
    );
    assert_eq!(
        Input::from(Cursor::new(input)).match_lines(input_pattern!(usize), ..100),
        &[1, 2, 3, 4, 5, 6]
    );
    assert_eq!(
        Input::from(Cursor::new(input)).match_lines(input_pattern!(usize), ..),
        &[1, 2, 3, 4, 5, 6]
    );
    assert_eq!(
        Input::from(Cursor::new(input)).match_lines(input_pattern!(usize), 4..),
        &[1, 2, 3, 4, 5, 6]
    );
}

#[test]
fn test_match_lines_opt() {
    let input = "1 2 3\n4 5 6\nhello";
    let mut reader = Input::from(Cursor::new(input));
    assert!(reader
        .match_lines_opt(input_pattern!([i32, " "?]), 1..)
        .is_some());
    assert_eq!(reader.read_line().unwrap(), "hello");
}

#[test]
fn test_match_string_lines() {
    let input = "l1\nl2\n";
    assert_eq!(
        Input::from(Cursor::new(input)).match_lines(input_pattern!(String), ..),
        vec!["l1".to_string(), "l2".to_string()]
    );
}

#[test]
fn test_match_no_lines() {
    let input = "hello\n";
    assert_eq!(
        Input::from(Cursor::new(input)).match_lines(input_pattern!(usize), ..),
        vec![]
    );
}

#[test]
#[should_panic]
fn test_match_too_few_lines() {
    let input = "1\n2\n3\n4\n5\n6";
    Input::from(Cursor::new(input)).match_lines(input_pattern!(usize), 7..);
}
