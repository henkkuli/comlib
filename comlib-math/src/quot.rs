use crate::{gcd, Integer, NonZero, Numeric, Signed};
use std::{fmt, ops};

/// A quotient. Represents a rational number as `numerator/denominator`.
///
/// The denominator is stored as non-zeroable type, so this type benefits from the niche optimization.
#[derive(Clone, Copy)]
pub struct Quot<T: Integer> {
    numerator: T,
    denominator: T::NonZero,
}

impl<T: Integer> Quot<T> {
    /// Constructs a new `Quot` from a numerator and a denominator.
    ///
    /// Return `None` if the denominator is 0.
    pub fn new(numerator: T, denominator: T) -> Option<Self> {
        Some(
            Self {
                numerator,
                denominator: T::NonZero::new(denominator)?,
            }
            .normalized(),
        )
    }

    /// Constructs a new `Quot` from a numerator and a non-zero denominator.
    pub fn new_nonzero(numerator: T, denominator: T::NonZero) -> Self {
        Self {
            numerator,
            denominator,
        }
        .normalized()
    }

    /// Gets the 0-value.
    pub fn zero() -> Self {
        Self {
            numerator: T::zero(),
            denominator: unsafe { T::NonZero::new_unchecked(T::one()) },
        }
    }

    /// Turns the value into normalized form.
    ///
    /// The numerator and the denominator of a quotient in the normalized form don't have a shared divisor.
    fn normalized(self) -> Self {
        let div = gcd(self.numerator, self.denominator.get());
        Self {
            numerator: self.numerator / div,
            denominator: unsafe { T::NonZero::new_unchecked(self.denominator.get() / div) },
        }
    }

    /// Gets the numerator of the quotient.
    pub fn numerator(self) -> T {
        self.numerator
    }

    /// Gets the denominator of the quotient.
    pub fn denominator(self) -> T {
        self.denominator.get()
    }

    /// Truncates the quotient.
    ///
    /// This gives out the integral part of the quotient.
    pub fn trunc(self) -> T {
        self.numerator() / self.denominator() * self.denominator()
    }

    /// Computes the absolute value of the quotient.
    pub fn abs(self) -> Self
    where
        T: Signed,
    {
        Self {
            numerator: self.numerator.get_abs(),
            denominator: self.denominator,
        }
    }
}

impl<T: Integer> fmt::Debug for Quot<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}/{:?}", self.numerator(), self.denominator())
    }
}

impl<T: Integer> fmt::Display for Quot<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denominator() == T::one() {
            write!(f, "{}", self.numerator())
        } else {
            write!(f, "{}/{}", self.numerator(), self.denominator())
        }
    }
}

impl<T: Integer> Default for Quot<T> {
    fn default() -> Self {
        Self::zero()
    }
}

// Math operators

impl<T: Integer, R> ops::Add<R> for Quot<T>
where
    R: Into<Self>,
{
    type Output = Self;

    fn add(self, rhs: R) -> Self::Output {
        let rhs = rhs.into();
        Self::new(
            self.numerator() * rhs.denominator() + rhs.numerator() * self.denominator(),
            self.denominator() * rhs.denominator(),
        )
        .unwrap()
    }
}

impl<T: Integer, R> ops::Sub<R> for Quot<T>
where
    R: Into<Self>,
{
    type Output = Self;

    fn sub(self, rhs: R) -> Self::Output {
        let rhs = rhs.into();
        Self::new(
            self.numerator() * rhs.denominator() - rhs.numerator() * self.denominator(),
            self.denominator() * rhs.denominator(),
        )
        .unwrap()
    }
}

impl<T: Integer + Signed> ops::Neg for Quot<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}

impl<T: Integer, R> ops::Mul<R> for Quot<T>
where
    R: Into<Self>,
{
    type Output = Self;

    fn mul(self, rhs: R) -> Self::Output {
        let rhs = rhs.into();
        Self::new(
            self.numerator() * rhs.numerator(),
            self.denominator() * rhs.denominator(),
        )
        .unwrap()
    }
}

impl<T: Integer, R> ops::Div<R> for Quot<T>
where
    R: Into<Self>,
{
    type Output = Self;

    fn div(self, rhs: R) -> Self::Output {
        let rhs = rhs.into();
        Self::new(
            self.numerator() * rhs.denominator(),
            self.denominator() * rhs.numerator(),
        )
        .unwrap()
    }
}

impl<T: Integer, R> ops::Rem<R> for Quot<T>
where
    R: Into<Self>,
{
    type Output = Self;

    fn rem(self, rhs: R) -> Self::Output {
        let rhs = rhs.into();
        self - Self::from((self / rhs).trunc()) * rhs
    }
}

// Assign versions of the operators

impl<T: Integer, R> ops::AddAssign<R> for Quot<T>
where
    R: Into<Self>,
{
    fn add_assign(&mut self, rhs: R) {
        *self = *self + rhs
    }
}

impl<T: Integer, R> ops::SubAssign<R> for Quot<T>
where
    R: Into<Self>,
{
    fn sub_assign(&mut self, rhs: R) {
        *self = *self - rhs
    }
}

impl<T: Integer, R> ops::MulAssign<R> for Quot<T>
where
    R: Into<Self>,
{
    fn mul_assign(&mut self, rhs: R) {
        *self = *self * rhs
    }
}

impl<T: Integer, R> ops::DivAssign<R> for Quot<T>
where
    R: Into<Self>,
{
    fn div_assign(&mut self, rhs: R) {
        *self = *self / rhs
    }
}

impl<T: Integer, R> ops::RemAssign<R> for Quot<T>
where
    R: Into<Self>,
{
    fn rem_assign(&mut self, rhs: R) {
        *self = *self % rhs
    }
}

// Comparisons

impl<T: Integer> PartialEq for Quot<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl<T: Integer> Eq for Quot<T> where Self: PartialEq {}

impl<T: Integer> PartialOrd for Quot<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Integer> Ord for Quot<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.numerator() * other.denominator()).cmp(&(other.numerator() * self.denominator()))
    }
}

impl<T: Integer> From<T> for Quot<T> {
    fn from(v: T) -> Self {
        Quot {
            numerator: v,
            denominator: unsafe { T::NonZero::new_unchecked(T::one()) },
        }
    }
}

impl<T: Integer> From<&T> for Quot<T> {
    fn from(v: &T) -> Self {
        Quot {
            numerator: *v,
            denominator: unsafe { T::NonZero::new_unchecked(T::one()) },
        }
    }
}

impl<T: Integer> Numeric for Quot<T> {
    fn zero() -> Self {
        T::zero().into()
    }

    fn one() -> Self {
        T::one().into()
    }

    fn from_int(value: i8) -> Self {
        Quot {
            numerator: T::from_int(value),
            denominator: unsafe { T::NonZero::new_unchecked(T::one()) },
        }
    }

    fn as_f32(self) -> f32 {
        self.numerator().as_f32() / self.denominator().as_f32()
    }

    fn as_f64(self) -> f64 {
        self.numerator().as_f64() / self.denominator().as_f64()
    }
}

impl<T: Integer + Signed> Signed for Quot<T> {
    fn get_sign(self) -> crate::Sign {
        let signum = self.numerator().get_sign();
        match self.denominator().get_sign() {
            crate::Sign::Negative => -signum,
            crate::Sign::Neutral => unreachable!("The denominator should never be 0"),
            crate::Sign::Positive => signum,
        }
    }

    fn get_abs(self) -> Self {
        self.abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn denominator_is_positive() {
        assert_eq!(Quot::new(2, 1).unwrap().denominator(), 1);
        assert_eq!(Quot::new(2, -1).unwrap().denominator(), 1);
        assert_eq!(Quot::new(-2, 1).unwrap().denominator(), 1);
        assert_eq!(Quot::new(-2, -1).unwrap().denominator(), 1);
    }
}
