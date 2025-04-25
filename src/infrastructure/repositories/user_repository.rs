use crate::domain::{
    errors::repositories_errors::{RepositoryError, RepositoryResult},
    models::{
        app_config::AppConfig,
        apporg_client_id_models::CleanAppOrgByClientId,
        user_models::{CreateUser, CreatedUser, User, UserOrganization},
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
    async fn create(
        &self,
        c_apporg: CleanAppOrgByClientId,
        user: CreateUser,
    ) -> RepositoryResult<Uuid>;

    async fn create_user(&self, user: User, u_org: UserOrganization) -> RepositoryResult<()>;
    async fn insert_into_user(&self, user: &User) -> RepositoryResult<()>;
    async fn insert_into_user_by_email(&self, user: &User) -> RepositoryResult<()>;
    async fn insert_into_user_by_username(&self, user: &User) -> RepositoryResult<()>;
    async fn insert_into_user_organizations(
        &self,
        user_org: &UserOrganization,
    ) -> RepositoryResult<()>;
    async fn insert_into_user_organizations_by_user(
        &self,
        user_org: &UserOrganization,
    ) -> RepositoryResult<()>;
    async fn find_user_by_email(
        &self,
        email: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<Option<CreatedUser>>;

    async fn find_user_by_username(
        &self,
        username: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<Option<CreatedUser>>;
    fn check_rule_name(&self, rule_name: String) -> RepositoryResult<()>;
    async fn check_rule_email_can_be_nullable(
        &self,
        app_config: AppConfig,
        user: CreateUser,
        c_apporg: CleanAppOrgByClientId,
    ) -> RepositoryResult<()>;

    async fn check_rule_is_must_username_unique(
        &self,
        app_config: AppConfig,
        user: CreateUser,
        c_apporg: CleanAppOrgByClientId,
    ) -> RepositoryResult<()>;
    fn hash_password(&self, password: String) -> RepositoryResult<String>;
    fn verify_password(&self, stored_hash: String, password: String) -> RepositoryResult<bool>;
    async fn send_otp(&self);
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(
        &self,
        c_apporg: CleanAppOrgByClientId,
        user: CreateUser,
    ) -> RepositoryResult<Uuid> {
        let Ok(app_config) = c_apporg.get_config() else {
            return Err(RepositoryError::new(
                "failed to read config".to_string(),
                500,
            ));
        };
        self.check_rule_name(user.username.clone())?;
        self.check_rule_email_can_be_nullable(app_config.clone(), user.clone(), c_apporg.clone())
            .await?;
        self.check_rule_is_must_username_unique(app_config.clone(), user.clone(), c_apporg.clone())
            .await?;
        let hashed_password = self.hash_password(user.password)?;
        let new_user = User::new(c_apporg.clone(), user.username, hashed_password, user.email);
        let new_uorg = UserOrganization::new(c_apporg, new_user.clone());
        let user_id = new_user.user_id.clone();
        self.create_user(new_user, new_uorg).await?;

        Ok(user_id)
    }

    fn check_rule_name(&self, rule_name: String) -> RepositoryResult<()> {
        if rule_name.len() <= 2 || rule_name.len() >= 50 {
            return Err(RepositoryError::new(
                "username is not validate".to_string(),
                400,
            ));
        }
        Ok(())
    }
    async fn check_rule_is_must_username_unique(
        &self,
        app_config: AppConfig,
        user: CreateUser,
        c_apporg: CleanAppOrgByClientId,
    ) -> RepositoryResult<()> {
        if !app_config.is_must_name_unique {
            return Ok(());
        }
        let fetched_user = self
            .find_user_by_username(
                user.username,
                c_apporg.application_id,
                c_apporg.organization_id,
            )
            .await?;
        if fetched_user.is_some() {
            return Err(RepositoryError::new(
                "this username has used".to_string(),
                400,
            ));
        }
        Ok(())
    }
    async fn check_rule_email_can_be_nullable(
        &self,
        app_config: AppConfig,
        user: CreateUser,
        c_apporg: CleanAppOrgByClientId,
    ) -> RepositoryResult<()> {
        if app_config.can_allow_email_nullable {
            return Ok(());
        }
        let Some(email) = user.email.as_ref() else {
            return Err(RepositoryError::new("email is required".to_string(), 399));
        };
        let fetched_user = self
            .find_user_by_email(
                email.clone(),
                c_apporg.application_id,
                c_apporg.organization_id,
            )
            .await?;
        if fetched_user.is_some() {
            return Err(RepositoryError::new("this email has used".to_string(), 399));
        }
        Ok(())
    }

    async fn create_user(&self, user: User, u_org: UserOrganization) -> RepositoryResult<()> {
        self.insert_into_user(&user).await?;
        self.insert_into_user_by_email(&user).await?;
        self.insert_into_user_by_username(&user).await?;
        self.insert_into_user_organizations(&u_org).await?;
        self.insert_into_user_organizations_by_user(&u_org).await?;
        Ok(())
    }
    async fn insert_into_user(&self, user: &User) -> RepositoryResult<()> {
        let query = "INSERT INTO axcelium.users (
            user_id, organization_id, application_id,
            username, email, password_hash,
            created_at, updated_at,
            is_active, is_verified, is_locked,
            last_login, mfa_enabled, deactivated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
        self.database.query_unpaged(query, user).await?;
        Ok(())
    }

    async fn insert_into_user_by_email(&self, user: &User) -> RepositoryResult<()> {
        if user.email.is_some() {
            let query = "INSERT INTO axcelium.users_by_email (
                    email, organization_id, application_id,
                    user_id, username, password_hash,
                    created_at, updated_at,
                    is_active, is_verified, is_locked,
                    last_login, mfa_enabled, deactivated_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
            self.database.query_unpaged(query, &user).await?;
        }
        Ok(())
    }

    async fn insert_into_user_by_username(&self, user: &User) -> RepositoryResult<()> {
        let query = "INSERT INTO axcelium.users_by_username (
                username, organization_id, application_id,
                email, user_id, password_hash,
                created_at, updated_at,
                is_active, is_verified, is_locked,
                last_login, mfa_enabled, deactivated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
        self.database.query_unpaged(query, &user).await?;
        Ok(())
    }

    async fn insert_into_user_organizations(
        &self,
        user_org: &UserOrganization,
    ) -> RepositoryResult<()> {
        let query = "INSERT INTO axcelium.user_organizations (
            organization_id, user_id, role,
            username, user_email,
            organization_name, organization_slug, contact_email,
            joined_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)";
        self.database.query_unpaged(query, &user_org).await?;
        Ok(())
    }

    async fn insert_into_user_organizations_by_user(
        &self,
        user_org: &UserOrganization,
    ) -> RepositoryResult<()> {
        let query = "INSERT INTO axcelium.user_organizations_by_user (
            user_id, organization_id, role,
            username, user_email,
            organization_name, organization_slug, contact_email,
            joined_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)";
        self.database.query_unpaged(query, &user_org).await?;
        Ok(())
    }
    async fn find_user_by_email(
        &self,
        email: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<Option<CreatedUser>> {
        let query = "SELECT username FROM users_by_email \
                    WHERE email = ? AND application_id = ? AND organization_id = ?";

        let result = self
            .database
            .query_unpaged(query, (email, application_id, organization_id))
            .await?
            .into_rows_result()?;

        let row = result.maybe_first_row::<CreatedUser>()?;
        Ok(row)
    }

    async fn find_user_by_username(
        &self,
        username: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<Option<CreatedUser>> {
        let query = "SELECT username FROM users_by_username \
                WHERE username = ? AND application_id = ? AND organization_id = ?";
        let result = self
            .database
            .query_unpaged(query, (username, application_id, organization_id))
            .await?
            .into_rows_result()?;

        let row = result.maybe_first_row::<CreatedUser>()?;
        Ok(row)
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
