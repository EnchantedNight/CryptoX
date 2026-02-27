use sha2::{Digest, Sha256};

pub fn sha256(val: &'static [u8]) -> String {
    let hash = Sha256::digest(val);
    format!("{:x}", hash)
}