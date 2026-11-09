# CryptoX

`cryptox` is a library providing centralized cryptography utility functions for Rust.

## Features

- **`uuid`**: Provides functions to generate Universally Unique Identifiers (UUIDs) version 4, 5 and 7, parse them back from strings (`stringify`), and access the `Uuid` type.
- **`hash`**: Provides one-shot SHA-256/SHA-512 hashes via `Algorithm`, and incremental hashing via `StreamingHasher` (pick an algorithm with an `Algorithm` enum, chainable `update`, `finalize_hex` / `finalize_bytes`).
- **`rand`**: Provides functions to generate random bytes, hex strings, and number ranges.
- **`full`**: Enables all features.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cryptox = { version = "1.1.0", features = ["full"] }
```

## Quick Start

```rust
use cryptox::{Algorithm, uuid::{self, Uuid}, hash::StreamingHasher};

fn main() {
    // Generate a SHA256 hash
    let hashed = Algorithm::sha256(b"content");
    println!("Hash is - {}", hashed);

    // Hash data incrementally as it arrives
    let streamed = StreamingHasher::new(Algorithm::Sha256).update(b"cont").update(b"ent").finalize_hex();
    println!("Streaming hash is - {}", streamed);

    // Generate a UUID v4
    let id = uuid::v4();
    println!("Id is - {}", id);

    // Generate a deterministic UUID v5 (same namespace + name, same UUID)
    let stable = uuid::v5(Uuid::NAMESPACE_OID, "stable-thing");
    println!("Stable Id is - {}", stable);

    // Generate a random hex string
    let hex = rand::hex(16);
    println!("Hex is - {}", hex);
}
```

## Modules

- [`uuid`](https://docs.rs/cryptox/latest/cryptox/uuid/)
- [`hash`](https://docs.rs/cryptox/latest/cryptox/hash/)
- [`rand`](https://docs.rs/cryptox/latest/cryptox/rand/)

## License

MIT
