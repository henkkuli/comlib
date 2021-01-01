use std::ops::{Bound, RangeBounds};

use comlib_math::{Mod1e9p7, ModInt, Modulus};
use comlib_range::Bit;
use rand::{thread_rng, RngCore};

/// Rolling hash for strings
///
/// # Current implementation
/// The rolling hash is based on the following idea: Let
/// <code>s = c<sub>0</sub>c<sub>1</sub>c<sub>2</sub>...c<sub>n-1</sub></code> be a string and let `x` be an element of
/// a modular group. Now we can construct a hash
/// <code>h = c<sub>0</sub> + c<sub>1</sub>x + c<sub>2</sub>x<sup>2</sup> + ... + c<sub>n-1</sub>x<sup>n-1</sup></code>
/// for the whole string. Because `h` is evaluated in a modular group, it is not unique, but it is unlikely to find two
/// strings which produce the same hash unless they are produced by an adversary<sup>1</sup>. Now the interesting part
/// is that we can compute the hash for any substring
/// <code>s<sub>l...r</sub> = c<sub>l</sub>c<sub>l+1</sub>...c<sub>r-1</sub>c<sub>r</sub></code> as
/// <code>h<sub>l...r</sub> = (c<sub>l</sub>x<sup>l</sup> + c<sub>l+1</sub>x<sup>l+1</sup> + ... + c<sub>r</sub>x<sup>r</sup>) / x<sup>l</sup></code>.
/// The sum can be computed efficiently by storing the terms in a [Binary indexed tree](comlib_range::Bit) which allow
/// querying the sum over a range in `O(log n)` time. The Binary indexed tree also allows updating the terms of the sum
/// in `O(log n)` time meaning that we can modify the string one character at a time.
///
/// <sup>1</sup>: The change of a collision attack is tried to be mitigated by randomly choosing the value of `x` for
/// each run.
#[derive(Clone)]
pub struct RollingHash<M = Mod1e9p7>
where
    M: Modulus + Copy,
{
    /// Terms of the hash
    hashes: Bit<ModInt<M>>,
    /// Original characters to facilitate easier modifications.
    chars: Vec<char>,
    /// The group element used for hashing.
    x: ModInt<M>,
}

impl<M> RollingHash<M>
where
    M: Modulus + Copy + Default,
    M::Base: From<u64>,
{
    /// Constructs new `RollingHash`.
    ///
    /// The `x` is chosen randomly
    pub fn new<S: AsRef<str>>(input: S) -> Self {
        // Choose random `x`
        let x = ModInt::from(thread_rng().next_u64());
        Self::with_x(input, x)
    }

    /// Constructs new `RollingHash` which uses the given `x`.
    pub fn with_x<S: AsRef<str>>(input: S, x: ModInt<M>) -> Self {
        let input = input.as_ref();
        let chars: Vec<char> = input.chars().collect();
        let hashes = Bit::from(
            chars
                .iter()
                .copied()
                // Construct the terms of the hash iteratively.
                .scan(ModInt::from((M::Base::from(1), x.modulus())), |s, c| {
                    // Value for the current term
                    let hash = *s * ModInt::from((M::Base::from(c as u64), x.modulus()));
                    // Iteratively increase the power of x
                    *s *= x;
                    Some(hash)
                })
                .collect::<Vec<_>>(),
        );

        Self { hashes, chars, x }
    }

    /// Gets the hash of the substring over the given range.
    ///
    /// Note that the range is given in characters, not in bytes like with [`str`].
    pub fn get_hash<R: RangeBounds<usize>>(&self, range: R) -> ModInt<M> {
        let x = match range.start_bound() {
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i + 1,
            Bound::Unbounded => 0,
        };
        let x_pow = self.x.pow(x);

        self.hashes.sum(range) / x_pow
    }

    /// Replaces the character at the given index with new one.
    pub fn set_char(&mut self, index: usize, new_char: char) {
        let old_char = std::mem::replace(&mut self.chars[index], new_char);
        let x_pow = self.x.pow(index);

        // The binary indexed tree allows for efficient additions and subtractions at the given positions. Compute the
        // difference needed to change the term, namely `x^i (c_new - c_old)`.
        let change = x_pow
            * (ModInt::from((M::Base::from(new_char as u64), x_pow.modulus()))
                - ModInt::from((M::Base::from(old_char as u64), x_pow.modulus())));

        self.hashes.add(index, change);
    }

    /// Returns the x used for hashing.
    pub fn x(&self) -> ModInt<M> {
        self.x
    }
}
