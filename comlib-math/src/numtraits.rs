use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};

/// Trait implemented by integers.
pub trait Integer:
    Copy
    + fmt::Display
    + fmt::Debug
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + Eq
    + Ord
    + Default
{
    /// Returns zero.
    fn zero() -> Self;

    /// Tests whether value is zero.
    #[inline(always)]
    fn is_zero(self) -> bool {
        self == Self::zero()
    }

    /// Returns one.
    fn one() -> Self;

    /// Tests whether value is one.
    #[inline(always)]
    fn is_one(self) -> bool {
        self == Self::one()
    }

    /// Converts the given integer to the type.
    ///
    /// This function implements best-effort conversion and may fail in unexpected ways for values unsuitable for the
    /// target data type.
    fn from_int(value: i8) -> Self;
}

macro_rules! impl_integer {
    ($t:ty) => {
        impl Integer for $t {
            #[inline(always)]
            fn zero() -> $t {
                0
            }

            #[inline(always)]
            fn one() -> $t {
                1
            }

            #[inline(always)]
            fn from_int(value: i8) -> $t {
                value as $t
            }
        }
    };
}

impl_integer!(u8);
impl_integer!(u16);
impl_integer!(u32);
impl_integer!(u64);
impl_integer!(u128);
impl_integer!(usize);
impl_integer!(i8);
impl_integer!(i16);
impl_integer!(i32);
impl_integer!(i64);
impl_integer!(i128);
impl_integer!(isize);
