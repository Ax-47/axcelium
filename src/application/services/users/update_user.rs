use crate::{
    application::{
        dto::{payload::user::UpdateUserPayload, response::user::UpdateUsersResponse},
        repositories::users::update_user::UpdateUserRepository,
    },
    domain::{
        entities::apporg_client_id::CleanAppOrgByClientId,
        errors::repositories_errors::{RepositoryError, RepositoryResult},
    },
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
#[derive(Clone)]
pub struct UpdateUserServiceImpl {
    pub repository: Arc<dyn UpdateUserRepository>,
}
impl UpdateUserServiceImpl {
    pub fn new(repository: Arc<dyn UpdateUserRepository>) -> Self {
        Self { repository }
    }
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UpdateUserService: 'static + Sync + Send {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        user_id: Uuid,
        update: UpdateUserPayload,
    ) -> RepositoryResult<UpdateUsersResponse>;
}
#[async_trait]
impl UpdateUserService for UpdateUserServiceImpl {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        user_id: Uuid,
        update: UpdateUserPayload,
    ) -> RepositoryResult<UpdateUsersResponse> {
        let Some(mut user) = self
            .repository
            .find_user(c_apporg.organization_id, c_apporg.application_id, user_id)
            .await?
        else {
            return Err(RepositoryError::new("not found user".to_string(), 400));
        };
        if let Some(password) = update.password {
            user.hashed_password = self.repository.hash_password(password)?;
        }
        if let Some(username) = update.username {
            self.repository
                .validate_new_username(&user.username, &username)?;
            if self
                .repository
                .find_user_by_username(
                    username.clone(),
                    c_apporg.application_id,
                    c_apporg.organization_id,
                )
                .await?
                .is_some()
            {
                return Err(RepositoryError::new("username already used".into(), 400));
            }
            user.username = username;
        }
        if let Some(email) = update.email {
            self.repository.validate_new_email(&user.email, &email)?;
            if self
                .repository
                .find_user_by_email(
                    email.clone(),
                    c_apporg.application_id,
                    c_apporg.organization_id,
                )
                .await?
                .is_some()
            {
                return Err(RepositoryError::new("email already used".into(), 400));
            }
            user.email = Some(email);
        }
        self.repository.update_user(user).await?;
        Ok(UpdateUsersResponse {
            massage: "success".to_string(),
        })
    }
}
