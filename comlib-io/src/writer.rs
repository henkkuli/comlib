use std::fmt;

/// Implementation for [`spaced`].
#[derive(Debug, Clone, Copy)]
pub struct SpacedWriter<I>(I)
where
    I: Iterator + Clone,
    <I as Iterator>::Item: fmt::Display;

impl<I> fmt::Display for SpacedWriter<I>
where
    I: Iterator + Clone,
    <I as Iterator>::Item: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut it = self.0.clone();
        if let Some(v) = it.next() {
            write!(f, "{}", v)?;
            for v in it {
                write!(f, " {}", v)?;
            }
        }

        Ok(())
    }
}

/// Wrapper for displaying items of iterator as separated by spaces.
///
/// Note that this does *not* mean that the [`Debug`](std::fmt::Debug) would format the values as separated by spaces.
///
/// # Examples
/// ```
/// # use comlib_io::*;
/// assert_eq!(format!("{}", spaced(vec![1, 2, 3])), "1 2 3");
/// ```
pub fn spaced<I>(i: I) -> SpacedWriter<<I as IntoIterator>::IntoIter>
where
    I: IntoIterator,
    <I as IntoIterator>::IntoIter: Clone,
    <I as IntoIterator>::Item: fmt::Display,
{
    SpacedWriter(i.into_iter())
}
