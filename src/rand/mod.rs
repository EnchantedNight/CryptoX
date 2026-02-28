//! # Rand Module
//!
//! Provides functions to generate random bytes, hex strings, and numeric ranges.
//! Requires the `rand` feature.

#[cfg(not(feature = "rand"))]
pub fn not_available() {
    compile_error!("Rand module requires 'rand' feature");
}

use rand::prelude::*;
use rand::{TryRng, rngs::SysRng};

/// Generates a vector of random bytes with the specified length.
///
/// # Examples
///
/// ```
/// use cryptox::rand;
/// let random_bytes = rand::bytes(16);
/// assert_eq!(random_bytes.len(), 16);
/// ```
pub fn bytes(len: usize) -> Vec<u8> {
    let mut bytes = vec![0u8; len];
    SysRng {}.try_fill_bytes(&mut bytes).unwrap();
    bytes
}

/// Generates a random hex string with the specified length (in bytes).
/// The resulting string will have a length of `len * 2`.
///
/// # Examples
///
/// ```
/// use cryptox::rand;
/// let random_hex = rand::hex(8);
/// assert_eq!(random_hex.len(), 16);
/// ```
pub fn hex(len: usize) -> String {
    let mut bytes = vec![0u8; len];
    SysRng {}.try_fill_bytes(&mut bytes).unwrap();
    hex::encode(&bytes)
}

/// Returns a random integer between `begin` (exclusive) and `end` (inclusive).
///
/// # Examples
///
/// ```
/// use cryptox::rand;
/// let value = rand::range(0, 10);
/// assert!(value > 0 && value <= 10);
/// ```
pub fn range(begin: i64, end: i64) -> i64 {
    let mut rng = rand::rng();
    let mut vec: Vec<i64> = (begin + 1..=end).collect();
    vec.shuffle(&mut rng);
    *vec.choose(&mut rng).unwrap()
}

#[cfg(test)]
mod tests;
