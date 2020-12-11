use std::io::{BufRead, Error, ErrorKind, Stdin, StdinLock};
use std::{ops::RangeBounds, str::FromStr};

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
    /// Update cache containing next line.
    fn update_cache(&mut self) -> Result<(), Error> {
        if self.1.is_none() {
            let mut line = String::new();
            let read_len = self.0.read_line(&mut line)?;
            if read_len == 0 {
                // Nothing found, this has to be the end
                return Err(ErrorKind::Other.into());
            }
            // Trim control characters from the end
            while line.chars().last().map(|c| c.is_control()).unwrap_or(false) {
                line.pop();
            }
            self.1 = Some(line);
        }
        Ok(())
    }

    /// Peek next line of the input without consuming it.
    pub fn peek_line(&mut self) -> Result<&str, Error> {
        self.update_cache()?;
        Ok(self.1.as_ref().unwrap())
    }

    /// Read the next line of the input and consume it.
    pub fn read_line(&mut self) -> Result<String, Error> {
        self.update_cache()?;
        Ok(self.1.take().unwrap())
    }

    /// Read line a with the given type.
    ///
    /// # Panics
    /// Panics if the line can't be converted to `U`.
    #[track_caller]
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
    #[track_caller]
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
    #[track_caller]
    pub fn match_lines<P, R>(&mut self, pattern: P, range: R) -> Vec<P::Output>
    where
        P: InputPattern,
        R: RangeBounds<usize> + std::fmt::Debug,
    {
        let mut res = vec![];
        let mut lower_bound_reached = range.contains(&0);
        while !lower_bound_reached || range.contains(&(res.len() + 1)) {
            if let Some(item) = self.match_line_opt(pattern.clone()) {
                res.push(item);
            } else {
                break;
            }
            if range.contains(&res.len()) {
                lower_bound_reached = true;
            }
        }
        assert!(
            lower_bound_reached,
            "not enough lines matched the pattern. {} lines matched, but expected {:?} lines",
            res.len(),
            range
        );
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
