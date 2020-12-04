use comlib_input::*;
use std::io::Cursor;

#[test]
fn test_parse_line() {
    let input = "1\nasd\n3";
    let mut input = Input::from(Cursor::new(input));
    assert_eq!(input.parse_line::<usize>(), Some(1));
    assert_eq!(input.parse_line::<usize>(), None);
    assert_eq!(input.parse_line::<String>(), Some("asd".into()));
    assert_eq!(input.parse_line::<usize>(), Some(3));
    assert_eq!(input.parse_line::<usize>(), None);
}

#[test]
fn test_match_line() {
    let input = "1\nasd\n3";
    let mut input = Input::from(Cursor::new(input));
    assert_eq!(input.match_line(input_pattern!(usize)), Some(1));
    assert_eq!(input.match_line(input_pattern!(usize)), None);
    assert_eq!(input.match_line(input_pattern!(String)), Some("asd".into()));
    assert_eq!(input.match_line(input_pattern!(usize)), Some(3));
    assert_eq!(input.match_line(input_pattern!(usize)), None);
}
