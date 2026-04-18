use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
use sha2::{Digest, Sha256};
use thiserror::Error;

const VERSION: &str = "v1";
const NONCE_LEN: usize = 12;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("invalid encryption key")]
    InvalidKey,
    #[error("encryption failed")]
    Encrypt,
    #[error("invalid encrypted secret format")]
    InvalidFormat,
    #[error("decryption failed")]
    Decrypt,
    #[error("random source unavailable")]
    Random,
}

pub fn encrypt(secret_key: &str, plaintext: &str) -> Result<String, CryptoError> {
    let cipher = cipher(secret_key)?;
    let nonce_bytes = random_nonce()?;
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce_bytes), plaintext.as_bytes())
        .map_err(|_| CryptoError::Encrypt)?;

    Ok(format!(
        "{VERSION}:{}:{}",
        STANDARD_NO_PAD.encode(nonce_bytes),
        STANDARD_NO_PAD.encode(ciphertext)
    ))
}

#[allow(dead_code)]
pub fn decrypt(secret_key: &str, stored_value: &str) -> Result<String, CryptoError> {
    let mut parts = stored_value.split(':');
    let version = parts.next().ok_or(CryptoError::InvalidFormat)?;
    let nonce = parts.next().ok_or(CryptoError::InvalidFormat)?;
    let ciphertext = parts.next().ok_or(CryptoError::InvalidFormat)?;

    if version != VERSION || parts.next().is_some() {
        return Err(CryptoError::InvalidFormat);
    }

    let nonce = STANDARD_NO_PAD
        .decode(nonce)
        .map_err(|_| CryptoError::InvalidFormat)?;
    if nonce.len() != NONCE_LEN {
        return Err(CryptoError::InvalidFormat);
    }

    let ciphertext = STANDARD_NO_PAD
        .decode(ciphertext)
        .map_err(|_| CryptoError::InvalidFormat)?;

    let cipher = cipher(secret_key)?;
    let plaintext = cipher
        .decrypt(Nonce::from_slice(&nonce), ciphertext.as_ref())
        .map_err(|_| CryptoError::Decrypt)?;

    String::from_utf8(plaintext).map_err(|_| CryptoError::Decrypt)
}

fn cipher(secret_key: &str) -> Result<Aes256Gcm, CryptoError> {
    let key = Sha256::digest(secret_key.as_bytes());
    Aes256Gcm::new_from_slice(&key).map_err(|_| CryptoError::InvalidKey)
}

fn random_nonce() -> Result<[u8; NONCE_LEN], CryptoError> {
    let mut nonce = [0_u8; NONCE_LEN];
    getrandom::getrandom(&mut nonce).map_err(|_| CryptoError::Random)?;
    Ok(nonce)
}
