//! Module containing functions and structs for iterating subsets.
//!
//! Most users only need [`subsets`] method. See its documentation for examples.

use std::ops::RangeInclusive;

/// Constructs an iterator over subsets of given size.
///
/// For example the subsets of `[1, 2, 3]` are `[]`, `[1]`, `[2]`, `[1, 2]`, `[3]`, `[1, 3]`, `[2, 3]`, and `[1, 2, 3]`.
///
/// Note that the subsets are based on indices and hence this method doesn't do any kind of deduplication of the data.
/// For example subsets of array `[1, 1]` are `[]`, `[1]`, `[1]`, and `[1, 1]`.
///
/// See [`Subset`] for methods implemented on subsets.
///
/// # Examples
/// ```
/// # use comlib_math::subsets;
/// // Let's count the number of positive integers less than 10 that are divided by 2, 3, or 5, using
/// // inclusion-exclusion principle.
/// let primes = [2, 3, 5];
/// let result: i32 = subsets(primes.len())
///     .map(|subset| {
///         // We don't want to consider the empty subset
///         if subset.is_empty() {
///             return 0;
///         }
///
///         // Compute the value represented by the subset of primes
///         let num: i32 = subset.select(&primes).product();
///         // Compute the number of integers at most 100 that are divisible by num
///         let count = 100 / num;
///
///         if subset.len() % 2 == 1 {
///             // We add odd subsets
///             count
///         } else {
///             // and subtract even ones
///             -count
///         }
///     })
///     .sum();
/// assert_eq!(result, 74);
/// ```
pub fn subsets(n: usize) -> Subsets {
    debug_assert!(n <= 64, "Subsets supports at most 64 element sets");
    Subsets {
        mask_iter: 0..=(u64::MAX >> (64 - n)),
    }
}

/// Iterator over subsets.
///
/// Use [`subsets`] to construct. See its documentation for more usage examples.
#[derive(Debug, Clone)]
pub struct Subsets {
    mask_iter: RangeInclusive<u64>,
}

impl Iterator for Subsets {
    type Item = Subset;

    fn next(&mut self) -> Option<Self::Item> {
        self.mask_iter.next().map(|mask| Subset { mask })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // Forward size hint to the inner iterator
        self.mask_iter.size_hint()
    }
}

/// Subset of some elements.
///
/// Use [`subsets`] to construct an iterator to get [`Subset`]s.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Subset {
    /// Bit mask encoding the subset.
    ///
    /// The lowest bit encodes item 0, the second lowest bit encodes item 1 etc.
    pub mask: u64,
}

impl Subset {
    /// Selects the elements belonging to the subset from the given iterator.
    ///
    /// This is especially useful when iterating over all subsets of a list.
    pub fn select<I: IntoIterator>(self, iter: I) -> SubsetIter<I::IntoIter> {
        SubsetIter {
            mask: self.mask,
            iter: iter.into_iter(),
        }
    }

    /// Checks whether the set is empty.
    pub fn is_empty(self) -> bool {
        self.mask == 0
    }

    /// Checks whether the set contains the given element.
    pub fn contains(self, i: usize) -> bool {
        (self.mask >> i) & 1 != 0
    }

    /// Returns the size of the subset.
    pub fn len(self) -> usize {
        self.mask.count_ones() as usize
    }
}

/// An iterator selecting elements based on a subset.
///
/// This can be constructed using the [`select`] method on [`Subset`].
///
/// [`select`]: Subset::select
pub struct SubsetIter<I: Iterator> {
    mask: u64,
    iter: I,
}

impl<I: Iterator> Iterator for SubsetIter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {
            let selects = self.mask & 1 != 0;
            self.mask >>= 1;
            if selects {
                return Some(item);
            }
        }
        None
    }
}
