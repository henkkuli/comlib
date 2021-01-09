//! # Comlib String Utilities
//! This library contains some commonly used string algorithms.
//!
//! ## Content
//! - [Rolling hash](RollingHash)
//!
//! ## Still missing
//! - Z algorithm
//! - Automata
//! - Pattern matching

#![warn(missing_docs)]

mod rolling_hash;
pub use rolling_hash::RollingHash;
