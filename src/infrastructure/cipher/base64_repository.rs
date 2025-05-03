use base64::{engine::general_purpose, Engine as _};

pub trait Base64Repository: Send + Sync {
    fn encode(&self, bytes: &[u8]) -> String;
    fn decode(&self, b64: &str) -> Result<Vec<u8>, base64::DecodeError>;
}

pub struct Base64RepositoryImpl;

impl Base64Repository for Base64RepositoryImpl {
    fn encode(&self, bytes: &[u8]) -> String {
        general_purpose::STANDARD.encode(bytes)
    }

    fn decode(&self, b64: &str) -> Result<Vec<u8>, base64::DecodeError> {
        general_purpose::STANDARD.decode(b64)
    }
}
