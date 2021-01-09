use std::iter::once;

use crate::Integer;
use comlib_common::MiniMap;
use rand::{thread_rng, Rng};

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

/// Raises base to given exponent in the given modulus.
///
/// Note that it's up to the caller to ensure that the type can store (modulus-1)^2. If this is not the case, it is
/// undefined what this function returns.
pub fn mod_pow<I: Integer>(base: I, exponent: I, modulus: I) -> I {
    if exponent.is_zero() {
        I::one() % modulus
    } else if exponent.is_one() {
        base % modulus
    } else if (exponent % I::from_int(2)).is_zero() {
        let p = mod_pow(base, exponent / I::from_int(2), modulus);
        (p * p) % modulus
    } else {
        (base * mod_pow(base, exponent - I::from_int(1), modulus)) % modulus
    }
}

/// Checks whether a given number is a prime.
///
/// Implements deterministic [Miller-Rabin primality test] for all 64-bit integers.
///
/// [Miller-Rabin primality test]: https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test
pub fn is_prime(candidate: u64) -> bool {
    // 2 is the only even prime
    if candidate <= 2 || candidate % 2 == 0 {
        return candidate == 2;
    }

    // Write candidate as 2^r * d + 1
    let r = (candidate - 1).trailing_zeros();
    let d = candidate / 2u64.pow(r);
    debug_assert!(d % 2 == 1, "d has to be odd");
    debug_assert_eq!(2u64.pow(r) * d + 1, candidate);

    // Bases which allow testing all 64-bit numbers
    // https://miller-rabin.appspot.com/
    const BASES: [u64; 7] = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];

    'witness_loop: for &base in BASES.iter() {
        // We need to reduce the base to modulo candidate
        let base = base % candidate;
        // If the base is a multiple of the candidate, it must be that the candidate is a prime.
        if base == 0 {
            return true;
        }

        // Compute base^d mod candidate
        let x = mod_pow(base as u128, d as u128, candidate as u128) as u64;

        if x == 1 || x == candidate - 1 {
            // Possibly prime, but might be that base is just a strong liar
            continue;
        } else {
            // Repeatedly square x to find out whether it is a square root of 1
            let mut x = x;
            for _ in 0..r - 1 {
                x = ((x as u128 * x as u128) % candidate as u128) as u64;
                if x == candidate - 1 {
                    // Possibly prime
                    continue 'witness_loop;
                }
            }
        }

        // Definitely a composite number
        return false;
    }

    // Tested all necessary witnesses. It must be that the candidate is a prime
    true
}

/// Factorizes the given integer into its prime factors.
///
/// Implements [Pollard's rho algorithm] to find the factorization.
///
/// # Time complexity
/// The expected time-complexity is O(n^(1/4)).
///
/// [Pollard's rho algorithm]: https://en.wikipedia.org/wiki/Pollard%27s_rho_algorithm
pub fn factorize(n: u64) -> Vec<(u64, usize)> {
    let mut factors = MiniMap::new();
    let mut n = n;
    // Take all trivial twos
    while n % 2 == 0 {
        *factors.entry(2).or_insert(0) += 1;
        n /= 2;
    }

    fn factorize(n: u64, factors: &mut MiniMap<u64, usize>) {
        if n == 1 {
            // Do nothing
        } else if is_prime(n) {
            // The only factor of a prime is itself
            *factors.entry(n).or_insert(0) += 1;
        } else {
            // Use the Pollard's rho algorithm with polynomial (x^2 + c) starting at a random x and using random c.
            loop {
                let mut x = thread_rng().gen_range(1, n) as u128;
                let c = thread_rng().gen_range(1, n) as u128;
                let mut y = x;
                let n = n as u128;

                loop {
                    x = (x * x + c) % n;
                    y = (y * y + c) % n;
                    y = (y * y + c) % n;
                    let d = gcd(x.max(y) - x.min(y), n);
                    if d != 1 {
                        if d == n {
                            // Failed :E
                            // -> try with different x and c
                            break;
                        } else {
                            // d is a factor
                            factorize(d as u64, factors);
                            factorize((n / d) as u64, factors);
                            return;
                        }
                    }
                }
            }
        }
    }

    if n > 1 {
        factorize(n, &mut factors);
    }

    factors.into_inner()
}

/// Sieve of Eratosthenes.
///
/// Sieve of Eratosthenes can be quickly used to determine whether a number is a prime and to find out its prime
/// factorization.
///
/// # Time complexity
/// The construction of the sieve takes O(n log log n) time. After this checking whether a number is a prime takes O(1)
/// time and finding the factorization takes O(log n) time.
///
/// # Implementation details
/// Internally the sieve stores for each index the largest prime which divides the corresponding value. For 0 and 1 the
/// sieve stores 0 as neither is divisible by any prime.
pub struct PrimeSieve(Vec<u64>);

impl PrimeSieve {
    /// Constructs a new [`PrimeSieve`].
    ///
    /// Takes O(n log log n) time.
    pub fn new(n: u64) -> Self {
        let mut sieve = vec![0; n as usize + 1];

        for i in once(2).chain((3..=n).step_by(2)) {
            if sieve[i as usize] == 0 {
                // Found a prime, hence we need to mark all higher multiples as non-primes
                for j in (i..=n).step_by(i as usize).skip(0) {
                    sieve[j as usize] = i;
                }
            }
        }

        Self(sieve)
    }

    /// Checks whether the given number is a prime.
    pub fn is_prime(&self, n: u64) -> bool {
        if n < 2 {
            false
        } else {
            self.0[n as usize] == n
        }
    }

    /// Factorizes the given number into its prime factors.
    ///
    /// Returns the factors as pairs indicating the prime and the number of times its present in the factorization.
    /// The factorization is ordered in increasing order by the prime.
    pub fn factorize(&self, n: u64) -> Vec<(u64, usize)> {
        let mut factors = MiniMap::new();
        let mut n = n;

        while n > 1 {
            *factors.entry(self.0[n as usize]).or_insert(0) += 1;
            n /= self.0[n as usize];
        }

        factors.into_inner()
    }

    /// Turns the sieve into raw vector telling the largest prime divisor for each index.
    pub fn into_inner(self) -> Vec<u64> {
        self.0
    }
}
