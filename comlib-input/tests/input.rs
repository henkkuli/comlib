use comlib_input::*;
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
#[should_panic]
fn test_match_too_few_lines() {
    let input = "1\n2\n3\n4\n5\n6";
    Input::from(Cursor::new(input)).match_lines(input_pattern!(usize), 7..);
}
