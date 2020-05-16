use bcrypt::{DEFAULT_COST, hash, verify};

/// Hashes data and returns as an unsigned 8-bit buffer for TCP communication.
pub fn hash_data(buffer: &[u8]) -> Vec<u8> {
    let hashed = hash(buffer, DEFAULT_COST).unwrap();

    Vec::from(hashed)
}

/// Verifies a hash with a word, and returns the result.
pub fn verify_data(word: &str, hash: &str) -> bool {
    let valid: bool = verify(word, hash).unwrap();

    valid
}
