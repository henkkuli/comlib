use crate::{Point, Segment, ValidCoordinate};
use comlib_math::Numeric;

pub struct Polygon<T: ValidCoordinate>(Vec<Point<T>>);

impl<T: ValidCoordinate> Polygon<T> {
    pub fn points(&self) -> impl Iterator<Item = Point<T>> + '_ {
        self.0.iter().copied()
    }

    pub fn segments(&self) -> PolygonSegmentIter<'_, T> {
        PolygonSegmentIter(*self.0.last().expect("polygon cannot be empty"), &self.0)
    }

    /// Computes the signed area of the polygon.
    ///
    /// The sign of the area is positive if the polygon is defined in counter-clockwise order, and negative otherwise.
    pub fn area(&self) -> T::Coordinate {
        let mut area = T::Coordinate::zero();
        for segment in self.segments() {
            area += segment.0.x() * segment.1.y() - segment.1.x() * segment.0.y();
        }
        area / T::Coordinate::from_int(2)
    }
}

impl<T: ValidCoordinate> From<Vec<Point<T>>> for Polygon<T> {
    fn from(points: Vec<Point<T>>) -> Self {
        Self(points)
    }
}

pub struct PolygonSegmentIter<'a, T: ValidCoordinate>(Point<T>, &'a [Point<T>]);

impl<'a, T: ValidCoordinate> Iterator for PolygonSegmentIter<'a, T> {
    type Item = Segment<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let (first, rest) = self.1.split_first()?;
        let segment = Segment::between(self.0, *first).unwrap();
        self.0 = *first;
        self.1 = rest;
        Some(segment)
    }
}

impl<'a, T: ValidCoordinate> DoubleEndedIterator for PolygonSegmentIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.1 {
            [] => None,
            [end] => {
                self.1 = &[];
                Some(Segment::between(self.0, *end).unwrap())
            }
            [.., s1, s2] => {
                self.1 = &self.1[..self.1.len() - 1];
                Some(Segment::between(*s1, *s2).unwrap())
            }
        }
    }
}
