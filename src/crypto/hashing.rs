// src/crypto/hashing.rs

use sha2::{Sha256, Digest};

/// 32-byte hash
pub type Hash = [u8; 32];

pub fn hash(data: &[u8]) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}
