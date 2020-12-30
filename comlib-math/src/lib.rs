//! # Comlib Math Utilities
//! This library contains some commonly used math utilities.
//!
//! ## Content
//! - [Greatest common divisor](gcd)
//! - [Modular integers](ModInt)
//!
//! ## Still missing
//! - Fourier Transform, both number theoretic and complex
//! - Primality test and factorization

mod modint;
pub use modint::{Mod1e9p7, ModInt, Modulus, RuntimePrimeModulus};

mod numtraits;
pub use numtraits::Integer;

mod number_theory;
pub use number_theory::gcd;
