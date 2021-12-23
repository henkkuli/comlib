use comlib_math::Numeric;
use std::{fmt, ops};

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Unit;

impl Into<f32> for Unit {
    fn into(self) -> f32 {
        1.
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "1")
    }
}

impl ops::Add for Unit {
    type Output = Unit;

    fn add(self, _: Self) -> Self::Output {
        panic!("Units cannot be added")
    }
}
impl ops::Sub for Unit {
    type Output = Unit;

    fn sub(self, _: Self) -> Self::Output {
        panic!("Units cannot be subtracted")
    }
}
impl ops::Mul for Unit {
    type Output = Unit;

    fn mul(self, _: Self) -> Self::Output {
        Unit
    }
}
impl ops::Div for Unit {
    type Output = Unit;

    fn div(self, _: Self) -> Self::Output {
        Unit
    }
}
impl ops::Rem for Unit {
    type Output = Unit;

    fn rem(self, _: Self) -> Self::Output {
        panic!("Units cannot be computed remainder of")
    }
}
impl ops::AddAssign for Unit {
    fn add_assign(&mut self, _: Self) {
        panic!("Units cannot be added")
    }
}
impl ops::SubAssign for Unit {
    fn sub_assign(&mut self, _: Self) {
        panic!("Units cannot be subtracted")
    }
}
impl ops::MulAssign for Unit {
    fn mul_assign(&mut self, _: Self) {}
}
impl ops::DivAssign for Unit {
    fn div_assign(&mut self, _: Self) {}
}
impl ops::RemAssign for Unit {
    fn rem_assign(&mut self, _: Self) {
        panic!("Units cannot be computed remainder of")
    }
}

impl Numeric for Unit {
    fn zero() -> Self {
        panic!("Unit cannot be zero")
    }

    fn one() -> Self {
        Self
    }

    fn from_int(_: i8) -> Self {
        panic!("Unit cannot be created from integer")
    }

    fn as_f64(self) -> f64 {
        1.0
    }
}

impl ops::Mul<f32> for Unit {
    type Output = f32;

    fn mul(self, rhs: f32) -> Self::Output {
        rhs
    }
}
