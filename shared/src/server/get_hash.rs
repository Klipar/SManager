use base64::{engine::general_purpose, Engine as _};
use sha2::{Sha256, Digest};

pub fn get_hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();

    general_purpose::STANDARD.encode(result)
}