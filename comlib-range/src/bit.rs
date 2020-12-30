use std::{
    fmt,
    ops::{Add, AddAssign, Bound, RangeBounds, Sub, SubAssign},
};

/// Binary indexed tree.
///
/// [Binary indexed tree](https://en.wikipedia.org/wiki/Binary_indexed_tree), also known as Fenwick tree, is a data
/// structure that allows effectively updating a single value and querying the sum of values over a range.
///
/// Note that unlike most binary indexed tree implementations, this implementation uses 0-indexing instead of 1-indexing.
///
/// # Time complexity
/// All operations on binary indexed tree take `O(log n)` time, unless otherwise stated.
#[derive(Clone, Eq, PartialEq)]
pub struct Bit<T>(Vec<T>);

impl<T> Bit<T> {
    /// Computes the sum of values up to and including the given index.
    fn sum_until(&self, index: usize) -> T
    where
        T: Add<Output = T> + Clone,
    {
        let mut sum = self.0[index].clone();
        let mut index = index;
        while index > (index + 1) & (!index) {
            index -= (index + 1) & (!index);
            sum = sum + self.0[index].clone();
        }

        sum
    }

    /// Computes the sum of values on the given range.
    pub fn sum<R: RangeBounds<usize>>(&self, range: R) -> T
    where
        T: Add<Output = T> + Sub<Output = T> + Clone + Default,
    {
        // Find the sum until the non-inclusive lower bound.
        let lower = match range.start_bound() {
            Bound::Excluded(i) => self.sum_until(*i),
            Bound::Included(0) | Bound::Unbounded => Default::default(),
            Bound::Included(i) => self.sum_until(i - 1),
        };

        // Find the sum until the inclusive upper bound.
        let upper = match range.end_bound() {
            Bound::Excluded(0) => Default::default(),
            Bound::Excluded(i) => self.sum_until(i - 1),
            Bound::Included(i) => self.sum_until(*i),
            Bound::Unbounded => self.sum_until(self.0.len() - 1),
        };

        upper - lower
    }

    /// Increases the value at the given index by the given value.
    pub fn add(&mut self, index: usize, value: T)
    where
        T: AddAssign + Clone,
    {
        let mut index = index;
        while index < self.0.len() {
            self.0[index] += value.clone();
            index += (index + 1) & (!index);
        }
    }

    /// Decreases the value at the given index by the given value.
    pub fn sub(&mut self, index: usize, value: T)
    where
        T: SubAssign + Clone,
    {
        let mut index = index;
        while index < self.0.len() {
            self.0[index] -= value.clone();
            index += (index + 1) & (!index);
        }
    }
}

impl<T> From<Vec<T>> for Bit<T>
where
    T: AddAssign + Clone,
{
    /// Constructs binary indexed tree from the given `Vec`.
    ///
    /// # Time complexity
    /// Construction takes `O(n log n)` time.
    fn from(data: Vec<T>) -> Self {
        let mut data = data;
        let n = data.len();
        for index in (0..n).rev() {
            let offset = index + 1;
            let data = data.split_at_mut(index + 1);
            let v = &data.0[index];
            let data = data.1;

            let mut index = index + ((index + 1) & (!index));
            while index < n {
                data[index - offset] += v.clone();
                index += (index + 1) & (!index);
            }
        }
        Self(data)
    }
}

/// [`Debug`] on [`Bit`] prints the original values of the array, _not_ the values in the nodes of the tree.
///
/// # Time complexity
/// Printing takes `O(n log n)` time.
///
/// [`Debug`]: std::fmt::Debug
impl<T> fmt::Debug for Bit<T>
where
    T: Add<Output = T> + Sub<Output = T> + Clone + Default + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.0.len();
        f.debug_list()
            .entries((0..n).map(|i| self.sum(i..=i)))
            .finish()
    }
}
