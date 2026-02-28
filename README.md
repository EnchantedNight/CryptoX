# CryptoX

`cryptox` is a library providing centralized cryptography utility functions for Rust.

## Features

- **`uuid`**: Provides functions to generate Universally Unique Identifiers (UUIDs) version 4 and 7.
- **`hash`**: Provides functions to generate SHA256 and SHA512 hashes.
- **`rand`**: Provides functions to generate random bytes, hex strings, and number ranges.
- **`full`**: Enables all features.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cryptox = { version = "0.1.0", features = ["full"] }
```

## Quick Start

```rust
use cryptox::{Algorithm, uuid, rand};

fn main() {
    // Generate a SHA256 hash
    let hashed = Algorithm::sha256(b"content");
    println!("Hash is - {}", hashed);

    // Generate a UUID v4
    let id = uuid::v4();
    println!("Id is - {}", id);

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
