use bcrypt::{hash as sign, verify as unsign};

pub enum Cost {
    DEFAULT = 12,
    MIN = 4,
    MAX = 31
}

pub fn hash(val: &str, cost: Cost) -> String {
    sign(val, cost as u32).unwrap()
}

pub fn verify(val: &str, hashed: &str) -> bool {
    unsign(val, hashed).unwrap()
}