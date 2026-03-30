use rand::RngCore;
use base64::{engine::general_purpose, Engine as _};
use sha2::{Sha256, Digest};

pub fn generate_token() -> (String, String) {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);

    // encode in Base64
    let token = general_purpose::STANDARD.encode(&bytes);

    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    let result = hasher.finalize();

    // encode hash in Base64
    let token_hash = general_purpose::STANDARD.encode(result);

    (token, token_hash)
}