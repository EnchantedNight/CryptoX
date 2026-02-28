//! # Hash Module
//!
//! Provides functions to generate SHA-256 and SHA-512 hashes.
//! Requires the `hash` feature.

#[cfg(not(feature = "hash"))]
pub fn not_available() {
    compile_error!("Hash module requires 'hash' feature");
}

use sha2::{Digest, Sha256, Sha512};

/// A collection of hashing algorithms.
pub struct Algorithm {}

impl Algorithm {
    /// Generates a SHA256 hash of the input value
    ///
    /// # Examples
    ///
    /// ```
    /// use cryptox::hash::Algorithm;
    /// let hashed: String = Algorithm::sha256(b"content");
    /// println!("Hash: {}", hashed);
    /// ```
    pub fn sha256(val: &[u8]) -> String {
        let hash = Sha256::digest(val);
        format!("{:x}", hash)
    }

    /// Generates a SHA512 hash of the input value
    ///
    /// # Examples
    ///
    /// ```
    /// use cryptox::hash::Algorithm;
    /// let hashed: String = Algorithm::sha512(b"content");
    /// println!("Hash: {}", hashed);
    /// ```
    pub fn sha512(val: &[u8]) -> String {
        let hash = Sha512::digest(val);
        format!("{:x}", hash)
    }
}

#[cfg(test)]
mod tests;
