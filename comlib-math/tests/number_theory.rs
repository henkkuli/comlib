use comlib_math::gcd;

#[test]
fn gcd_works() {
    assert_eq!(gcd(1, 2), 1);
    assert_eq!(gcd(99, 0), 99);
    assert_eq!(gcd(0, 99), 99);
    assert_eq!(gcd(6, 9), 3);
    assert_eq!(gcd(9, 6), 3);
}
