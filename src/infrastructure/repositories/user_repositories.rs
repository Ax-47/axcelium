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
use sqlx::MySqlPool;
use std::sync::Arc;
pub struct UserRepositoryImpl {
    pub cache: Arc<RedisClient>,
    pub database: Arc<MySqlPool>,
}

impl UserRepositoryImpl {
    pub fn new(cache: Arc<RedisClient>, database: Arc<MySqlPool>) -> Self {
        Self { cache, database }
    }
}
#[async_trait]
pub trait UserRepository: Send + Sync {
    fn check_rule_name(&self, rule_name: String) -> bool;
    fn hash_password(&self, password: String) -> RepositoryResult<String>;
    fn verify_password(&self, stored_hash: String, password: String) -> RepositoryResult<bool>;
    async fn create(&self, user: CreateUser) -> RepositoryResult<u64>;
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
        if self.check_rule_name(user.username.clone()) {
            return Err(RepositoryError {
                message: "username is not validate".to_string(),
            });
        }
        let pool = &*self.database;
        let hashed_password = self.hash_password(user.password.to_string())?;
        let query = "INSERT INTO users (username, password, role) VALUES (?, ?, ?)";
        let result = sqlx::query(query)
            .bind(&user.username)
            .bind(&hashed_password)
            .bind(&user.role)
            .execute(pool)
            .await?;
        Ok(result.last_insert_id())
    }
}
