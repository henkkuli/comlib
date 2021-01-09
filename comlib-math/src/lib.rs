//! # Comlib Math Utilities
//! This library contains some commonly used math utilities.
//!
//! ## Content
//! - [Greatest common divisor](gcd)
//! - [Modular integers](ModInt)
//! - [Sieve of Eratosthenes](PrimeSieve)
//! - [Primality test](is_prime)
//! - [Factorization](factorize)
//! - [Modular exponentiation](mod_pow)
//! - [Finding next permutation of a list](next_permutation)
//!
//! ## Still missing
//! - Fourier Transform, both number theoretic and complex

#![warn(missing_docs)]

mod modint;
pub use modint::{
    InvertibleModulus, Mod1e9p7, ModInt, Modulus, RuntimeModulus, RuntimePrimeModulus,
};

mod numtraits;
pub use numtraits::Integer;

mod number_theory;
pub use number_theory::{factorize, gcd, is_prime, mod_pow, PrimeSieve};

mod permutations;
pub use permutations::next_permutation;
