use std::io::{BufRead, Error, Stdin, StdinLock};
use std::str::FromStr;

mod consumable;
pub use consumable::{Consumable, InputPattern};

/// Helper for reading objects implementing [`InputPattern`] trait.
pub struct Input<T>(T, Option<String>);

impl<'a> Input<StdinLock<'a>> {
    /// Construct [`Input`] from [`&Stdin`].
    ///
    /// [`&Stdin`]: std::io::Stdin
    pub fn from_stdin(stdin: &'a Stdin) -> Self {
        Self(stdin.lock(), None)
    }
}

impl<T> Input<T>
where
    T: BufRead,
{
    /// Peek next line of the input without consuming it.
    pub fn peek_line(&mut self) -> Result<&str, Error> {
        if self.1.is_none() {
            let mut line = String::new();
            self.0.read_line(&mut line)?;
            // Trim control characters from the end
            while line.chars().last().map(|c| c.is_control()).unwrap_or(false) {
                line.pop();
            }
            self.1 = Some(line);
        }
        Ok(self.1.as_ref().unwrap())
    }

    /// Read the next line of the input and consume it.
    pub fn read_line(&mut self) -> Result<String, Error> {
        if self.1.is_none() {
            let mut line = String::new();
            self.0.read_line(&mut line)?;
            // Trim control characters from the end
            while line.chars().last().map(|c| c.is_control()).unwrap_or(false) {
                line.pop();
            }
            self.1 = Some(line);
        }
        Ok(self.1.take().unwrap())
    }

    /// Read line a with the given type.
    ///
    /// If the line can't be parsed in the given type, then None is returned and the line is kept in the input.
    pub fn parse_line<U: FromStr>(&mut self) -> Option<U> {
        let res = U::from_str(self.peek_line().ok()?).ok()?;
        self.read_line().unwrap();
        Some(res)
    }

    /// Read line matching given `pattern`.
    ///
    /// If the line doesn't match the pattern, None is returned and the line is kept in the input.
    ///
    /// See examples of [`input_pattern`] to see how how to use the pattern.
    pub fn match_line<P: InputPattern>(&mut self, pattern: P) -> Option<P::Output> {
        let res = pattern.parse_all(self.peek_line().ok()?)?;
        self.read_line().unwrap();
        Some(res)
    }

    /// Read `n` at most lines matching given `pattern`.
    ///
    /// See examples of [`input_pattern`] to see how how to use the pattern.
    pub fn match_n_lines<P: InputPattern>(&mut self, n: usize, pattern: P) -> Vec<P::Output> {
        self.match_n_lines_impl(usize::MAX, n, pattern)
    }

    /// Read all lines matching given `pattern`.
    ///
    /// See examples of [`input_pattern`] to see how how to use the pattern.
    pub fn match_all_lines<P: InputPattern>(&mut self, pattern: P) -> Vec<P::Output> {
        self.match_n_lines_impl(usize::MAX, 1, pattern)
    }

    /// Implementation for both [`match_n_lines`] and [`match_all_lines`]
    ///
    /// [`match_n_lines`]: Input::match_n_lines
    /// [`match_all_lines`]: Input::match_all_lines
    fn match_n_lines_impl<P: InputPattern>(
        &mut self,
        n: usize,
        reserve: usize,
        pattern: P,
    ) -> Vec<P::Output> {
        let mut res = Vec::with_capacity(reserve);
        for _ in 0..n {
            if let Some(item) = self.match_line(pattern.clone()) {
                res.push(item)
            } else {
                break;
            }
        }
        res
    }
}

impl<T: BufRead> From<T> for Input<T> {
    /// Construct [`Input`] from any [`BufRead`].
    ///
    /// [`BufRead`]: std::io::BufRead
    fn from(reader: T) -> Self {
        Self(reader, None)
    }
}
