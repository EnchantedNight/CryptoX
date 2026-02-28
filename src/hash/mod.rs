//! ### Hash module provides functions to generate SHA256 and SHA512 hashes

#[cfg(not(feature = "hash"))]
pub fn not_available() {
    compile_error!("Hash module requires 'hash' feature");
}

use sha2::{Digest, Sha256, Sha512};

/// Provides functions to generated SHA256 and SHA512 hash
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
