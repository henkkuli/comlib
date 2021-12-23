//! # Comlib Geometric Primitives and Algorithms
//!
//! This crate provides geometric primitives and algorithms that work on them.
//! The main primitive types are [`Point`] and [`Line`].
//! For convenience the crate also provides [`Segment`] and [`Polygon`] types.
//!
//! Currently only the following algorithms have been implemented:
//! - [Convex hull](convex_hull)
#![warn(missing_docs)]

use comlib_math::{gcd, Quot, Signed};
use std::num::NonZeroI64;

mod primitive;
pub use primitive::{Line, LineIntersection, Ordering, Point, Segment, SegmentIntersection};

mod unit;
pub use unit::Unit;

mod polygon;
pub use polygon::{Polygon, PolygonSegmentIter};

pub trait ValidCoordinate: Sized + Signed + PartialEq {
    type Divisor: Into<Self> + Copy;
    type Coordinate: Signed;

    fn try_normalize<const N: usize>(
        values: [Self; N],
        last: Self,
    ) -> Option<([Self; N], Self::Divisor)>;

    fn normalize<const N: usize>(values: [Self; N]) -> [Self; N];

    fn normalize_with_divisor<const N: usize>(
        values: ([Self; N], Self::Divisor),
    ) -> ([Self; N], Self::Divisor);

    fn from_coordinates<const N: usize>(
        coordinates: [Self::Coordinate; N],
    ) -> ([Self; N], Self::Divisor);

    fn to_coordinates<const N: usize>(values: ([Self; N], Self::Divisor)) -> [Self::Coordinate; N];
}

impl ValidCoordinate for i64 {
    type Divisor = NonZeroI64;
    type Coordinate = Quot<i64>;

    fn try_normalize<const N: usize>(
        values: [i64; N],
        last: i64,
    ) -> Option<([i64; N], NonZeroI64)> {
        Some(Self::normalize_with_divisor((
            values,
            NonZeroI64::new(last)?,
        )))
    }

    fn normalize<const N: usize>(values: [i64; N]) -> [i64; N] {
        let mut div = values[0];
        let mut values = values;
        for i in 1..N {
            div = gcd(div, values[i]);
        }
        for i in 0..N {
            values[i] /= div;
        }
        values
    }

    fn normalize_with_divisor<const N: usize>(
        values: ([i64; N], NonZeroI64),
    ) -> ([i64; N], NonZeroI64) {
        let mut div = values.1.get();
        let mut values = values;
        for i in 0..N {
            div = gcd(div, values.0[i]);
        }
        for i in 0..N {
            values.0[i] /= div;
        }
        (values.0, unsafe {
            NonZeroI64::new_unchecked(values.1.get() / div)
        })
    }

    fn from_coordinates<const N: usize>(coordinates: [Quot<i64>; N]) -> ([Self; N], Self::Divisor) {
        let mut div = coordinates[0].denominator();
        for i in 1..N {
            div = (div.checked_mul(coordinates[i].denominator()).unwrap())
                / gcd(div, coordinates[i].denominator());
        }
        let mut values = [0; N];
        for i in 0..N {
            values[i] = coordinates[i].numerator() * (div / coordinates[i].denominator());
        }
        (values, unsafe { NonZeroI64::new_unchecked(div) })
    }

    fn to_coordinates<const N: usize>(values: ([Self; N], NonZeroI64)) -> [Quot<i64>; N] {
        let mut coordinates = [Quot::zero(); N];
        for i in 0..N {
            coordinates[i] = Quot::new_nonzero(values.0[i], values.1);
        }
        coordinates
    }
}

impl ValidCoordinate for f32 {
    type Divisor = Unit;
    type Coordinate = f32;

    fn try_normalize<const N: usize>(mut values: [f32; N], last: f32) -> Option<([f32; N], Unit)> {
        let normalizer = 1.0 / last;
        for i in 0..N {
            values[i] *= normalizer;
        }
        if IntoIterator::into_iter(values).any(f32::is_nan) {
            None
        } else {
            Some((values, Unit))
        }
    }

    fn normalize<const N: usize>(mut values: [f32; N]) -> [f32; N] {
        // Normalize the scale such that the first non-zero variable is 1
        for i in 0..N {
            if values[i] > 1e-6 {
                let multiplier = 1.0 / values[i];
                for j in 0..N {
                    values[j] *= multiplier;
                }
                return values;
            }
        }

        values
    }

    fn normalize_with_divisor<const N: usize>(values: ([f32; N], Unit)) -> ([f32; N], Unit) {
        values
    }

    fn from_coordinates<const N: usize>(coordinates: [f32; N]) -> ([Self; N], Self::Divisor) {
        (coordinates, Unit)
    }

    fn to_coordinates<const N: usize>(values: ([Self; N], Unit)) -> [f32; N] {
        values.0
    }
}

/// Computes the convex hull of the given set of points.
///
/// Return `None` if all points are equal, otherwise returns the convex hull. The convex hull is returned in
/// counter-clockwise order.
///
/// The hull contains all points that are on the edge of the hull.
///
/// # Convex hull
/// A convex hull of a set of points is the minimum-area convex polygon containing all of the points. It is also the
/// minimum-perimeter polygon containing all of the points.
///
/// The intuitive way to think about the convex hull is to think about nails on a board (i.e. the set of the points) and
/// the shape a tight rubber band stretched around the nails would form (i.e. the convex hull).
pub fn convex_hull<T: ValidCoordinate>(mut points: Vec<Point<T>>) -> Option<Polygon<T>> {
    if points.len() <= 1 {
        return None;
    }

    // Sort the points in the order of increasing x-coordinate
    points.sort_by(|a, b| {
        a.x()
            .partial_cmp(&b.x())
            .expect("coordinates to be comparable")
            .then(
                a.y()
                    .partial_cmp(&b.y())
                    .expect("coordinates to be comparable"),
            )
    });

    let mut points = points.into_iter();
    let mut points = points.by_ref();

    // Initialize both hull halves
    let mut lower_hull = vec![points.next().unwrap()];
    for point in &mut points {
        if point != lower_hull[0] {
            lower_hull.push(point);
            break;
        }
    }

    // Check that all points are not the same
    if lower_hull.len() == 1 {
        return None;
    }

    let mut upper_hull = lower_hull.clone();

    // Do one sweep over the point cloud and compute both parts of the hull
    for point in points {
        while lower_hull.len() >= 2
            && Point::ordering([
                lower_hull[lower_hull.len() - 2],
                lower_hull[lower_hull.len() - 1],
                point,
            ]) == Ordering::Clockwise
        {
            lower_hull.pop();
        }
        lower_hull.push(point);

        while upper_hull.len() >= 2
            && Point::ordering([
                upper_hull[upper_hull.len() - 2],
                upper_hull[upper_hull.len() - 1],
                point,
            ]) == Ordering::Counterclockwise
        {
            upper_hull.pop();
        }
        upper_hull.push(point);
    }

    // Make sure that both hulls start and end at the same point
    debug_assert_eq!(lower_hull.first(), upper_hull.first());
    debug_assert_eq!(lower_hull.last(), upper_hull.last());
    debug_assert_ne!(lower_hull.first(), lower_hull.last());

    // Reverse the order of the upper hull and append it to the lower
    upper_hull.reverse();
    lower_hull.extend_from_slice(&upper_hull[1..upper_hull.len() - 1]);

    Some(lower_hull.into())
}

// /// Represents the set of points (x, y) which satisfy ax^2 + ay^2 + bxz + cyz + dz^2 = 0.
// #[derive(Debug, Clone, Copy)]
// pub struct Circle<T:ValidCoordinate> {
//     a: T,
//     b: T,
//     c: T,
//     d: T,
// }

// impl<T:ValidCoordinate>  Circle<T> {
//     pub fn from_center_and_radius<P: Into<Point<T>>, R: Into<Quot<T>>>(center: P, radius: R) -> Self {
//         let center = center.into();
//         let radius = radius.into();

//         let b = center.x() * -2;
//         let c = center.y() * -2;
//         let d = center.x() * center.x() + center.y() * center.y() - radius * radius;
//         let a = lcm(lcm(b.denominator(), c.denominator()), d.denominator());
//         let b = (b * a).numerator();
//         let c = (c * a).numerator();
//         let d = (d * a).numerator();
//         assert_ne!(a, 0, "The circle can't be degenerate");

//         Self { a, b, c, d }
//     }

//     // TODO: Better API, probably one which returns Inside/AtEdge/Outside
//     pub fn contains<P: Into<Point<T>>>(self, p: P) -> bool {
//         let p = p.into();
//         let x = p.x;
//         let y = p.y;
//         let z = p.z.get();
//         self.a * (x * x + y * y) + self.b * x * z + self.c * y * z + self.d * z * z == 0
//     }

//     /// Constructs the line on which the intersections of the given circles reside.
//     pub fn intersection_line(self, other: Self) -> Option<Line<T>> {
//         let a = self.b * other.a - self.a * other.b;
//         let b = self.c * other.a - self.a * other.c;
//         let c = self.d * other.a - self.a * other.d;
//         println!("{} {} {}", a, b, c);

//         Some(Line { a, b, c }.normalized())
//     }

//     /// Computes the squared radius of the circle.
//     pub fn radius2(self) -> Quot<T> {
//         let c = self.center();
//         c.x() * c.x() + c.y() * c.y() - Quot::new(self.d, self.a).unwrap()
//     }

//     pub fn center(self) -> Point<T> {
//         let z = -2 * self.a;
//         let x = self.b;
//         let y = self.c;
//         Point {
//             x,
//             y,
//             z: unsafe { NonZeroI64::new_unchecked(z) },
//         }
//     }
// }

// macro_rules! impl_vec {
//     (impl<$t:ident> math for $v:ident, $dim:tt) => {
//         impl_vec!(@IMPL: impl<$t> Add [add, +] for $v, $dim);
//         impl_vec!(@IMPL: impl<$t> Sub [sub, -] for $v, $dim);
//         impl_vec!(@IMPL: impl<$t> Mul [mul, *] for $v, $dim);
//         impl_vec!(@IMPL: impl<$t> Div [div, /] for $v, $dim);
//         impl_vec!(@IMPL: impl<$t> Rem [rem, %] for $v, $dim);
//         impl_vec!(@IMPL: impl<$t> BitAnd [bitand, &] for $v, $dim);
//         impl_vec!(@IMPL: impl<$t> BitOr [bitor, |] for $v, $dim);
//         impl_vec!(@IMPL: impl<$t> BitXor [bitxor, ^] for $v, $dim);
//         impl_vec!(@IMPL: impl<$t> Shl [shl, <<] for $v, $dim);
//         impl_vec!(@IMPL: impl<$t> Shr [shr, >>] for $v, $dim);

//         impl_vec!(@IMPL ASSIGN: impl<$t> AddAssign [add_assign, +=] for $v, $dim);
//         impl_vec!(@IMPL ASSIGN: impl<$t> SubAssign [sub_assign, -=] for $v, $dim);
//         impl_vec!(@IMPL ASSIGN: impl<$t> MulAssign [mul_assign, *=] for $v, $dim);
//         impl_vec!(@IMPL ASSIGN: impl<$t> DivAssign [div_assign, /=] for $v, $dim);
//         impl_vec!(@IMPL ASSIGN: impl<$t> RemAssign [rem_assign, %=] for $v, $dim);
//         impl_vec!(@IMPL ASSIGN: impl<$t> BitAndAssign [bitand_assign, &=] for $v, $dim);
//         impl_vec!(@IMPL ASSIGN: impl<$t> BitOrAssign [bitor_assign, |=] for $v, $dim);
//         impl_vec!(@IMPL ASSIGN: impl<$t> BitXorAssign [bitxor_assign, ^=] for $v, $dim);
//         impl_vec!(@IMPL ASSIGN: impl<$t> ShlAssign [shl_assign, <<=] for $v, $dim);
//         impl_vec!(@IMPL ASSIGN: impl<$t> ShrAssign [shr_assign, >>=] for $v, $dim);

//         impl_vec!(@IMPL UNARY: impl<$t> Neg [neg, -] for $v, $dim);
//         impl_vec!(@IMPL UNARY: impl<$t> Not [not, !] for $v, $dim);
//     };

//     (@IMPL: impl<$t:ident> $op:path [$f:ident, $o:tt] for $v:ident, $dim:tt) => {
//         impl<$t> $op for $v<$t>
//         where
//             $t: $op,
//         {
//             type Output = $v<<T as $op>::Output>;

//             fn $f(self, rhs: Self) -> Self::Output {
//                 impl_vec!(@OP: $v, self $o rhs, $dim)
//             }
//         }
//     };

//     (@IMPL ASSIGN: impl<$t:ident> $op:path [$f:ident, $o:tt] for $v:ident, $dim:tt) => {
//         impl<$t> $op for $v<$t>
//         where
//             $t: $op,
//         {

//             fn $f(&mut self, rhs: Self) {
//                 impl_vec!(@OP ASSIGN: $v, self $o rhs, $dim);
//             }
//         }
//     };

//     (@IMPL UNARY: impl<$t:ident> $op:path [$f:ident, $o:tt] for $v:ident, $dim:tt) => {
//         impl<$t> $op for $v<$t>
//         where
//             $t: $op,
//         {
//             type Output = $v<<T as $op>::Output>;

//             fn $f(self) -> Self::Output {
//                 impl_vec!(@OP UNARY: $v, $o self, $dim)
//             }
//         }
//     };

//     (@OP: $v:ident, $l:ident $o:tt $r:ident, 1) => {
//         $v {
//             x: $l.x $o $r.x,
//         }
//     };

//     (@OP ASSIGN: $v:ident, $l:ident $o:tt $r:ident, 1) => {
//         $l.x $o $r.x;
//     };

//     (@OP UNARY: $v:ident,  $o:tt $s:ident, 1) => {
//         $v {
//             x: $o $s.x,
//         }
//     };

//     (@OP: $v:ident, $l:ident $o:tt $r:ident, 2) => {
//         $v {
//             x: $l.x $o $r.x,
//             y: $l.y $o $r.y,
//         }
//     };

//     (@OP ASSIGN: $v:ident, $l:ident $o:tt $r:ident, 2) => {
//         $l.x $o $r.x;
//         $l.y $o $r.y;
//     };

//     (@OP UNARY: $v:ident,  $o:tt $s:ident, 2) => {
//         $v {
//             x: $o $s.x,
//             y: $o $s.y,
//         }
//     };

//     (@OP: $v:ident, $l:ident $o:tt $r:ident, 3) => {
//         $v {
//             x: $l.x $o $r.x,
//             y: $l.y $o $r.y,
//             z: $l.z $o $r.z,
//         }
//     };

//     (@OP ASSIGN: $v:ident, $l:ident $o:tt $r:ident, 3) => {
//         $l.x $o $r.x;
//         $l.y $o $r.y;
//         $l.z $o $r.z;
//     };

//     (@OP UNARY: $v:ident,  $o:tt $s:ident, 3) => {
//         $v {
//             x: $o $s.x,
//             y: $o $s.y,
//             z: $o $s.z,
//         }
//     };

//     (@OP: $v:ident, $l:ident $o:tt $r:ident, 4) => {
//         $v {
//             x: $l.x $o $r.x,
//             y: $l.y $o $r.y,
//             z: $l.z $o $r.z,
//             w: $l.w $o $r.w,
//         }
//     };

//     (@OP ASSIGN: $v:ident, $l:ident $o:tt $r:ident, 4) => {
//         $l.x $o $r.x;
//         $l.y $o $r.y;
//         $l.z $o $r.z;
//         $l.w $o $r.w;
//     };

//     (@OP UNARY: $v:ident,  $o:tt $s:ident, 4) => {
//         $v {
//             x: $o $s.x,
//             y: $o $s.y,
//             z: $o $s.z,
//             w: $o $s.w,
//         }
//     };
// }

// // #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// // pub struct Vec1<T> {
// //     pub x: T,
// // }

// pub trait Zero {
//     fn zero() -> Self;
// }

// pub trait One {
//     fn one() -> Self;
// }

// macro_rules! impl_zero_one {
//     ($t:ty) => {
//         impl Zero for $t {
//             fn zero() -> Self {
//                 0 as $t
//             }
//         }

//         impl One for $t {
//             fn one() -> Self {
//                 1 as $t
//             }
//         }
//     };
// }

// impl_zero_one!(i8);
// impl_zero_one!(i16);
// impl_zero_one!(i32);
// impl_zero_one!(i64);
// impl_zero_one!(i128);
// impl_zero_one!(isize);
// impl_zero_one!(u8);
// impl_zero_one!(u16);
// impl_zero_one!(u32);
// impl_zero_one!(u64);
// impl_zero_one!(u128);
// impl_zero_one!(usize);
// impl_zero_one!(f32);
// impl_zero_one!(f64);

// pub trait AlmostEq {
//     fn almost_eq(&self, rhs: &Self) -> bool;
// }

// macro_rules! impl_almost_eq {
//     (exact $t:ty) => {
//         impl AlmostEq for $t {
//             fn almost_eq(&self, rhs: &Self) -> bool {
//                 self == rhs
//             }
//         }
//     };
//     (epsilon $t:ty, $eps:expr) => {
//         impl AlmostEq for $t {
//             fn almost_eq(&self, rhs: &Self) -> bool {
//                 (self - rhs).abs() < $eps
//             }
//         }
//     };
// }
// impl_almost_eq!(exact i8);
// impl_almost_eq!(exact i16);
// impl_almost_eq!(exact i32);
// impl_almost_eq!(exact i64);
// impl_almost_eq!(exact i128);
// impl_almost_eq!(exact isize);
// impl_almost_eq!(exact u8);
// impl_almost_eq!(exact u16);
// impl_almost_eq!(exact u32);
// impl_almost_eq!(exact u64);
// impl_almost_eq!(exact u128);
// impl_almost_eq!(exact usize);
// impl_almost_eq!(epsilon f32, 1e-5);
// impl_almost_eq!(epsilon f64, 1e-8);
// pub trait WithY<T> {
//     fn with_y(self, y: T) -> Vec2<T>;
// }

// impl<T> WithY<T> for T {
//     fn with_y(self, y: T) -> Vec2<T> {
//         Vec2 { x: self, y }
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub struct Vec2<T> {
//     pub x: T,
//     pub y: T,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub struct Vec3<T> {
//     pub x: T,
//     pub y: T,
//     pub z: T,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub struct Vec4<T> {
//     pub x: T,
//     pub y: T,
//     pub z: T,
//     pub w: T,
// }

// // impl_vec!(impl<T> math for Vec1, 1);
// impl_vec!(impl<T> math for Vec2, 2);
// impl_vec!(impl<T> math for Vec3, 3);
// impl_vec!(impl<T> math for Vec4, 4);

// impl<T> Vec2<T> {
//     pub fn new(x: T, y: T) -> Self {
//         Self { x, y }
//     }

//     pub fn dot(self, rhs: Self) -> <T as Mul>::Output
//     where
//         T: Mul,
//         <T as Mul>::Output: Add<Output = <T as Mul>::Output>,
//     {
//         self.x * rhs.x + self.y * rhs.y
//     }

//     pub fn cross(self) -> Vec2<T>
//     where
//         T: Neg<Output = T>,
//     {
//         Vec2 {
//             x: self.y,
//             y: -self.x,
//         }
//     }

//     pub fn with_z(self, z: T) -> Vec3<T> {
//         Vec3 {
//             x: self.x,
//             y: self.y,
//             z,
//         }
//     }
// }

// impl<T> Vec3<T> {
//     pub fn new(x: T, y: T, z: T) -> Self {
//         Self { x, y, z }
//     }

//     pub fn dot(self, rhs: Self) -> <T as Mul>::Output
//     where
//         T: Mul,
//         <T as Mul>::Output: Add<Output = <T as Mul>::Output>,
//     {
//         self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
//     }

//     pub fn cross(self, rhs: Self) -> Vec3<<<T as Mul>::Output as Sub>::Output>
//     where
//         T: Mul + Clone,
//         <T as Mul>::Output: Sub,
//     {
//         Vec3 {
//             x: self.y.clone() * rhs.z.clone() - self.z.clone() * rhs.y.clone(),
//             y: self.z * rhs.x.clone() - self.x.clone() * rhs.z,
//             z: self.x * rhs.y - self.y * rhs.x,
//         }
//     }

//     pub fn with_w(self, w: T) -> Vec4<T> {
//         Vec4 {
//             x: self.x,
//             y: self.y,
//             z: self.z,
//             w,
//         }
//     }
// }

// // struct P2 {
// //     x: isize,
// //     y: isize,
// //     w: isize,
// // }

// /// Point in 2D plane represented by homogeneous coordinates.
// #[derive(Debug, Clone, Copy, Hash)]
// pub struct HP2<T> {
//     x: T,
//     y: T,
//     w: T,
// }

// impl<T> HP2<T> {
//     pub fn into_vec2_f32(self) -> Vec2<f32>
//     where
//         T: Into<f32>,
//     {
//         let w = self.w.into();
//         Vec2 {
//             x: self.x.into() / w,
//             y: self.y.into() / w,
//         }
//     }

//     pub fn into_vec2_f64(self) -> Vec2<f64>
//     where
//         T: Into<f64>,
//     {
//         let w = self.w.into();
//         Vec2 {
//             x: self.x.into() / w,
//             y: self.y.into() / w,
//         }
//     }
// }

// impl<T> From<Vec2<T>> for HP2<T>
// where
//     T: One,
// {
//     fn from(v: Vec2<T>) -> Self {
//         HP2 {
//             x: v.x,
//             y: v.y,
//             w: T::one(),
//         }
//     }
// }

// impl<T> PartialEq for HP2<T>
// where
//     T: Mul + Clone,
//     <T as Mul>::Output: PartialEq,
// {
//     fn eq(&self, other: &Self) -> bool {
//         self.x.clone() * other.w.clone() == other.x.clone() * self.w.clone()
//             && self.y.clone() * other.w.clone() == other.y.clone() * self.w.clone()
//     }
// }

// impl<T> Eq for HP2<T>
// where
//     T: Mul + Clone,
//     <T as Mul>::Output: Eq,
// {
// }

// ///
// ///
// /// Represents the set of points (x, y) which satisfy ax + by = c. Note that this differs from the more common
// /// representation ax + by + c = 0 by the sign of c.
// #[derive(Debug, Clone, Copy, Hash)]
// pub struct Line2<T> {
//     a: T,
//     b: T,
//     c: T,
// }

// #[derive(Debug, Clone, Copy, Hash)]
// pub enum Intersection<T> {
//     None,
//     Point(HP2<T>),
//     Line(Line2<T>),
// }

// impl<T> Intersection<T> {
//     pub fn unwrap_none(self) {
//         match self {
//             Intersection::None => {}
//             Intersection::Point(_) => panic!("expected None but was Point"),
//             Intersection::Line(_) => panic!("expected None but was Line"),
//         }
//     }

//     pub fn unwrap_point(self) -> HP2<T> {
//         match self {
//             Intersection::None => panic!("expected Point but was None"),
//             Intersection::Point(p) => p,
//             Intersection::Line(_) => panic!("expected Point but was Line"),
//         }
//     }

//     pub fn unwrap_line(self) -> Line2<T> {
//         match self {
//             Intersection::None => panic!("expected Line but was None"),
//             Intersection::Point(_) => panic!("expected Line but was Point"),
//             Intersection::Line(l) => l,
//         }
//     }
// }

// impl<T> Line2<T> {
//     pub fn between<A, B, T1>(p1: A, p2: B) -> Line2<T>
//     where
//         A: Into<HP2<T1>>,
//         B: Into<HP2<T1>>,
//         T1: Mul + Clone,
//         <T1 as Mul>::Output: Sub<Output = T>,
//     {
//         let p1 = p1.into();
//         let p2 = p2.into();

//         let a = p2.w.clone() * p1.y.clone() - p1.w.clone() * p2.y.clone();
//         let b = p1.w * p2.x.clone() - p2.w * p1.x.clone();
//         let c = p2.x * p1.y - p1.x * p2.y;

//         Line2 { a, b, c }
//     }

//     pub fn is_on_line<P>(self, p: P) -> bool
//     where
//         P: Into<HP2<T>>,
//         T: Mul,
//         <T as Mul>::Output: Add<Output = T>,
//         T: AlmostEq,
//     {
//         let p = p.into();
//         (self.a * p.x + self.b * p.y).almost_eq(&self.c)
//     }

//     pub fn intersect(self, other: Self) -> Intersection<<<T as Mul>::Output as Sub>::Output>
//     where
//         T: Mul + Clone,
//         <T as Mul>::Output: Sub,
//         <<T as Mul>::Output as Sub>::Output: Zero + AlmostEq,
//     {
//         let candidate = HP2 {
//             x: self.c.clone() * other.b.clone() - self.b.clone() * other.c.clone(),
//             y: self.a.clone() * other.c - self.c * other.a.clone(),
//             w: self.a * other.b - self.b * other.a,
//         };
//         if candidate.w.almost_eq(&Zero::zero()) {
//             Intersection::None
//         } else {
//             Intersection::Point(candidate)
//         }
//     }
// }

// impl<T> PartialEq for Line2<T>
// where
//     T: Mul + Clone,
//     <T as Mul>::Output: PartialEq,
// {
//     fn eq(&self, other: &Self) -> bool {
//         self.a.clone() * other.c.clone() == other.a.clone() * self.c.clone()
//             && self.b.clone() * other.c.clone() == other.b.clone() * self.c.clone()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // TODO: Check vector operations

//     #[test]
//     fn test_is_on_line() {
//         let a = Vec2::new(1, -2);
//         let b = Vec2::new(3, 3);
//         let l = Line2::between(a, b);
//         assert!(l.is_on_line(Vec2::new(1, -2)));
//         assert!(l.is_on_line(Vec2::new(3, 3)));
//         assert!(l.is_on_line(Vec2::new(5, 8)));
//         assert!(!l.is_on_line(Vec2::new(0, 0)));
//         assert!(!l.is_on_line(Vec2::new(-1, 2)));
//         assert!(!l.is_on_line(Vec2::new(2, 0)));
//         assert!(!l.is_on_line(Vec2::new(2, 1)));
//     }

//     // #[test]
//     // fn test_intersect() {
//     //     let l1 = Line2::between(Vec2::new(1, -2), Vec2::new(3, 3));
//     //     let l2 = Line2::between(Vec2::new(1, 1), Vec2::new(2, -2));
//     //     let p1 = l1.intersect(l2).unwrap_point();
//     //     let p2 = l2.intersect(l1).unwrap_point();
//     //     assert_eq!(p1, p2);
//     //     let p1 = p1.into_vec2_f64();
//     //     assert_eq!(p1, Vec2::new(17. / 11., -7. / 11.));

//     //     let l1 = Line2::between(Vec2::new(1, 1), Vec2::new(3, 5));
//     //     let l2 = Line2::between(Vec2::new(2, -2), Vec2::new(3, 0));
//     //     assert_ne!(l1, l2);
//     //     l1.intersect(l2).unwrap_none();
//     //     let l2 = Line2::between(Vec2::new(-2, -5), Vec2::new(-1, -3));
//     //     assert_eq!(l1, l2);
//     //     l1.intersect(l2).unwrap_line();
//     // }
// }
