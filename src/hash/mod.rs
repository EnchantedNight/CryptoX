//! ### Hash module provides functions to generate SHA256 hashes

#[cfg(not(feature = "hash"))]
pub fn not_available() {
    compile_error!("Hash module requires 'hash' feature");
}

use sha2::{Digest, Sha256};

/// Generates a SHA256 hash of the input value
/// 
/// # Examples
/// 
/// ```
/// use cryptox::hash::sha256;
/// let hashed: String = sha256(b"hello world");
/// println!("Hash: {}", hashed);
/// ```
pub fn sha256(val: &[u8]) -> String {
    let hash = Sha256::digest(val);
    format!("{:x}", hash)
}