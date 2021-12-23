use comlib_geometry::{Line, Point, Segment, SegmentIntersection};

// TODO: Check vector operations

#[test]
fn test_is_on_line() {
    let l = Line::spanned_by((1, -2), (3, 3));
    assert!(l.contains((1, -2)));
    assert!(l.contains((3, 3)));
    assert!(l.contains((5, 8)));
    assert!(!l.contains((0, 0)));
    assert!(!l.contains((-1, 2)));
    assert!(!l.contains((2, 0)));
    assert!(!l.contains((2, 1)));
}

#[test]
fn test_point_equality() {
    assert_eq!(Point::from((1, 3)), Point::from((1, 3)));
    assert_ne!(Point::from((1, 3)), Point::from((3, 1)));

    assert_eq!(Point::from((1, 3)), Point::try_new(2, 6, 2).unwrap());
    assert_eq!(
        Point::try_new(3, 9, 3).unwrap(),
        Point::try_new(2, 6, 2).unwrap()
    );
}

#[test]
fn test_line_intersect() {
    let l1 = Line::spanned_by((1, -2), (3, 3));
    let l2 = Line::spanned_by((1, 1), (2, -2));
    let p1 = l1.intersect(l2).unwrap_point();
    let p2 = l2.intersect(l1).unwrap_point();
    assert_eq!(p1, p2);
    let p1 = p1.to_f64_pair();
    assert_eq!(p1, (17. / 11., -7. / 11.));

    let l1 = Line::spanned_by((1, 1), (3, 5));
    let l2 = Line::spanned_by((2, -2), (3, 0));
    assert_ne!(l1, l2);
    l1.intersect(l2).unwrap_none();
    let l2 = Line::spanned_by((-2, -5), (-1, -3));
    assert_eq!(l1, l2);
    l1.intersect(l2).unwrap_line();
}

#[test]
fn test_line_closest_point_to() {
    let l1 = Line::spanned_by((1, 3), (2, 1));
    assert_eq!(l1.closest_point_to((0, 0)), Point::from((2, 1)));
    assert_eq!(l1.closest_point_to((7, 1)), Point::from((3, -1)));
    assert_eq!(l1.closest_point_to((1, 3)), Point::from((1, 3)));

    // Horizontal line
    let l2 = Line::spanned_by((1, 5), (2, 5));
    assert_eq!(l2.closest_point_to((1, 5)), Point::from((1, 5)));
    assert_eq!(l2.closest_point_to((-10, 5)), Point::from((-10, 5)));
    assert_eq!(l2.closest_point_to((7, 8)), Point::from((7, 5)));

    // Vertical line
    let l3 = Line::spanned_by((3, 1), Point::from((3, 4)));
    assert_eq!(l3.closest_point_to((0, 0)), Point::from((3, 0)));
    assert_eq!(l3.closest_point_to((6, 3)), Point::from((3, 3)));
    assert_eq!(l3.closest_point_to((-123, 45)), Point::from((3, 45)));
}

#[test]
fn test_segment_intersection() {
    // -2 -5 7 -5 -3 -5 -4 -5
    assert_eq!(
        Segment::between((-2, -5), (7, -5))
            .unwrap()
            .intersect(Segment::between((-3, -5), (-4, -5)).unwrap()),
        SegmentIntersection::None
    );

    assert_eq!(
        Segment::between((-2, -5), (7, -5))
            .unwrap()
            .intersect(Segment::between((-3, -5), (-2, -5)).unwrap()),
        SegmentIntersection::Point(Point::from((-2, -5)))
    );

    assert_eq!(
        Segment::between((-2, -5), (7, -5))
            .unwrap()
            .intersect(Segment::between((-3, -5), (-1, -5)).unwrap()),
        SegmentIntersection::Segment(Segment::between((-1, -5), (-2, -5)).unwrap())
    );

    // -4 -4 -4 -9 -4 1 -4 -2
    assert_eq!(
        Segment::between((-4, -4), (-4, -9))
            .unwrap()
            .intersect(Segment::between((-4, 1), (-4, -2)).unwrap()),
        SegmentIntersection::None
    );
}

#[test]
fn test_segment_equality() {
    assert_eq!(
        Segment::between((0, 1), (0, 2)).unwrap(),
        Segment::between((0, 1), (0, 2)).unwrap()
    );
    assert_eq!(
        Segment::between((0, 1), (0, 2)).unwrap(),
        Segment::between((0, 2), (0, 1)).unwrap()
    );
}

// #[test]
// fn test_circle_from_center_and_radius() {
//     for z in 1..10 {
//         for x in -10..=10 {
//             for y in -10..=10 {
//                 for r in 0..10 {
//                     let center = Point::new(x, y, z).unwrap();
//                     let circle = Circle::from_center_and_radius(center, r);
//                     println!("{:?}: {:?} / {}", circle, center, r);
//                     assert_eq!(circle.center(), center);
//                     assert_eq!(circle.radius2(), r * r);
//                 }
//             }
//         }
//     }
// }

// #[test]
// fn test_circle_intersection_line() {
//     // Two intersections on y-axis
//     assert_eq!(
//         Circle::from_center_and_radius((-3, 0), 4)
//             .intersection_line(Circle::from_center_and_radius((3, 0), 4)),
//         Some(Line::between((0, -1), (0, 1)))
//     );

//     // Intersect at origin
//     assert_eq!(
//         Circle::from_center_and_radius((-3, 0), 3)
//             .intersection_line(Circle::from_center_and_radius((3, 0), 3)),
//         Some(Line::between((0, -1), (0, 1)))
//     );

//     // No intersection
//     assert_eq!(
//         Circle::from_center_and_radius((-3, 0), 2)
//             .intersection_line(Circle::from_center_and_radius((3, 0), 2)),
//         None
//     );
// }
