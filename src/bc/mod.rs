//! ### Bcrypt module provides functions to hash and verify passwords using bcrypt algorithm

#[cfg(not(feature = "bcrypt"))]
pub fn not_available() {
    compile_error!("Bcrypt module requires 'bcrypt' feature");
}

use bcrypt::{hash as sign, verify as unsign};

/// Enumeration of bcrypt costs (complexity)
pub enum Cost {
    /// Default cost (12)
    Default = 12,
    /// Minimum allowed cost (4)
    Min = 4,
    /// Maximum allowed cost (31)
    Max = 31
}

/// Hashes a value using bcrypt algorithm with the specified cost
/// 
/// # Examples
/// 
/// ```
/// use cryptox::bc::{hash, Cost};
/// let hashed = hash("password", Cost::DEFAULT);
/// ```
pub fn hash(val: &str, cost: Cost) -> String {
    sign(val, cost as u32).unwrap()
}

/// Verifies if a value matches a bcrypt hash
/// 
/// # Examples
/// 
/// ```
/// use cryptox::bc::{hash, verify, Cost};
/// let hashed = hash("password", Cost::DEFAULT);
/// assert!(verify("password", &hashed));
/// ```
pub fn verify(val: &str, hashed: &str) -> bool {
    unsign(val, hashed).unwrap()
}