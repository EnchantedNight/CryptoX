//! # UUID Module
//!
//! Provides functions to generate Universally Unique Identifiers (UUIDs)
//! version 4, 5 and 7, parse them from strings, and access the [`Uuid`] type.
//! Requires the `uuid` feature.
//!
//! The `stringify` function wraps [`Uuid::from_str`] to turn a string back
//! into a [`Uuid`].

pub use uuid::Uuid;

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

/// Generates a Universally Unique Identifier version 5.
///
/// Version 5 is deterministic: the same namespace and name always map to the
/// same UUID. Useful for stable identities derived from content, such as
/// vector-database point IDs.
///
/// # Examples
///
/// ```
/// use cryptox::uuid::{self, Uuid};
/// let a = uuid::v5(Uuid::NAMESPACE_OID, "stable-thing");
/// let b = uuid::v5(Uuid::NAMESPACE_OID, "stable-thing");
/// assert_eq!(a, b);
/// ```
pub fn v5(ns: Uuid, name: &str) -> Uuid {
    Uuid::new_v5(&ns, name.as_bytes())
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

/// Parses a UUID string into a [`Uuid`].
///
/// Wraps [`Uuid::from_str`] (which accepts both the standard hyphenated form
/// and the bare hex form).
///
/// # Errors
///
/// Returns the underlying [`uuid::Error`] when the string is not a valid UUID.
///
/// # Examples
///
/// ```
/// use cryptox::uuid::{self, Uuid};
/// let id = uuid::v4();
/// let parsed = uuid::stringify(&id.to_string()).unwrap();
/// assert_eq!(id, parsed);
/// ```
pub fn stringify(id: &str) -> Result<Uuid, uuid::Error> {
    id.parse()
}

#[cfg(test)]
mod tests;
