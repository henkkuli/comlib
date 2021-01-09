/// Computes lexicographically next smallest permutation.
///
/// This is done by finding a the longest decreasing suffix of the given slice, and replacing the preceding element by
/// the next smallest element of the suffix.
///
/// # Examples
/// All permutations of a list can be iterated as follows:
/// ```
/// # use comlib_math::next_permutation;
/// let mut list = [1, 3, 2, 4, 2];
/// list.sort();
/// # let mut permutation_count = 0;
/// loop {
///     // Do something with list
/// #   permutation_count += 1;
///
///     if !next_permutation(&mut list) {
///         break;
///     }
/// }
/// # assert_eq!(permutation_count, 60);
/// ```
pub fn next_permutation<T>(data: &mut [T]) -> bool
where
    T: Ord,
{
    // Empty one one-element lists have already visited all of their permutations
    if data.len() <= 1 {
        return false;
    }

    // Iterate from the back until we find two consecutive elements which are not in decreasing order
    let mut i = data.len() - 1;
    while i > 0 && data[i - 1] >= data[i] {
        i -= 1;
    }

    if i == 0 {
        // All permutations done
        data.reverse();
        return false;
    }

    // We have found a decreasing suffix of data. To turn that into the lexicographically next smallest slice, we need
    // to increase the element preceding by as little as possible. This can be done by finding the smallest element in
    // the suffix that is larger than the preceding element, swapping those elements, and finally reversing the suffix.
    for j in (0..data.len()).rev() {
        if data[i - 1] < data[j] {
            data.swap(i - 1, j);
            data[i..].reverse();
            return true;
        }
    }

    unreachable!("The comparison operator must be wrong. At least j == i-1 should have matched.");
}

// TODO: prev_permutation
