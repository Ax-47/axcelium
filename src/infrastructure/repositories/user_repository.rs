use crate::domain::{
    errors::repositories_errors::{RepositoryError, RepositoryResult},
    models::{
        apporg_client_id_models::CleanAppOrgByClientId,
        user_models::{CreateUser, CreatedUser},
    },
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, PasswordHash,
};

use async_trait::async_trait;
use redis::Client as RedisClient;
use scylla::client::session::Session;
use std::sync::Arc;
use uuid::Uuid;
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
    async fn find_user_by_email(
        &self,
        email: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<CreatedUser>;

    async fn find_user_by_username(
        &self,
        username: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<CreatedUser>;
    fn check_rule_name(&self, rule_name: String) -> bool;
    fn hash_password(&self, password: String) -> RepositoryResult<String>;
    fn verify_password(&self, stored_hash: String, password: String) -> RepositoryResult<bool>;
    async fn create(
        &self,
        c_apporg: CleanAppOrgByClientId,
        user: CreateUser,
    ) -> RepositoryResult<u64>;
    async fn send_otp(&self);
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(
        &self,
        c_apporg: CleanAppOrgByClientId,
        user: CreateUser,
    ) -> RepositoryResult<u64> {
        if !self.check_rule_name(user.username.clone()) {
            return Err(RepositoryError::new(
                "username is not validate".to_string(),
                400,
            ));
        }
        let Ok(app_config) = c_apporg.get_config() else {
            return Err(RepositoryError::new(
                "failed to read config".to_string(),
                500,
            ));
        };
        if !app_config.can_allow_email_nullable
            && self
                .find_user_by_email(
                    user.email,
                    c_apporg.application_id,
                    c_apporg.organization_id,
                )
                .await
                .is_ok()
        {
            return Err(RepositoryError::new("this email has used".to_string(), 400));
        }
        if !app_config.is_must_name_unique
            && self
                .find_user_by_username(
                    user.username,
                    c_apporg.application_id,
                    c_apporg.organization_id,
                )
                .await
                .is_ok()
        {
            return Err(RepositoryError::new("this username has used".to_string(), 400));
        }
        let _hashed_password = self.hash_password(user.password)?;
        //TODO create user
        Ok(3)
    }
    async fn find_user_by_email(
        &self,
        email: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<CreatedUser> {
        let query = "SELECT username FROM users_by_email \
                 WHERE email = ? AND application_id = ? AND organization_id = ?";

        let result = self
            .database
            .query_unpaged(query, (email, application_id, organization_id))
            .await
            .map_err(|e| RepositoryError {
                message: format!("DB query failed: {}", e),
                code: 500,
            })?
            .into_rows_result()
            .map_err(|e| RepositoryError {
                message: format!("Failed to parse DB rows: {}", e),
                code: 500,
            })?;

        let row = result
            .first_row::<CreatedUser>()
            .map_err(|e| RepositoryError {
                message: format!("No matching user found or row error: {}", e),
                code: 404,
            })?;
        Ok(row)
    }

    async fn find_user_by_username(
        &self,
        username: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<CreatedUser> {
        let query = "SELECT username FROM users_by_username \
                WHERE username = ? AND application_id = ? AND organization_id = ?";
        let result = self
            .database
            .query_unpaged(query, (username, application_id, organization_id))
            .await
            .map_err(|e| RepositoryError {
                message: format!("DB query failed: {}", e),
                code: 500,
            })?
            .into_rows_result()
            .map_err(|e| RepositoryError {
                message: format!("Failed to parse DB rows: {}", e),
                code: 500,
            })?;

        let row = result
            .first_row::<CreatedUser>()
            .map_err(|e| RepositoryError {
                message: format!("No matching user found or row error: {}", e),
                code: 404,
            })?;
        Ok(row)
    }
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
    async fn send_otp(&self) {}
}
