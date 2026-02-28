use cryptox::{uuid, hash::Algorithm, rand};

fn main() {
    let id = String::from(uuid::v7());
    println!("Id is - {}", id);

    let mut sha = Algorithm::sha256(&id.as_bytes());
    println!("SHA256 is - {}", sha);

    sha = Algorithm::sha512(&id.as_bytes());
    println!("SHA512 is - {}", sha);

    let bytes = rand::bytes(10);
    println!("Bytes are - {:?}", bytes);

    let hex = rand::hex(10);
    println!("Hex is - {}", hex);

    let val = rand::range(0, 10);
    println!("Value is - {}", val);
}