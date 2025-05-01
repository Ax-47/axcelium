use crate::{
    domain::{
        errors::repositories_errors::{RepositoryError, RepositoryResult},
        models::{
            apporg_client_id_models::CleanAppOrgByClientId,
            user_models::{CreateUser, User, UserOrganization},
        },
    },
    infrastructure::{
        database::user_repository::UserDatabaseRepository,
        rule_checker::user_rule::UserRuleCheckerRepository,
        security::argon2_repository::PasswordHasherRepository,
    },
};
use async_trait::async_trait;
use redis::Client as RedisClient;
use std::sync::Arc;
use uuid::Uuid;
pub struct UserRepositoryImpl {
    pub cache: Arc<RedisClient>,
    database_repo: Arc<dyn UserDatabaseRepository>,
    hasher_repo: Arc<dyn PasswordHasherRepository>,
    user_rule_repo: Arc<dyn UserRuleCheckerRepository>,
}

impl UserRepositoryImpl {
    pub fn new(
        cache: Arc<RedisClient>,
        database_repo: Arc<dyn UserDatabaseRepository>,
        hasher_repo: Arc<dyn PasswordHasherRepository>,
        user_rule_repo: Arc<dyn UserRuleCheckerRepository>,
    ) -> Self {
        Self {
            cache,
            database_repo,
            hasher_repo,
            user_rule_repo,
        }
    }
}
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(
        &self,
        c_apporg: CleanAppOrgByClientId,
        user: CreateUser,
    ) -> RepositoryResult<Uuid>;
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

        self.user_rule_repo
            .check_rule_name(user.username.as_str())?;

        self.user_rule_repo
            .check_email_nullable(&app_config, &user, &c_apporg)
            .await?;

        self.user_rule_repo
            .check_username_unique(&app_config, &user, &c_apporg)
            .await?;

        let hashed_password = self.hasher_repo.hash(user.password.as_str())?;

        let new_user = User::new(c_apporg.clone(), user.username, hashed_password, user.email);
        let new_uorg = UserOrganization::new(c_apporg, new_user.clone());
        let user_id = new_user.user_id.clone();
        self.database_repo.create_user(new_user, new_uorg).await?;

        Ok(user_id)
    }
}
