use crate::ValidCoordinate;
use comlib_math::{Float, Numeric, Sign, Signed};
use std::fmt;

#[derive(Clone, Copy, Eq, PartialOrd, Ord, Hash)]
pub struct Point<T: ValidCoordinate> {
    pub x: T,
    pub y: T,
    pub z: T::Divisor,
}

impl<T: ValidCoordinate> Point<T> {
    pub fn new(x: T, y: T, z: T::Divisor) -> Self {
        Self { x, y, z }.normalized()
    }

    pub fn try_new(x: T, y: T, z: T) -> Option<Self> {
        let ([x, y], z) = T::try_normalize([x, y], z)?;
        Some(Self { x, y, z })
    }

    pub fn normalize(&mut self) {
        *self = self.normalized();
    }

    pub fn normalized(self) -> Self {
        let normalized = T::normalize_with_divisor(([self.x, self.y], self.z));
        Self {
            x: normalized.0[0],
            y: normalized.0[1],
            z: normalized.1,
        }
    }

    pub fn to_f32_pair(self) -> (f32, f32) {
        (self.x().as_f32(), self.y().as_f32())
    }

    pub fn to_f64_pair(self) -> (f64, f64) {
        (self.x().as_f64(), self.y().as_f64())
    }

    pub fn to_f32(self) -> Point<f32> {
        let (x, y) = self.to_f32_pair();
        Point::from((x, y))
    }

    // pub fn to_f64(self) -> Point<f64> {
    //     let (x, y) = self.to_f64_pair();
    //     Point::from((x, y))
    // }

    pub fn x(self) -> T::Coordinate {
        T::to_coordinates(([self.x], self.z))[0]
    }

    pub fn y(self) -> T::Coordinate {
        T::to_coordinates(([self.y], self.z))[0]
    }

    pub fn ordering<P: Into<Self>>(points: [P; 3]) -> Ordering {
        let [p0, p1, p2] = points;
        let points = [p0.into(), p1.into(), p2.into()];
        // Compute the sign of the signed area of the parallelogram spanned by the points
        let dx1 = points[1].x() - points[0].x();
        let dy1 = points[1].y() - points[0].y();
        let dx2 = points[2].x() - points[0].x();
        let dy2 = points[2].y() - points[0].y();
        match (dx1 * dy2 - dx2 * dy1).get_sign() {
            Sign::Negative => Ordering::Clockwise,
            Sign::Neutral => Ordering::Collinear,
            Sign::Positive => Ordering::Counterclockwise,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Ordering {
    Counterclockwise,
    Collinear,
    Clockwise,
}

// TODO: Generic
// impl<T: ValidCoordinate> From<(T::Coordinate, T::Coordinate)> for Point<T> {
//     fn from((x, y): (T::Coordinate, T::Coordinate)) -> Self {
//         let ([x, y], z) = T::from_coordinates([x, y]);
//         Self { x, y, z }
//     }
// }
impl From<(i64, i64)> for Point<i64> {
    fn from((x, y): (i64, i64)) -> Self {
        let ([x, y], z) = i64::from_coordinates([x.into(), y.into()]);
        Self { x, y, z }
    }
}

impl From<(f32, f32)> for Point<f32> {
    fn from((x, y): (f32, f32)) -> Self {
        let ([x, y], z) = f32::from_coordinates([x.into(), y.into()]);
        Self { x, y, z }
    }
}

impl<T: ValidCoordinate> fmt::Debug for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({0}/{2}, {1}/{2})", self.x, self.y, self.z.into())
    }
}
impl<T: ValidCoordinate> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({0}/{2}, {1}/{2})", self.x, self.y, self.z.into())
    }
}

impl<T: ValidCoordinate> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        let self_z = self.z.into();
        let other_z = other.z.into();
        other_z * self.x == self_z * other.x && other_z * self.y == self_z * other.y
    }
}

/// Represents the set of points (x, y) which satisfy ax + by + c = 0.
#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
pub struct Line<T: ValidCoordinate> {
    pub a: T,
    pub b: T,
    pub c: T,
}

impl<T: ValidCoordinate> Line<T> {
    pub fn spanned_by<P1: Into<Point<T>>, P2: Into<Point<T>>>(p1: P1, p2: P2) -> Self {
        let p1 = p1.into();
        let p2 = p2.into();
        let p1_z = p1.z.into();
        let p2_z = p2.z.into();

        let a = p2_z * p1.y - p1_z * p2.y;
        let b = p1_z * p2.x - p2_z * p1.x;
        let c = p1.x * p2.y - p2.x * p1.y;

        Self { a, b, c }.normalized()
    }

    pub fn normalize(&mut self) {
        *self = self.normalized();
    }

    pub fn normalized(self) -> Self {
        let [a, b, c] = T::normalize([self.a, self.b, self.c]);
        Self { a, b, c }
    }

    pub fn contains<P: Into<Point<T>>>(self, p: P) -> bool {
        let p = p.into();
        (self.a * p.x + self.b * p.y + self.c * p.z.into()).is_zero()
    }

    pub fn intersect(self, other: Self) -> LineIntersection<T> {
        let (x, y, z) = (
            other.c * self.b - self.c * other.b,
            self.c * other.a - other.c * self.a,
            self.a * other.b - other.a * self.b,
        );
        if let Some(p) = Point::try_new(x, y, z) {
            LineIntersection::Point(p)
        } else if x.is_zero() && y.is_zero() {
            LineIntersection::Line(self)
        } else {
            LineIntersection::None
        }
    }

    pub fn closest_point_to<P: Into<Point<T>>>(self, p: P) -> Point<T> {
        let p = p.into();
        let x = self.b * (self.b * p.x - self.a * p.y) - self.a * self.c;
        let y = self.a * (self.a * p.y - self.b * p.x) - self.b * self.c;
        let z = (self.a * self.a + self.b * self.b) * p.z.into();

        let ([x, y], z) = T::try_normalize([x, y], z).expect("z should be normalizable");

        Point { x, y, z }
    }
}

impl<T: ValidCoordinate> PartialEq for Line<T> {
    fn eq(&self, other: &Self) -> bool {
        self.a * other.c == other.a * self.c && self.b * other.c == other.b * self.c
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineIntersection<T: ValidCoordinate> {
    None,
    Point(Point<T>),
    Line(Line<T>),
}

impl<T: ValidCoordinate> LineIntersection<T> {
    pub fn unwrap_none(self) {
        match self {
            Self::None => {}
            Self::Point(_) => panic!("expected None but was Point"),
            Self::Line(_) => panic!("expected None but was Line"),
        }
    }

    pub fn unwrap_point(self) -> Point<T> {
        match self {
            Self::None => panic!("expected Point but was None"),
            Self::Point(p) => p,
            Self::Line(_) => panic!("expected Point but was Line"),
        }
    }

    pub fn unwrap_line(self) -> Line<T> {
        match self {
            Self::None => panic!("expected Line but was None"),
            Self::Point(_) => panic!("expected Line but was Point"),
            Self::Line(l) => l,
        }
    }
}

// TODO: Eq, Ord
#[derive(Debug, Clone, Copy)]
// TODO: Decide whether they should be pub or not
pub struct Segment<T: ValidCoordinate>(pub Point<T>, pub Point<T>);

impl<T: ValidCoordinate> Segment<T> {
    pub fn between<P1: Into<Point<T>>, P2: Into<Point<T>>>(p1: P1, p2: P2) -> Option<Self> {
        let p1 = p1.into();
        let p2 = p2.into();
        if p1 != p2 {
            Some(Self(p1, p2))
        } else {
            None
        }
    }

    pub fn to_line(self) -> Line<T> {
        Line::spanned_by(self.0, self.1)
    }

    pub fn intersect(self, other: Self) -> SegmentIntersection<T> {
        fn order_points_by<T: ValidCoordinate, R: PartialOrd, F: Fn(Point<T>) -> R>(
            p1: Point<T>,
            p2: Point<T>,
            f: F,
        ) -> Option<(Point<T>, Point<T>)> {
            let k1 = f(p1);
            let k2 = f(p2);
            match k1.partial_cmp(&k2) {
                Some(std::cmp::Ordering::Less) => Some((p1, p2)),
                Some(std::cmp::Ordering::Equal) => None,
                Some(std::cmp::Ordering::Greater) => Some((p2, p1)),

                None => None,
            }
        }

        // Check on which sides of self the endpoints of the other segment lie
        match (
            Point::ordering([self.0, self.1, other.0]),
            Point::ordering([self.0, self.1, other.1]),
        ) {
            // The endpoints lie on the same side -> no collision possible
            (Ordering::Counterclockwise, Ordering::Counterclockwise) => SegmentIntersection::None,
            (Ordering::Clockwise, Ordering::Clockwise) => SegmentIntersection::None,
            // All of the points are on the same line -> the intersection might be a segment
            (Ordering::Collinear, Ordering::Collinear) => {
                // Order the points according to one of their coordinates
                if let Some((start1, end1)) = order_points_by(self.0, self.1, |p| p.x()) {
                    let (start2, end2) = order_points_by(other.0, other.1, |p| p.x())
                        .expect("segment must not be degenerate");

                    let start = if start1.x() < start2.x() {
                        start2
                    } else {
                        start1
                    };
                    let end = if end1.x() > end2.x() { end2 } else { end1 };
                    match start.x().partial_cmp(&end.x()) {
                        Some(std::cmp::Ordering::Less) => {
                            SegmentIntersection::Segment(Segment(start, end))
                        }
                        Some(std::cmp::Ordering::Equal) => SegmentIntersection::Point(start),
                        _ => SegmentIntersection::None,
                    }
                } else if let Some((start1, end1)) = order_points_by(self.0, self.1, |p| p.y()) {
                    let (start2, end2) = order_points_by(other.0, other.1, |p| p.y())
                        .expect("segment must not be degenerate");

                    let start = if start1.y() < start2.y() {
                        start2
                    } else {
                        start1
                    };
                    let end = if end1.y() > end2.y() { end2 } else { end1 };

                    match start.y().partial_cmp(&end.y()) {
                        Some(std::cmp::Ordering::Less) => {
                            SegmentIntersection::Segment(Segment(start, end))
                        }
                        Some(std::cmp::Ordering::Equal) => SegmentIntersection::Point(start),
                        _ => SegmentIntersection::None,
                    }
                } else {
                    SegmentIntersection::None
                }
            }

            // The segments may cross at a point
            _ => {
                // Check the ordering of the points from the other segment's perspective
                match (
                    Point::ordering([other.0, other.1, self.0]),
                    Point::ordering([other.0, other.1, self.1]),
                ) {
                    // The endpoints lie on the same side -> no collision possible
                    (Ordering::Counterclockwise, Ordering::Counterclockwise) => {
                        SegmentIntersection::None
                    }
                    (Ordering::Clockwise, Ordering::Clockwise) => SegmentIntersection::None,
                    // This should never happen
                    (Ordering::Collinear, Ordering::Collinear) => {
                        unreachable!("the points are and aren't collinear")
                    }
                    // A collision happens -> use the line collision to get the exact point
                    _ => SegmentIntersection::Point(
                        self.to_line().intersect(other.to_line()).unwrap_point(),
                    ),
                }
            }
        }
    }

    pub fn sq_len(self) -> T::Coordinate {
        let dx = self.1.x() - self.0.x();
        let dy = self.1.y() - self.0.y();
        dx * dx + dy * dy
    }

    pub fn len(self) -> T::Coordinate
    where
        T::Coordinate: Float,
    {
        self.sq_len().get_sqrt()
    }
}

impl<T: ValidCoordinate> PartialEq for Segment<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SegmentIntersection<T: ValidCoordinate> {
    None,
    Point(Point<T>),
    Segment(Segment<T>),
}

impl<T: ValidCoordinate> SegmentIntersection<T> {
    pub fn unwrap_none(self) {
        match self {
            Self::None => {}
            Self::Point(_) => panic!("expected None but was Point"),
            Self::Segment(_) => panic!("expected None but was Segment"),
        }
    }

    pub fn unwrap_point(self) -> Point<T> {
        match self {
            Self::None => panic!("expected Point but was None"),
            Self::Point(p) => p,
            Self::Segment(_) => panic!("expected Point but was Segment"),
        }
    }

    pub fn unwrap_line(self) -> Segment<T> {
        match self {
            Self::None => panic!("expected Segment but was None"),
            Self::Point(_) => panic!("expected Segment but was Point"),
            Self::Segment(l) => l,
        }
    }
}
