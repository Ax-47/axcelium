use crate::{
    application::{
        dto::{payload::user::CreateUserPayload, response::user::CreateUserResponse},
        repositories::user_repository::UserRepository,
    },
    domain::{
        entities::apporg_client_id::CleanAppOrgByClientId,
        entities::apporg_client_id::HasAppConfig,
        errors::repositories_errors::{RepositoryError, RepositoryResult},
    },
};
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserServiceImpl {
    pub repository: Arc<dyn UserRepository>,
}
impl UserServiceImpl {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        UserServiceImpl { repository }
    }
}
#[async_trait]
pub trait UserService: 'static + Sync + Send {
    async fn create(
        &self,
        c_apporg: CleanAppOrgByClientId,
        user: CreateUserPayload,
    ) -> RepositoryResult<CreateUserResponse>;
}
#[async_trait]
impl UserService for UserServiceImpl {
    async fn create(
        &self,
        c_apporg: CleanAppOrgByClientId,
        user: CreateUserPayload,
    ) -> RepositoryResult<CreateUserResponse> {
        if !(2..=50).contains(&user.username.len()) {
            return Err(RepositoryError::new("username is not valid".into(), 400));
        }
        let Ok(config) = c_apporg.get_config() else {
            return Err(RepositoryError::new(
                "failed to read config".to_string(),
                500,
            ));
        };

        if !config.can_allow_email_nullable {
            let Some(email) = user.email.as_ref() else {
                return Err(RepositoryError::new("email is required".to_string(), 400));
            };

            let found = self
                .repository
                .find_user_by_email(
                    email.clone(),
                    c_apporg.application_id,
                    c_apporg.organization_id,
                )
                .await?;

            if found.is_some() {
                return Err(RepositoryError::new("email already used".into(), 400));
            }
        }

        if !config.is_must_name_unique {
            let found = self
                .repository
                .find_user_by_username(
                    user.username.clone(),
                    c_apporg.application_id,
                    c_apporg.organization_id,
                )
                .await?;
            if found.is_some() {
                return Err(RepositoryError::new("username already used".into(), 400));
            }
        }
        let hashed_password = self.repository.hash_password(user.password.clone())?;
        let new_user = self
            .repository
            .new_user(c_apporg.clone(), user, hashed_password);
        let new_uorg = self
            .repository
            .new_user_organization(c_apporg, new_user.clone());
        self.repository
            .create_user(new_user.clone(), new_uorg)
            .await?;
        return Ok(CreateUserResponse {
            user_id: new_user.user_id.to_string(),
            username: new_user.username,
            email: new_user.email,
        });
    }
}
