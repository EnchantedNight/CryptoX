//! ### Rand module provides functions to generate random bytes/hex
#[cfg(not(feature = "rand"))]
pub fn not_available() {
    compile_error!("Rand module requires 'rand' feature");
}

use rand::prelude::*;
use rand::{TryRng, rngs::SysRng};

pub fn bytes(len: usize) -> Vec<u8> {
    let mut bytes = vec![0u8; len];
    SysRng {}.try_fill_bytes(&mut bytes).unwrap();
    bytes
}

pub fn hex(len: usize) -> String {
    let mut bytes = vec![0u8; len];
    SysRng {}.try_fill_bytes(&mut bytes).unwrap();
    hex::encode(&bytes)
}

pub fn range(begin: i64, end: i64) -> i64 {
    let mut rng = rand::rng();
    let mut vec: Vec<i64> = (begin + 1..=end).collect();
    vec.shuffle(&mut rng);
    *vec.choose(&mut rng).unwrap()
}

#[cfg(test)]
mod tests;
