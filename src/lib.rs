//! ### CryptoX library provider centralized cryptography utils
//!
//! #### Features:
//! 
//! - #### uuid - `module cryptox::uuid`
//! 
//! - #### bcrypt - `module cryptox::bc`
//! 
//! - #### hash - `module cryptox::hash`

/// ### UUID module providers function to generate universal unique identifier version 4 and 7
#[cfg(feature = "uuid")]
pub mod uuid;
/// ### Bcrypt module provides functions to hash and verify passwords using bcrypt algorithm
#[cfg(feature = "bcrypt")]
pub mod bc;
/// ### Hash module provides functions to generate SHA256 hashes
#[cfg(feature = "hash")]
pub mod hash;

#[cfg(feature = "uuid")]
pub use uuid::*;
#[cfg(feature = "bcrypt")]
pub use bc::*;
#[cfg(feature = "hash")]
pub use hash::*;