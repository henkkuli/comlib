use std::{
    fmt,
    ops::{Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};

/// Modulus contains the modulus and the actual storage type of [`ModInt`].
pub trait Modulus: Copy {
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
}

/// Marker trait for indicating that modular numbers in this modulo can be inverted.
pub trait InvertibleModulus: Modulus {
    /// Computes the inverse of the given [`ModInt`].
    fn inverse(self, value: ModInt<Self>) -> ModInt<Self>;
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
pub struct ModInt<M: Modulus>(M::Base, M);

impl<M: Modulus> ModInt<M> {
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
    pub fn inv(self) -> Self
    where
        M: InvertibleModulus,
    {
        self.modulus().inverse(self)
    }

    /// Returns the modulus used.
    pub fn modulus(self) -> M {
        self.1
    }

    /// Returns the inner value without the modulus.
    ///
    /// Always in the range `[0, self.modulus())`.
    pub fn into_inner(self) -> M::Base {
        self.0
    }
}

impl<M: Modulus> Clone for ModInt<M> {
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}
impl<M: Modulus> Copy for ModInt<M> {}

impl<M: Modulus> Default for ModInt<M>
where
    M: Default,
{
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}

impl<M: Modulus> fmt::Debug for ModInt<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} (mod {:?})", self.0, self.1.modulus())
    }
}
impl<M: Modulus> fmt::Display for ModInt<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<M: Modulus> PartialEq for ModInt<M> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<M: Modulus> Eq for ModInt<M> {}

impl<M: Modulus> From<u8> for ModInt<M>
where
    M: Default,
{
    fn from(val: u8) -> Self {
        let modulus: M = Default::default();
        Self(M::Base::from(val) % modulus.modulus(), modulus)
    }
}

impl<M: Modulus> From<u64> for ModInt<M>
where
    M: Default,
    M::Base: From<u64>,
{
    fn from(val: u64) -> Self {
        let modulus: M = Default::default();
        Self(M::Base::from(val) % modulus.modulus(), modulus)
    }
}

impl<M: Modulus> Deref for ModInt<M> {
    type Target = M::Base;
    fn deref(&self) -> &M::Base {
        &self.0
    }
}

impl<M: Modulus> From<(M::Base, M)> for ModInt<M> {
    fn from((val, modulus): (M::Base, M)) -> Self {
        Self(val % modulus.modulus(), modulus)
    }
}

impl<M: Modulus> Add for ModInt<M> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self((self.0 + rhs.0) % self.1.modulus(), self.1)
    }
}

impl<M: Modulus> AddAssign for ModInt<M> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<M: Modulus> Mul for ModInt<M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self((self.0 * rhs.0) % self.1.modulus(), self.1)
    }
}

impl<M: Modulus> MulAssign for ModInt<M> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<M: Modulus> Sub for ModInt<M> {
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

impl<M: Modulus> SubAssign for ModInt<M> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<M: Modulus + InvertibleModulus> Div for ModInt<M> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl<M: Modulus + InvertibleModulus> DivAssign for ModInt<M> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
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
}

impl InvertibleModulus for Mod1e9p7 {
    #[inline(always)]
    fn inverse(self, value: ModInt<Self>) -> ModInt<Self> {
        value.pow(self.modulus() as usize - 2)
    }
}

/// Modulus whose value can be selected at runtime.
#[derive(Debug, Clone, Copy)]
pub struct RuntimeModulus<T>(T);

/// Modulus whose value can be selected at runtime.
///
/// The modulus must be a prime. This differs from [`RuntimeModulus`] in that this implements [`InvertibleModulus`]
/// because numbers in prime modulus can be inverted by taking their p-2:th power.
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
}

impl<T> InvertibleModulus for RuntimePrimeModulus<T>
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
    /// Computes the inverse of the given [`ModInt`].
    ///
    /// Because the modulus is known to be a prime, this can be computed as `value.pow(self.modulus() - 2)` due to
    /// [Fermat's little theorem](https://en.wikipedia.org/wiki/Fermat%27s_little_theorem).
    #[inline(always)]
    fn inverse(self, value: ModInt<Self>) -> ModInt<Self> {
        value.pow(self.modulus().into() - 2)
    }
}

impl<T> From<T> for RuntimePrimeModulus<T> {
    fn from(modulus: T) -> Self {
        Self(modulus)
    }
}
