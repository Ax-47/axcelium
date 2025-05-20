use crate::{
    application::{
        dto::response::user::CreateUserResponse,
        mappers::{application::ApplicationMapper, user::found::FoundUserMapper},
    },
    domain::errors::repositories_errors::{RepositoryError, RepositoryResult},
    infrastructure::{
        models::user::UserModel,
        repositories::{
            database::user_repository::UserDatabaseRepository,
            security::argon2_repository::PasswordHasherRepository,
        },
    },
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
pub struct UpdateUserRepositoryImpl {
    database_repo: Arc<dyn UserDatabaseRepository>,

    hasher_repo: Arc<dyn PasswordHasherRepository>,
}

impl UpdateUserRepositoryImpl {
    pub fn new(
        database_repo: Arc<dyn UserDatabaseRepository>,
        hasher_repo: Arc<dyn PasswordHasherRepository>,
    ) -> Self {
        Self {
            database_repo,
            hasher_repo,
        }
    }
}
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UpdateUserRepository: Send + Sync {
    async fn update_user(&self, user: UserModel) -> RepositoryResult<()>;
    async fn find_user(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<Option<UserModel>>;
    fn hash_password(&self, password: String) -> RepositoryResult<String>;

    async fn find_user_by_username(
        &self,
        username: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<Option<CreateUserResponse>>;

    async fn find_user_by_email(
        &self,
        email: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<Option<CreateUserResponse>>;
    fn validate_new_email(&self,current: &Option<String>, new: &String) -> RepositoryResult<()>;
    fn validate_new_username(&self,current: &String, new: &String) -> RepositoryResult<()>;
}

#[async_trait]
impl UpdateUserRepository for UpdateUserRepositoryImpl {
    async fn find_user(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<Option<UserModel>> {
        self.database_repo
            .find_raw_user(application_id, organization_id, user_id)
            .await
    }
    async fn update_user(&self, user: UserModel) -> RepositoryResult<()> {
        self.database_repo.update_user(user).await
    }

    fn hash_password(&self, password: String) -> RepositoryResult<String> {
        self.hasher_repo.hash(password.as_str())
    }

    async fn find_user_by_username(
        &self,
        username: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<Option<CreateUserResponse>> {
        let Some(user) = self
            .database_repo
            .find_user_by_username(username, application_id, organization_id)
            .await?
        else {
            return Ok(None);
        };

        Ok(Some(FoundUserMapper::to_dto(user)))
    }

    async fn find_user_by_email(
        &self,
        email: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<Option<CreateUserResponse>> {
        let Some(user) = self
            .database_repo
            .find_user_by_email(email, application_id, organization_id)
            .await?
        else {
            return Ok(None);
        };
        Ok(Some(FoundUserMapper::to_dto(user)))
    }
    fn validate_new_username(&self,current: &String, new: &String) -> RepositoryResult<()> {
        if !(2..=50).contains(&new.len()) {
            return Err(RepositoryError::new("username is not valid".into(), 400));
        }
        if new == current {
            return Err(RepositoryError::new("username is the same".into(), 400));
        }
        Ok(())
    }

    fn validate_new_email(&self,current: &Option<String>, new: &String) -> RepositoryResult<()> {
        if let Some(old_email) = current {
            if old_email == new {
                return Err(RepositoryError::new("email is the same".into(), 400));
            }
        }

        Ok(())
    }
}
