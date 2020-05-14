use bcrypt::{DEFAULT_COST, hash, verify};
use std::borrow::Cow;

pub fn hash_data(buffer: &[u8]) -> Vec<u8> {
    let hashed = hash(buffer, DEFAULT_COST).unwrap();

    Vec::from(hashed)
}

pub fn verify_data(word: &str, hash: &str) -> bool {
    let valid: bool = verify(word, hash).unwrap();

    valid
}
