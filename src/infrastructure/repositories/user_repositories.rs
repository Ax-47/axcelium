use crate::domain::{
    errors::repositories_errors::{RepositoryError, RepositoryResult},
    models::user_models::CreateUser,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, PasswordHash,
};

use async_trait::async_trait;
use redis::Client as RedisClient;
use scylla::client::session::Session;
use std::sync::Arc;
pub struct UserRepositoryImpl {
    pub cache: Arc<RedisClient>,
    pub database: Arc<Session>,
}

impl UserRepositoryImpl {
    pub fn new(cache: Arc<RedisClient>, database: Arc<Session>) -> Self {
        Self { cache, database }
    }
}
#[async_trait]
pub trait UserRepository: Send + Sync {
    fn check_rule_name(&self, rule_name: String) -> bool;
    fn hash_password(&self, password: String) -> RepositoryResult<String>;
    fn verify_password(&self, stored_hash: String, password: String) -> RepositoryResult<bool>;
    async fn create(&self, user: CreateUser) -> RepositoryResult<u64>;
    async fn send_otp(&self);
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    fn check_rule_name(&self, rule_name: String) -> bool {
        rule_name.len() >= 2 && rule_name.len() <= 50
    }
    fn hash_password(&self, password: String) -> RepositoryResult<String> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        Ok(password_hash)
    }

    fn verify_password(&self, stored_hash: String, password: String) -> RepositoryResult<bool> {
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&stored_hash)?;
        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
    async fn create(&self, user: CreateUser) -> RepositoryResult<u64> {
        if !self.check_rule_name(user.username.clone()) {
            return Err(RepositoryError {
                message: "username is not validate".to_string(),
                code:400,
            });
        }
        Ok(3)
    }
    async fn send_otp(&self){

    }
}
