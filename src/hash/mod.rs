//! # Hash Module
//!
//! Provides functions to generate SHA256 and SHA512 hashes, both one-shot via
//! [`Algorithm`] and incrementally via [`StreamingHasher`].
//! Requires the `hash` feature.
//!
//! [`Algorithm`] covers the common "hash this value once" case. When the data
//! arrives in pieces (or you want to hash while streaming), pick an algorithm
//! and hand it to [`StreamingHasher::new`].

use sha2::{Digest, Sha256, Sha512};

/// The hashing algorithms of this crate. Selects the engine for one-shot
/// hashing and for [`StreamingHasher`].
///
/// [`StreamingHasher`]: StreamingHasher
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Algorithm {
    Sha256,
    Sha512,
}

/// Incremental hasher: pick an algorithm once, feed it pieces, extract the
/// digest whenever you want. Digest extraction does not consume the state, so
/// the hasher can keep accepting data afterwards.
pub struct StreamingHasher {
    state: AlgorithmInner,
}

enum AlgorithmInner {
    Sha256(Sha256),
    Sha512(Sha512),
}

impl StreamingHasher {
    /// Starts a new empty hasher for the given algorithm.
    ///
    /// # Examples
    ///
    /// ```
    /// use cryptox::hash::{Algorithm, StreamingHasher};
    ///
    /// let mut hasher = StreamingHasher::new(Algorithm::Sha256);
    /// hasher.update(b"hello ");
    /// hasher.update(b"world");
    /// let digest = hasher.finalize_hex();
    /// assert_eq!(
    ///     digest,
    ///     "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
    /// );
    /// ```
    pub fn new(algorithm: Algorithm) -> Self {
        let state = match algorithm {
            Algorithm::Sha256 => AlgorithmInner::Sha256(Sha256::new()),
            Algorithm::Sha512 => AlgorithmInner::Sha512(Sha512::new()),
        };
        Self { state }
    }

    /// Feeds more data into the hash state. Chainable.
    ///
    /// # Examples
    ///
    /// ```
    /// use cryptox::hash::{Algorithm, StreamingHasher};
    /// let digest = StreamingHasher::new(Algorithm::Sha256)
    ///     .update(b"hello ")
    ///     .update(b"world")
    ///     .finalize_hex();
    /// ```
    pub fn update(&mut self, data: &[u8]) -> &mut Self {
        match &mut self.state {
            AlgorithmInner::Sha256(inner) => inner.update(data),
            AlgorithmInner::Sha512(inner) => inner.update(data),
        }
        self
    }

    /// Produces the final digest as a lowercase hex string.
    ///
    /// Does not consume the state — you can keep updating afterwards.
    ///
    /// # Examples
    ///
    /// ```
    /// use cryptox::hash::{Algorithm, StreamingHasher};
    /// let digest = StreamingHasher::new(Algorithm::Sha256).update(b"hi").finalize_hex();
    /// assert_eq!(digest.len(), 64);
    /// ```
    pub fn finalize_hex(&self) -> String {
        match &self.state {
            AlgorithmInner::Sha256(inner) => to_hex(&inner.clone().finalize()),
            AlgorithmInner::Sha512(inner) => to_hex(&inner.clone().finalize()),
        }
    }

    /// Produces the final digest as raw bytes.
    ///
    /// Does not consume the state — you can keep updating afterwards.
    ///
    /// # Examples
    ///
    /// ```
    /// use cryptox::hash::{Algorithm, StreamingHasher};
    /// let bytes = StreamingHasher::new(Algorithm::Sha512).update(b"hi").finalize_bytes();
    /// assert_eq!(bytes.len(), 64);
    /// ```
    pub fn finalize_bytes(&self) -> Vec<u8> {
        match &self.state {
            AlgorithmInner::Sha256(inner) => inner.clone().finalize().to_vec(),
            AlgorithmInner::Sha512(inner) => inner.clone().finalize().to_vec(),
        }
    }
}

fn to_hex(digest: &[u8]) -> String {
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

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
