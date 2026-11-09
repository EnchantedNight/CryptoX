//! # CryptoX
//!
//! `cryptox` is a library providing centralized cryptography and utility functions.
//! It aims to be simple and easy to use, with features that can be enabled or disabled as needed.
//!
//! ## Features
//!
//! - **`uuid`**: Provides functions to generate Universally Unique Identifiers (UUIDs) version 4, 5 and 7, plus string parsing.
//! - **`hash`**: Provides one-shot SHA-256/SHA-512 via [`Algorithm`] and incremental hashing via [`StreamingHasher`].
//! - **`rand`**: Provides functions to generate random bytes, hex strings, and ranges.
//! - **`full`**: Enables all features.
//!
//! ## Modules
//!
//! Each feature corresponds to a module:
//!
//! - [`uuid`] (requires `uuid` feature)
//! - [`hash`] (requires `hash` feature)
//! - [`rand`] (requires `rand` feature)
//!
//! ## Re-exports
//!
//! For convenience, when a feature is enabled, its contents are re-exported at the crate root.
//!
//! ```rust
//! // Example usage (with 'hash' and 'uuid' features enabled)
//! # #[cfg(all(feature = "hash", feature = "uuid"))]
//! # {
//! use cryptox::{Algorithm, uuid::{self, Uuid}};
//!
//! let hashed = Algorithm::sha256(b"content");
//! let id = uuid::v4();
//! let stable_id = uuid::v5(Uuid::NAMESPACE_OID, "stable-thing");
//! # }
//! ```

#[cfg(feature = "hash")]
pub mod hash;
#[cfg(feature = "rand")]
pub mod rand;
#[cfg(feature = "uuid")]
pub mod uuid;

#[cfg(feature = "hash")]
pub use hash::*;
#[cfg(feature = "rand")]
pub use rand::*;
#[cfg(feature = "uuid")]
pub use uuid::*;
