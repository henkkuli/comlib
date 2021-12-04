//! # Comlib Input/Output Utilities
//! This library contains commonly needed utilities for handling IO.
//!
//! The most commonly needed API is the [`Input`] struct which can be used to parse input. The following code reads a
//! line count `n` from the standard input and then reads the following `n` lines, each containing a pair of numbers.
//! ```
//! use comlib_io::*;
//! let stdin = std::io::stdin();
//! let mut input = Input::from_stdin(&stdin);
//! #
//! # // For actual testing, let's produce some input
//! # let mut input = Input::from(std::io::Cursor::new("1\n1 2"));
//! let n = input.match_line(input_pattern!(usize));
//! let queries = input.match_lines(input_pattern!(usize, " ", usize), n..=n);
//! ```
//!
//! The library also contains some useful utilities for printing out multiple values. Currently the following wrappers
//! are supported:
//! - [Writing space-separated values](spaced)

#![warn(missing_docs)]

use std::collections::VecDeque;
use std::io::{BufRead, Error, ErrorKind, Stdin, StdinLock};
use std::ops::Bound;
use std::{ops::RangeBounds, str::FromStr};

mod consumable;
pub use consumable::{strip_prefix, Consumable, InputPattern};

mod writer;
pub use writer::spaced;

/// Helper for reading objects implementing [`InputPattern`] trait.
pub struct Input<T> {
    input: T,
    cache: VecDeque<String>,
}

impl<'a> Input<StdinLock<'a>> {
    /// Construct [`Input`] from [`&Stdin`].
    ///
    /// [`&Stdin`]: std::io::Stdin
    #[must_use]
    pub fn from_stdin(stdin: &'a Stdin) -> Self {
        Self {
            input: stdin.lock(),
            cache: VecDeque::new(),
        }
    }
}

impl<T> Input<T>
where
    T: BufRead,
{
    // Tries to read a raw line from the input, skipping the cache.
    fn try_read_raw_line(&mut self) -> Result<String, Error> {
        let mut line = String::new();

        let read_len = self.input.read_line(&mut line)?;
        if read_len == 0 {
            // Nothing found, this has to be the end
            return Err(ErrorKind::Other.into());
        }

        // Trim control characters from the end
        while line.chars().last().map_or(false, char::is_control) {
            line.pop();
        }

        Ok(line)
    }

    /// Ensure that cache contains at least one line.
    fn ensure_cache_contains_line(&mut self) -> Result<(), Error> {
        if self.cache.is_empty() {
            let line = self.try_read_raw_line()?;
            self.cache.push_back(line);
        }
        Ok(())
    }

    /// Peek next line of the input without consuming it.
    pub fn peek_line(&mut self) -> Result<&str, Error> {
        self.ensure_cache_contains_line()?;
        Ok(self.cache.front().unwrap())
    }

    /// Read the next line of the input and consume it.
    pub fn read_line(&mut self) -> Result<String, Error> {
        self.ensure_cache_contains_line()?;
        Ok(self.cache.pop_front().unwrap())
    }

    /// Read line a with the given type.
    ///
    /// # Panics
    /// Panics if the line can't be converted to `U`.
    // #[track_caller] // TODO: Once submission environments accept this, add back
    pub fn parse_line<U: FromStr>(&mut self) -> U {
        self.parse_line_opt().unwrap()
    }

    /// Read line a with the given type.
    ///
    /// If the line can't be parsed in the given type, then None is returned and the line is kept in the input.
    pub fn parse_line_opt<U: FromStr>(&mut self) -> Option<U> {
        let res = U::from_str(self.peek_line().ok()?).ok()?;
        self.read_line().unwrap();
        Some(res)
    }

    /// Read line matching given `pattern`.
    ///
    /// See examples of [`input_pattern`] to see how how to use the pattern.
    ///
    /// # Panics
    /// Panics if the line doesn't match the pattern.
    // #[track_caller] // TODO: Once submission environments accept this, add back
    pub fn match_line<P: InputPattern>(&mut self, pattern: P) -> P::Output {
        self.match_line_opt(pattern).unwrap()
    }

    /// Read line matching given `pattern`.
    ///
    /// If the line doesn't match the pattern, None is returned and the line is kept in the input.
    ///
    /// See examples of [`input_pattern`] to see how how to use the pattern.
    pub fn match_line_opt<P: InputPattern>(&mut self, pattern: P) -> Option<P::Output> {
        let res = pattern.parse_all(self.peek_line().ok()?)?;
        self.read_line().unwrap();
        Some(res)
    }

    /// Read multiple lines matching given `pattern`.
    ///
    /// # Panics
    /// Panics if not enough of the lines match the pattern.
    // #[track_caller] // TODO: Once submission environments accept this, add back
    pub fn match_lines<P, R>(&mut self, pattern: P, range: R) -> Vec<P::Output>
    where
        P: InputPattern,
        R: RangeBounds<usize> + std::fmt::Debug,
    {
        let res = self
            .match_lines_opt(pattern, (Bound::Included(0), range.end_bound().cloned()))
            .unwrap();
        assert!(
            range.contains(&res.len()),
            "not enough lines matched the pattern. {} lines matched, but expected {:?} lines",
            res.len(),
            range
        );
        res
    }

    /// Read multiple lines matching given `pattern`.
    ///
    /// Returns None if not enough lines could be matched.
    pub fn match_lines_opt<P, R>(&mut self, pattern: P, range: R) -> Option<Vec<P::Output>>
    where
        P: InputPattern,
        R: RangeBounds<usize> + std::fmt::Debug,
    {
        let mut res = vec![];
        let mut lower_bound_reached = range.contains(&0);
        while !lower_bound_reached || range.contains(&(res.len() + 1)) {
            // Get the next line if it exists, or read it and add to cache
            let line = match self.cache.get(res.len()) {
                Some(line) => line,
                None => {
                    match self.try_read_raw_line() {
                        Ok(line) => {
                            self.cache.push_back(line);
                            self.cache.get(res.len()).unwrap()
                        }
                        Err(_) => break, // No more input
                    }
                }
            };

            if let Some(item) = pattern.parse_all(line) {
                res.push(item);
            } else {
                break; // Failed to parse the line
            }
            if range.contains(&res.len()) {
                lower_bound_reached = true;
            }
        }
        if range.contains(&res.len()) {
            // Result is correct. Consume it from the cache
            self.cache = self.cache.split_off(res.len());
            Some(res)
        } else {
            None
        }
    }
}

impl<T: BufRead> From<T> for Input<T> {
    /// Construct [`Input`] from any [`BufRead`].
    ///
    /// [`BufRead`]: std::io::BufRead
    fn from(reader: T) -> Self {
        Self {
            input: reader,
            cache: VecDeque::new(),
        }
    }
}
