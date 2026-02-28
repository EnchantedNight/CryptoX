//! # UUID Module
//!
//! Provides functions to generate Universally Unique Identifiers (UUIDs) version 4 and 7.
//! Requires the `uuid` feature.

#[cfg(not(feature = "uuid"))]
pub fn not_available() {
    compile_error!("UUID module requires 'uuid' feature");
}

use uuid::Uuid;

/// Generates a Universally Unique Identifier version 4.
///
/// # Examples
///
/// ```
/// use cryptox::uuid;
/// let id = uuid::v4();
/// println!("Id is: {}", id);
/// ```
pub fn v4() -> Uuid {
    Uuid::new_v4()
}

/// Generates a Universally Unique Identifier version 7.
///
/// # Examples
///
/// ```
/// use cryptox::uuid;
/// let id = uuid::v7();
/// println!("Id is: {}", id);
/// ```
pub fn v7() -> Uuid {
    Uuid::now_v7()
}

#[cfg(test)]
mod tests;