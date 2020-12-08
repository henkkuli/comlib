use comlib_input::*;

#[test]
fn test_consume_usize() {
    assert_eq!(usize::consume("123"), Ok((123, "")));
    assert_eq!(usize::consume("123 "), Ok((123, " ")));
    assert_eq!(usize::consume("123 asdf"), Ok((123, " asdf")));
    assert_eq!(usize::consume("123asdf"), Ok((123, "asdf")));
    assert_eq!(usize::consume("123 1234"), Ok((123, " 1234")));
    assert!(usize::consume("ads").is_err());
    assert!(usize::consume("ads adsf").is_err());
    assert!(usize::consume("ads 123").is_err());
    assert!(usize::consume("").is_err());
    assert!(usize::consume("a123 1234").is_err());

    assert_eq!(usize::consume("+123"), Ok((123, "")));
    assert_eq!(usize::consume("+123 "), Ok((123, " ")));
}

#[test]
fn test_consume_isize() {
    assert_eq!(isize::consume("123"), Ok((123, "")));
    assert_eq!(isize::consume("123 "), Ok((123, " ")));
    assert_eq!(isize::consume("123 asdf"), Ok((123, " asdf")));
    assert_eq!(isize::consume("123asdf"), Ok((123, "asdf")));
    assert_eq!(isize::consume("123 1234"), Ok((123, " 1234")));
    assert!(isize::consume("ads").is_err());
    assert!(isize::consume("ads adsf").is_err());
    assert!(isize::consume("ads 123").is_err());
    assert!(isize::consume("").is_err());
    assert!(isize::consume("a123 1234").is_err());

    assert_eq!(isize::consume("-123"), Ok((-123, "")));
    assert_eq!(isize::consume("-123 "), Ok((-123, " ")));
    assert_eq!(isize::consume("-123 asdf"), Ok((-123, " asdf")));
    assert_eq!(isize::consume("-123asdf"), Ok((-123, "asdf")));
    assert_eq!(isize::consume("-123 1234"), Ok((-123, " 1234")));
    assert_eq!(isize::consume("-123-1234"), Ok((-123, "-1234")));
    assert!(isize::consume("-ads").is_err());
    assert!(isize::consume("-ads adsf").is_err());
    assert!(isize::consume("-ads 123").is_err());
    assert!(isize::consume("").is_err());
    assert!(isize::consume("-").is_err());
    assert!(isize::consume("-a123 1234").is_err());

    assert_eq!(isize::consume("+123"), Ok((123, "")));
    assert_eq!(isize::consume("+123 "), Ok((123, " ")));
}

#[test]
fn test_consume_char() {
    assert_eq!(char::consume("123"), Ok(('1', "23")));
    assert_eq!(char::consume(" 123"), Ok((' ', "123")));
    assert_eq!(char::consume("a"), Ok(('a', "")));
    assert!(char::consume("").is_err());
}

#[test]
fn test_consume_string() {
    assert_eq!(String::consume("123"), Ok(("123".to_string(), "")));
    assert_eq!(String::consume("123 "), Ok(("123 ".to_string(), "")));
    assert_eq!(
        String::consume("hello world!"),
        Ok(("hello world!".to_string(), ""))
    );
}

#[test]
fn test_consume_f32() {
    assert_eq!(f32::consume("123"), Ok((123., "")));
    assert_eq!(f32::consume("123 "), Ok((123., " ")));
    assert_eq!(f32::consume("123.4"), Ok((123.4, "")));
    assert_eq!(f32::consume("-123.4"), Ok((-123.4, "")));
    assert_eq!(f32::consume("-123"), Ok((-123., "")));
    assert_eq!(f32::consume("0123.4"), Ok((123.4, "")));
    assert_eq!(f32::consume("-0123.4"), Ok((-123.4, "")));
    assert_eq!(f32::consume("123.4.5"), Ok((123.4, ".5")));
    assert_eq!(f32::consume(".4.5"), Ok((0.4, ".5")));

    assert_eq!(f32::consume("+123.4"), Ok((123.4, "")));
    assert_eq!(f32::consume("+123"), Ok((123., "")));
}
