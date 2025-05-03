use crate::domain::errors::repositories_errors::RepositoryResult;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use base64::{engine::general_purpose, Engine as _};

use async_trait::async_trait;

pub struct AesGcmCipherImpl {
    cipher: Aes256Gcm,
}

impl AesGcmCipherImpl {
    pub fn new(key_bytes: &[u8]) -> Self {
        let key = Key::<Aes256Gcm>::from_slice(key_bytes);
        let cipher = Aes256Gcm::new(&key);
        Self { cipher }
    }
}

#[async_trait]
pub trait AesGcmCipherRepository: Send + Sync {
    async fn encrypt(&self, plaintext: &[u8]) -> RepositoryResult<(String, String)>; // base64(nonce), base64(ciphertext)
    async fn decrypt(&self, nonce_b64: &str, ciphertext_b64: &str) -> RepositoryResult<String>;
}

#[async_trait]
impl AesGcmCipherRepository for AesGcmCipherImpl {
    async fn encrypt(&self, plaintext: &[u8]) -> RepositoryResult<(String, String)> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = self.cipher.encrypt(&nonce, plaintext)?;

        let nonce_b64 = general_purpose::STANDARD.encode(&nonce);
        let ciphertext_b64 = general_purpose::STANDARD.encode(&ciphertext);

        Ok((nonce_b64, ciphertext_b64))
    }

    async fn decrypt(&self, nonce_b64: &str, ciphertext_b64: &str) -> RepositoryResult<String> {
        let nonce_bytes = general_purpose::STANDARD.decode(nonce_b64)?;
        let ciphertext_bytes = general_purpose::STANDARD.decode(ciphertext_b64)?;
        let nonce = Nonce::from_slice(&nonce_bytes);
        let plaintext = self.cipher.decrypt(nonce, ciphertext_bytes.as_ref())?;
        let result =
            String::from_utf8(plaintext).map_err(|_| aes_gcm::Error::from(aes_gcm::aead::Error))?;

        Ok(result)
    }
}
