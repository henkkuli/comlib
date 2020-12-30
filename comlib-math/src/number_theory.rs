use crate::Integer;

/// Computes the greatest common divisor of the given numbers.
///
/// The greatest common divisor of `a` and `b` is the largest integer which divides both `a` and `b`.
/// The function implements [Euclidean algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm).
pub fn gcd<I: Integer>(a: I, b: I) -> I {
    if b.is_zero() {
        a
    } else {
        gcd(b, a % b)
    }
}
