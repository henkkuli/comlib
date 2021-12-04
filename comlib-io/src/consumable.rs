/// Value which can be greedily consumed from `&str`.
pub trait Consumable
where
    Self: Sized,
{
    /// Type for representing errors which can happen during consuming values.
    type InputError;

    /// Consume input from the given `&str`.
    ///
    /// In case of success, returns the consumed value and the rest of the string.
    /// In case of an error, returns the error.
    fn consume(input: &str) -> Result<(Self, &str), Self::InputError>;
}

/// Parser pattern for parsing input.
///
/// The easiest way to implement this trait is to use [`input_pattern`] macro.
pub trait InputPattern: Clone {
    /// Type the parser produces.
    type Output;
    /// Parse the longest prefix which matches the pattern.
    ///
    /// Returns the parsed prefix and the rest of the string.
    /// Returns `None` if the prefix of the input doesn't match the pattern.
    fn parse_prefix<'a>(&self, input: &'a str) -> Option<(Self::Output, &'a str)>;

    /// Parse the whole string.
    ///
    /// Returns `None` if the input doesn't match the pattern.
    fn parse_all(&self, input: &str) -> Option<Self::Output> {
        match self.parse_prefix(input) {
            Some((output, "")) => Some(output),
            _ => None,
        }
    }
}

macro_rules! consumable {
    ($t:ty, $p:expr, $e:ty) => {
        impl Consumable for $t {
            type InputError = $e;
            fn consume(input: &str) -> Result<(Self, &str), Self::InputError> {
                #[allow(unused_mut)]
                let mut p = $p;
                let value = input.splitn(2, |c| !p(c)).next().unwrap();
                let (value, rest) = input.split_at(value.len());
                let value = value.parse::<$t>();
                match value {
                    Ok(value) => Ok((value, rest)),
                    Err(err) => Err(err),
                }
            }
        }
    };
}

macro_rules! int_pattern {
    ($($sign:tt)+) => {{
        let mut first = true;
        move |c| {
            if first {
                first = false;
                matches!(c, $($sign)+ | '0'..='9')
            } else {
                matches!(c, '0'..='9')
            }
        }
    }};
}

macro_rules! float_pattern {
    () => {{
        let mut first = true;
        let mut has_dot = true;
        move |c| {
            if first {
                first = false;
                has_dot = c == '.';
                matches!(c, '+' | '-' | '.' | '0'..='9')
            } else if has_dot {
                matches!(c, '0'..='9')
            } else {
                has_dot = c == '.';
                matches!(c, '.' | '0'..='9')
            }
        }
    }};
}

consumable!(
    char,
    {
        let mut first = true;
        move |_| {
            let r = first;
            first = false;
            r
        }
    },
    std::char::ParseCharError
);
consumable!(u8, int_pattern!('+'), std::num::ParseIntError);
consumable!(i8, int_pattern!('+' | '-'), std::num::ParseIntError);
consumable!(u16, int_pattern!('+'), std::num::ParseIntError);
consumable!(i16, int_pattern!('+' | '-'), std::num::ParseIntError);
consumable!(u32, int_pattern!('+'), std::num::ParseIntError);
consumable!(i32, int_pattern!('+' | '-'), std::num::ParseIntError);
consumable!(u64, int_pattern!('+'), std::num::ParseIntError);
consumable!(i64, int_pattern!('+' | '-'), std::num::ParseIntError);
consumable!(u128, int_pattern!('+'), std::num::ParseIntError);
consumable!(i128, int_pattern!('+' | '-'), std::num::ParseIntError);
consumable!(usize, int_pattern!('+'), std::num::ParseIntError);
consumable!(isize, int_pattern!('+' | '-'), std::num::ParseIntError);
consumable!(f32, float_pattern!(), std::num::ParseFloatError);
consumable!(f64, float_pattern!(), std::num::ParseFloatError);

/// String consumes the rest of the input
impl Consumable for String {
    type InputError = std::convert::Infallible;
    fn consume(input: &str) -> Result<(Self, &str), Self::InputError> {
        Ok((input.to_owned(), ""))
    }
}

/// Pattern for parsing input
///
/// The patterns matched greedily from the start. Each typed input is matched for as long prefix as possible. If the
/// typed input is followed by a string pattern, then the type is matched until the first occurrence of that pattern,
/// otherwise the type is matched greedily for as long prefix of the input as possible. For numeric types this means
/// until the first non-numeral character, and for strings until the end of the string. Note that for numeric types this
/// doesn't take into account that the value fits into the given type.
///
/// The pattern may contain any type which implements [`FromStr`] when it's followed by a string pattern, or any type
/// which implements [`Consumable`] when it's not followed by a string pattern.
///
/// The parser can parse variable number of occurrences of the pattern as [`Vec`]s. The variable arguments are enclosed
/// in `[brackets]` and can contain any valid pattern, including more vectors.
///
/// A string pattern can be made optional, in which case it is not necessary that it occurs in the input. Greedy
/// matching is still stopped at the first occurrence of the pattern if it exists. Optional patterns are especially
/// useful with vector patterns and to match plurals of words.
///
/// See examples on how to use the `input_pattern`.
///
/// [`Vec`]: std::vec::Vec
/// [`FromStr`]: std::str::FromStr
/// [`Consumable`]: Consumable
///
/// # Examples
/// ```rust
/// # #[macro_use] extern crate comlib_io;
/// # fn main() {
/// use comlib_io::{input_pattern, InputPattern};
///
/// // Parse two numerals separated by a space
/// assert_eq!(input_pattern!(usize, " ", usize).parse_all("1 2"), Some((1, 2)));
///
/// // Parse key-value pair
/// assert_eq!(input_pattern!(String, ": ", usize).parse_all("key: 2"), Some(("key".to_string(), 2)));
///
/// // Parse a vector of space-separated numbers
/// assert_eq!(input_pattern!([usize, " "?]).parse_all("1 2 3 4"), Some(vec![1, 2, 3, 4]));
///
/// // Parse a vector of key-value pairs
/// assert_eq!(
///     input_pattern!([String, " = ", String, " "?]).parse_all("key1 = v1 key2 = v2"),
///     Some(vec![
///         ("key1".to_string(), "v1".to_string()),
///         ("key2".to_string(), "v2".to_string()),
///     ])
/// );
///
/// // Without the optional space separator in the pattern, the string won't be matched
/// assert_eq!(
///     input_pattern!([String, " = ", String, " "]).parse_all("key1 = v1 key2 = v2"),
///     None
/// );
///
/// // Match both singular and plural form
/// assert_eq!(
///     input_pattern!([usize, " ", String, " item", "s"?, ", "?]).parse_all("1 red item, 2 blue items"),
///     Some(vec![
///         (1, "red".to_string()),
///         (2, "blue".to_string()),
///     ])
/// );
/// # }
/// ```
#[macro_export]
macro_rules! input_pattern {
    () => {
        $crate::input_pattern!("")
    };

    ( $($pattern:tt)+ ) => {
        {

            #[derive(Copy, Clone, Debug)]
            struct Pattern;
            impl $crate::InputPattern for Pattern {
                type Output = $crate::input_pattern_impl!(@OUT, @(), $($pattern)*, );
                fn parse_prefix<'a>(&self, input: &'a str) -> Option<(Self::Output, &'a str)> {
                    $crate::input_pattern_impl!(@START, input, $($pattern)+ )
                }
            }
            Pattern
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! input_pattern_impl {
    (@START, $input:expr, $pattern:literal, $($rest_pattern:tt)+ ) => {
        if let Some(rest) = $crate::strip_prefix($input, $pattern) {
            $crate::input_pattern_impl!(@IMPL, rest, @(), $($rest_pattern)*, )
        } else {
            None
        }
    };
    (@START, $input:expr, $($rest_pattern:tt)+ ) => {
        $crate::input_pattern_impl!(@IMPL, $input, @(), $($rest_pattern)*, )
    };

    // Arrays followed by a non-optional pattern are matched until the pattern
    (@IMPL, $input:expr, @($($consumed:expr),*), [$($inner:tt)+], $pattern:literal, $($rest_pattern:tt)* ) => {
        {
            {
                let input = $input;
                let array_pattern = input_pattern!([$($inner)+]);
                let mut parts = input.splitn(2, $pattern);
                let array = parts.next().unwrap();
                let rest = input.split_at(array.len()).1;
                match array_pattern.parse_all(array) {
                    Some(content) => {
                        $crate::input_pattern_impl!(@IMPL, rest, @($($consumed,)* content), $pattern, $($rest_pattern)*)
                    }
                    None => None,
                }
            }
        }
    };

    // Other arrays are greedily matched
    (@IMPL, $input:expr, @($($consumed:expr),*), [$($inner:tt)+], $($rest_pattern:tt)* ) => {
        {
            let parser = input_pattern!($($inner)+);
            let mut vec = Vec::new();
            let mut input = $input;
            if input != ""{
                while let Some((parsed, rest)) = parser.parse_prefix(input) {
                    vec.push(parsed);
                    input = rest;
                    if input == "" {
                        break;
                    }
                }
            }
            $crate::input_pattern_impl!(@IMPL, input, @($($consumed,)* vec), $($rest_pattern)*)
        }
    };

    // Type followed by a pattern is read until the pattern and the whole preceding part is matched
    (@IMPL, $input:expr, @($($consumed:expr),*), $type:ty, $pattern:literal $($rest_pattern:tt)* ) => {
        {
            let input = $input;
            let mut parts = input.splitn(2, $pattern);
            let item = parts.next().unwrap();
            let rest = input.split_at(item.len()).1;
            match item.parse::<$type>() {
                Ok(item) => {
                    $crate::input_pattern_impl!(@IMPL, rest, @($($consumed,)* item), $pattern $($rest_pattern)*)
                }
                Err(_) => None,
            }
        }
    };

    // Type implies greedy matching
    (@IMPL, $input:expr, @($($consumed:expr),*), $type:ty, $($rest_pattern:tt)* ) => {
        {
            use $crate::Consumable;
            match <$type>::consume($input) {
                Ok((item, rest)) => {
                    $crate::input_pattern_impl!(@IMPL, rest, @($($consumed,)* item), $($rest_pattern)*)
                }
                Err(_) => None,
            }
        }
    };

    // Optional pattern may be matched, or it may be ignored
    (@IMPL, $input:expr, @($($consumed:expr),*), $pattern:literal?, $($rest_pattern:tt)* ) => {
        {
            let input = $input;
            if let Some(rest) = $crate::strip_prefix(input, $pattern) {
                $crate::input_pattern_impl!(@IMPL, rest, @($($consumed),*), $($rest_pattern)*)
            } else {
                $crate::input_pattern_impl!(@IMPL, input, @($($consumed),*), $($rest_pattern)*)
            }
        }
    };

    // // Repeated pattern with + must be matched at least once
    (@IMPL, $input:expr, @($($consumed:expr),*), $pattern:literal+, $($rest_pattern:tt)* ) => {
        {
            let input = $input;
            if let Some(rest) = $crate::strip_prefix(input, $pattern) {
                $crate::input_pattern_impl!(@IMPL, rest, @($($consumed),*), $pattern*, $($rest_pattern)*)
            } else {
                None
            }
        }
    };

    // Repeated pattern with * can be matched any number of times
    (@IMPL, $input:expr, @($($consumed:expr),*), $pattern:literal*, $($rest_pattern:tt)* ) => {
        {
            let mut input = $input;
            while let Some(rest) = $crate::strip_prefix(input, $pattern) {
                input = rest;
            }
            $crate::input_pattern_impl!(@IMPL, input, @($($consumed),*), $($rest_pattern)*)
        }
    };

    // Pattern must be matched
    (@IMPL, $input:expr, @($($consumed:expr),*), $pattern:literal, $($rest_pattern:tt)* ) => {
        if let Some(rest) = $crate::strip_prefix($input, $pattern) {
            $crate::input_pattern_impl!(@IMPL, rest, @($($consumed),*), $($rest_pattern)*)
        } else {
            None
        }
    };

    // Combine consumed values into return value
    (@IMPL, $input:expr, @($($consumed:expr),*), ) => {
        Some((($($consumed),*), $input))
    };

    // Output type inference
    (@OUT, @($($types:ty),*), [$($inner:tt)+], $($rest:tt)* ) => {
        $crate::input_pattern_impl!(@OUT, @($($types,)* Vec<
            $crate::input_pattern_impl!(@OUT, @(), $($inner)+, )
        >), $($rest)*)
    };

    (@OUT, @($($types:ty),*), $type:ty, $($rest:tt)* ) => {
        $crate::input_pattern_impl!(@OUT, @($($types,)* $type), $($rest)*)
    };

    (@OUT, @($($types:ty),*), $pattern:literal?, $($rest:tt)* ) => {
        $crate::input_pattern_impl!(@OUT, @($($types),*), $($rest)*)
    };

    (@OUT, @($($types:ty),*), $pattern:literal+, $($rest:tt)* ) => {
        $crate::input_pattern_impl!(@OUT, @($($types),*), $($rest)*)
    };

    (@OUT, @($($types:ty),*), $pattern:literal*, $($rest:tt)* ) => {
        $crate::input_pattern_impl!(@OUT, @($($types),*), $($rest)*)
    };

    (@OUT, @($($types:ty),*), $pattern:literal, $($rest:tt)* ) => {
        $crate::input_pattern_impl!(@OUT, @($($types),*), $($rest)*)
    };

    (@OUT, @($type:ty), ) => {
        $type
    };

    (@OUT, @($($types:ty),*), ) => {
        ($($types),*)
    };
}

/// Backport of str::strip_prefix
#[doc(hidden)]
pub fn strip_prefix<'a>(string: &'a str, prefix: &str) -> Option<&'a str> {
    if string.starts_with(prefix) {
        Some(string.split_at(prefix.len()).1)
    } else {
        None
    }
}
