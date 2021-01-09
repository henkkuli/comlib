use comlib_math::{factorize, gcd, is_prime, PrimeSieve};

#[test]
fn test_gcd() {
    assert_eq!(gcd(1, 2), 1);
    assert_eq!(gcd(99, 0), 99);
    assert_eq!(gcd(0, 99), 99);
    assert_eq!(gcd(6, 9), 3);
    assert_eq!(gcd(9, 6), 3);
}

#[test]
fn test_prime_sieve_construction() {
    let sieve = PrimeSieve::new(10);
    assert_eq!(sieve.into_inner(), [0, 0, 2, 3, 2, 5, 3, 7, 2, 3, 5]);
}

#[test]
fn test_prime_sieve_is_prime() {
    let sieve = PrimeSieve::new(10_000);
    assert!(!sieve.is_prime(0));
    assert!(!sieve.is_prime(1));
    assert!(sieve.is_prime(2));
    assert!(sieve.is_prime(3));
    assert!(!sieve.is_prime(4));
    assert!(sieve.is_prime(5));
    assert!(!sieve.is_prime(6));
    assert!(sieve.is_prime(7));
    assert!(!sieve.is_prime(8));
    assert!(!sieve.is_prime(9));
    assert!(!sieve.is_prime(10));
    assert!(sieve.is_prime(11));

    assert!(sieve.is_prime(211));
    assert!(sieve.is_prime(3433));
    assert!(!sieve.is_prime(3435));
    assert!(sieve.is_prime(7717));
    assert!(!sieve.is_prime(7719));
    assert!(!sieve.is_prime(7721));
    assert!(sieve.is_prime(7723));
    assert!(sieve.is_prime(7547));
    assert!(sieve.is_prime(7549));
    assert!(!sieve.is_prime(10_000));
}

#[test]
fn test_prime_sieve_factorization() {
    let sieve = PrimeSieve::new(10_000);
    assert_eq!(sieve.factorize(2), [(2, 1)]);
    assert_eq!(sieve.factorize(4), [(2, 2)]);
    assert_eq!(sieve.factorize(8), [(2, 3)]);
    assert_eq!(sieve.factorize(9), [(3, 2)]);
    assert_eq!(sieve.factorize(21), [(3, 1), (7, 1)]);
    assert_eq!(sieve.factorize(1009), [(1009, 1)]);
    assert_eq!(sieve.factorize(10_000), [(2, 4), (5, 4)]);
}

#[test]
fn test_is_prime() {
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(!is_prime(4));
    assert!(is_prime(5));
    assert!(is_prime(7));
    assert!(!is_prime(21));
    assert!(is_prime(23));
    assert!(is_prime(1_000_000_007));
    assert!(is_prime(1_000_000_009));
}

#[test]
fn test_is_prime_against_sieve() {
    let n = if cfg!(debug_assertions) {
        100_000
    } else {
        10_000_000
    };
    let sieve = PrimeSieve::new(n);
    for i in 0..=n {
        assert_eq!(is_prime(i), sieve.is_prime(i), "Failed {}", i);
    }
}

#[test]
fn test_factorize() {
    assert_eq!(factorize(2), [(2, 1)]);
    assert_eq!(factorize(4), [(2, 2)]);
    assert_eq!(factorize(8), [(2, 3)]);
    assert_eq!(factorize(9), [(3, 2)]);
    assert_eq!(factorize(21), [(3, 1), (7, 1)]);
    assert_eq!(factorize(450775), [(5, 2), (13, 1), (19, 1), (73, 1)]);
}

#[test]
fn test_factorize_against_sieve() {
    let n = if cfg!(debug_assertions) {
        100_000
    } else {
        1_000_000
    };
    let sieve = PrimeSieve::new(n);

    for i in 1..=n {
        assert_eq!(factorize(i), sieve.factorize(i), "Failed {}", i);
    }
}
