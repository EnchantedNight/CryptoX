use cryptox::{hash::Algorithm, hash::StreamingHasher, rand, uuid, uuid::Uuid};

fn main() {
    let id = String::from(uuid::v7());
    println!("Id is - {}", id);

    let mut sha = Algorithm::sha256(&id.as_bytes());
    println!("SHA256 is - {}", sha);

    sha = Algorithm::sha512(&id.as_bytes());
    println!("SHA512 is - {}", sha);

    let streamed = StreamingHasher::new(Algorithm::Sha256)
        .update(b"hello ")
        .update(b"world")
        .finalize_hex();
    println!("Streaming SHA256 is - {}", streamed);

    let stable = uuid::v5(Uuid::NAMESPACE_OID, "cryptox/document");
    println!("Stable UUID v5 is - {}", stable);

    let parsed = uuid::stringify(&stable.to_string()).unwrap();
    println!("Parsed back is - {} (matches: {})", parsed, parsed == stable);

    let bytes = rand::bytes(10);
    println!("Bytes are - {:?}", bytes);

    let hex = rand::hex(10);
    println!("Hex is - {}", hex);

    let val = rand::range(0, 10);
    println!("Value is - {}", val);
}
