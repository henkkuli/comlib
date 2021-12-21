use std::{
    fmt, num,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

/// Sign of a number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sign {
    /// Negative sign.
    Negative,
    /// The number has no sign. It can either be 0 or NaN.
    Neutral,
    /// Positive sign.
    Positive,
}

impl Neg for Sign {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Negative => Self::Positive,
            Self::Neutral => Self::Neutral,
            Self::Positive => Self::Negative,
        }
    }
}

/// Trait implemented by numeric types.
pub trait Numeric:
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
    + PartialEq
    + PartialOrd
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

    /// Converts the number to [`f64`].
    fn as_f64(self) -> f64;

    /// Converts the number to [`f32`].
    ///
    /// The default implementation first converts the number into `f64` and then casts it to `f32`.
    fn as_f32(self) -> f32 {
        self.as_f64() as f32
    }
}

/// Trait implemented by signed numbers.
///
/// Contains methods applicable to signed numbers only.
pub trait Signed: Numeric + Neg<Output = Self> {
    /// Returns the sign of the number.
    fn get_sign(self) -> Sign;

    /// Returns the absolute value of the number.
    fn get_abs(self) -> Self;
}

/// Trait implemented by numeric types which cannot contain 0.
pub trait NonZero: Sized + Copy {
    /// Corresponding zeroable numeric type.
    type Base;

    /// Creates a new non-zero value.
    ///
    /// Returns `None` if the given value is 0.
    fn new(value: Self::Base) -> Option<Self>;

    /// Creates a new non-zero value without checking that the value is non-zero.
    ///
    /// # Safety
    /// The given `value` must be non-zero.
    unsafe fn new_unchecked(value: Self::Base) -> Self;

    /// Gets the value as a zeroable type.
    fn get(self) -> Self::Base;
}

/// Trait implemented by integral types.
pub trait Integer: Numeric + Eq + Ord {
    /// The corresponding non-zeroable type.
    type NonZero: NonZero<Base = Self>;
}

macro_rules! impl_numeric {
    ($t:ty) => {
        impl Numeric for $t {
            #[inline(always)]
            fn zero() -> $t {
                0 as $t
            }

            #[inline(always)]
            fn one() -> $t {
                1 as $t
            }

            #[inline(always)]
            fn from_int(value: i8) -> $t {
                value as $t
            }

            #[inline(always)]
            fn as_f64(self) -> f64 {
                self as f64
            }

            #[inline(always)]
            fn as_f32(self) -> f32 {
                self as f32
            }
        }
    };
    ($t:ty, signed) => {
        impl_numeric!($t);
        impl_numeric!($t, signed_only);
    };

    ($t:ty, signed_only) => {
        impl Signed for $t {
            #[inline(always)]
            fn get_sign(self) -> Sign {
                if self < 0 as $t {
                    Sign::Negative
                } else if self > 0 as $t {
                    Sign::Positive
                } else {
                    // Includes both 0 as NANs
                    Sign::Neutral
                }
            }

            #[inline(always)]
            fn get_abs(self) -> Self {
                self.abs()
            }
        }
    };
}
macro_rules! impl_integer {
    ($t:ty, $nonzero:ty) => {
        impl_numeric!($t);
        impl Integer for $t {
            type NonZero = $nonzero;
        }

        impl NonZero for $nonzero {
            type Base = $t;

            fn new(value: Self::Base) -> Option<Self> {
                <$nonzero>::new(value)
            }
            unsafe fn new_unchecked(value: Self::Base) -> Self {
                <$nonzero>::new_unchecked(value)
            }
            fn get(self) -> Self::Base {
                <$nonzero>::get(self)
            }
        }
    };
    ($t:ty, $nonzero:ty, signed) => {
        impl_integer!($t, $nonzero);
        impl_numeric!($t, signed_only);
    };
}

impl_integer!(u8, num::NonZeroU8);
impl_integer!(u16, num::NonZeroU16);
impl_integer!(u32, num::NonZeroU32);
impl_integer!(u64, num::NonZeroU64);
impl_integer!(u128, num::NonZeroU128);
impl_integer!(usize, num::NonZeroUsize);
impl_integer!(i8, num::NonZeroI8, signed);
impl_integer!(i16, num::NonZeroI16, signed);
impl_integer!(i32, num::NonZeroI32, signed);
impl_integer!(i64, num::NonZeroI64, signed);
impl_integer!(i128, num::NonZeroI128, signed);
impl_integer!(isize, num::NonZeroIsize, signed);

/// Trait implemented by floating-point numbers.
pub trait Float: Numeric + Signed {
    /// Computes the square root of the number
    fn get_sqrt(self) -> Self;
}

macro_rules! impl_float {
    ($t:ty) => {
        impl_numeric!($t, signed);

        impl Float for $t {
            #[inline(always)]
            fn get_sqrt(self) -> $t {
                self.sqrt()
            }
        }
    };
}

impl_float!(f32);
impl_float!(f64);
