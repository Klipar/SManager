use std::{env, sync::OnceLock};

use base64::{engine::general_purpose, Engine as _};
use log::error;
use sha2::{Digest, Sha256};

static SALT: OnceLock<String> = OnceLock::new();

pub fn set_salt_env_key(env_key: &str) {
    match env::var(env_key) {
        Ok(salt) => {
            if SALT.set(salt).is_err() {
                error!("Salt is already initialized");
            }
        }
        Err(_) => {
            error!("Environment variable '{}' is not set", env_key);
        }
    }
}

pub fn get_hash(data: &str) -> String {
    let mut hasher = Sha256::new();

    match SALT.get() {
        Some(salt) => hasher.update(salt.as_bytes()),
        None => error!("Salt is not initialized"),
    }

    hasher.update(data.as_bytes());

    let result = hasher.finalize();
    general_purpose::STANDARD.encode(result)
}