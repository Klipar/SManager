use rand::RngCore;
use base64::{engine::general_purpose, Engine as _};

use crate::server::get_hash::get_hash;

pub fn generate_token() -> (String, String) {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);

    let token = general_purpose::STANDARD.encode(&bytes);

    let token_hash = get_hash(&token);

    (token, token_hash)
}