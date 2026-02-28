//! ### CryptoX library provider centralized cryptography utils
//!
//! #### Features:
//! 
//! - #### uuid - `module cryptox::uuid`
//! 
//! - #### hash - `module cryptox::hash`

#[cfg(feature = "uuid")]
pub mod uuid;
#[cfg(feature = "hash")]
pub mod hash;
#[cfg(feature = "rand")]
pub mod rand;

#[cfg(feature = "uuid")]
pub use uuid::*;
#[cfg(feature = "hash")]
pub use hash::*;
#[cfg(feature = "rand")]
pub use rand::*;