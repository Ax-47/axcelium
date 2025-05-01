use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, PasswordHash,
};
use async_trait::async_trait;

use crate::domain::errors::repositories_errors::RepositoryResult;
pub struct PasswordHasherImpl;

impl PasswordHasherImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
pub trait PasswordHasherRepository: Send + Sync {
    fn hash(&self, password: &str) -> RepositoryResult<String>;
    fn verify(&self, hashed: &str, plain: &str) -> RepositoryResult<bool>;
}
#[async_trait]
impl PasswordHasherRepository for PasswordHasherImpl {
    fn hash(&self, password: &str) -> RepositoryResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        Ok(hash)
    }

    fn verify(&self, hashed: &str, plain: &str) -> RepositoryResult<bool> {
        let parsed = PasswordHash::new(hashed)?;
        Ok(Argon2::default()
            .verify_password(plain.as_bytes(), &parsed)
            .is_ok())
    }
}
