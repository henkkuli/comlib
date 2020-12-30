use std::{
    fmt,
    ops::{Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};

/// Modulus contains the modulus and the actual storage type of [`ModInt`].
pub trait Modulus {
    /// Type for holding values mod `modulus()`.
    ///
    /// Must be large enough to contain the square of `modulus() - 1`, that is the maximum value which can be encountered
    /// during multiplication.
    type Base: Copy
        + fmt::Display
        + fmt::Debug
        + Add<Output = Self::Base>
        + Sub<Output = Self::Base>
        + Mul<Output = Self::Base>
        + Div<Output = Self::Base>
        + Rem<Output = Self::Base>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + RemAssign
        + Eq
        + Ord
        + Default
        + From<u8>;

    /// Modulus in which computations should be done.
    fn modulus(self) -> Self::Base;

    /// The power a value needs to be raised to in order to get its modular inverse.
    ///
    /// If the modulus is a prime, then this is `modulus() - 2` by [Fermat's little theorem](https://en.wikipedia.org/wiki/Fermat%27s_little_theorem).
    /// For composite numbers this must be `φ(modulus()) - 1` where `φ` is the [Euler's totient function](https://en.wikipedia.org/wiki/Euler%27s_totient_function)
    /// which gives the number of integers smaller than `modulus()` which are coprime with `modulus()`.
    fn inverse_power(self) -> usize;
}

/// Commonly used modulus 10⁹ + 7.
#[derive(Debug, Clone, Copy, Default)]
pub struct Mod1e9p7;
impl Modulus for Mod1e9p7 {
    type Base = u64;

    #[inline(always)]
    fn modulus(self) -> u64 {
        1_000_000_007
    }

    #[inline(always)]
    fn inverse_power(self) -> usize {
        (self.modulus() - 2) as usize
    }
}

/// Modulus whose value can be selected at runtime.
///
/// The modulus must be a prime. Otherwise the value returned by [`inverse_power`] is wrong and division won't work.
///
/// [`inverse_power`]: RuntimePrimeModulus::inverse_power
#[derive(Debug, Clone, Copy)]
pub struct RuntimePrimeModulus<T>(T);

impl<T> Modulus for RuntimePrimeModulus<T>
where
    T: Copy
        + fmt::Display
        + fmt::Debug
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + RemAssign
        + Eq
        + Ord
        + Default
        + From<u8>
        + Into<usize>,
{
    type Base = T;

    #[inline(always)]
    fn modulus(self) -> T {
        self.0
    }

    #[inline(always)]
    fn inverse_power(self) -> usize {
        self.modulus().into() - 2
    }
}

impl<T> From<T> for RuntimePrimeModulus<T> {
    fn from(modulus: T) -> Self {
        Self(modulus)
    }
}

/// Integer type for which all computations are executed in modulo given by [`M::modulus`].
///
/// # Examples
/// ```
/// # use comlib_math::{ModInt, RuntimePrimeModulus};
/// let p = RuntimePrimeModulus::from(5u8);
/// let a = ModInt::from((7, p));
/// assert_eq!(a, ModInt::from((2, p)));
/// let b = ModInt::from((3, p));
/// assert_eq!(*(a + b), 0);
/// assert_eq!(*(a * b), 1);
/// assert_eq!(*(b - a), 1);
/// assert_eq!(*(a - b), 4);
/// assert_eq!(*(a / b), 4);
/// ```
/// [`M::modulus`]: Modulus::modulus
pub struct ModInt<M: Modulus + Copy>(M::Base, M);

impl<M: Modulus + Copy> ModInt<M> {
    /// Computes the value raised to the given power.
    pub fn pow(self, rhs: usize) -> Self {
        if rhs == 0 {
            Self(M::Base::from(1) % self.1.modulus(), self.1)
        } else if rhs == 1 {
            self
        } else if rhs % 2 == 0 {
            let p = self.pow(rhs / 2);
            p * p
        } else {
            self * self.pow(rhs - 1)
        }
    }

    /// Computes the inverse of the value.
    #[inline(always)]
    pub fn inv(self) -> Self {
        self.pow(self.1.inverse_power())
    }
}

impl<M: Modulus + Copy> Clone for ModInt<M> {
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}
impl<M: Modulus + Copy> Copy for ModInt<M> {}

impl<M: Modulus + Copy> Default for ModInt<M>
where
    M: Default,
{
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}

impl<M: Modulus + Copy> fmt::Debug for ModInt<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} (mod {:?})", self.0, self.1.modulus())
    }
}
impl<M: Modulus + Copy> fmt::Display for ModInt<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<M: Modulus + Copy> PartialEq for ModInt<M> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<M: Modulus + Copy> Eq for ModInt<M> {}

impl<M: Modulus + Copy> From<u64> for ModInt<M>
where
    M: Default,
    M::Base: From<u64>,
{
    fn from(val: u64) -> Self {
        let modulus: M = Default::default();
        Self(M::Base::from(val) % modulus.modulus(), modulus)
    }
}

impl<M: Modulus + Copy> Deref for ModInt<M> {
    type Target = M::Base;
    fn deref(&self) -> &M::Base {
        &self.0
    }
}

impl<M: Modulus + Copy> From<(M::Base, M)> for ModInt<M> {
    fn from((val, modulus): (M::Base, M)) -> Self {
        Self(val % modulus.modulus(), modulus)
    }
}

impl<M: Modulus + Copy> Add for ModInt<M> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self((self.0 + rhs.0) % self.1.modulus(), self.1)
    }
}

impl<M: Modulus + Copy> AddAssign for ModInt<M> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<M: Modulus + Copy> Mul for ModInt<M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self((self.0 * rhs.0) % self.1.modulus(), self.1)
    }
}

impl<M: Modulus + Copy> MulAssign for ModInt<M> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<M: Modulus + Copy> Sub for ModInt<M> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut l = self.0;
        let r = rhs.0;
        if l < r {
            l += self.1.modulus();
        }
        Self(l - r, self.1)
    }
}

impl<M: Modulus + Copy> SubAssign for ModInt<M> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<M: Modulus + Copy> Div for ModInt<M> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl<M: Modulus + Copy> DivAssign for ModInt<M> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
