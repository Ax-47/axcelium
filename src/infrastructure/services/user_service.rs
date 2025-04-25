use async_trait::async_trait;
use crate::{
    domain::{
        errors::repositories_errors::RepositoryResult,
        models::{apporg_client_id_models::CleanAppOrgByClientId, user_models::{CreateUser, CreatedUser}},
    },
    infrastructure::repositories::user_repository::UserRepository,
};
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
        user: CreateUser,
    ) -> RepositoryResult<CreatedUser>;
}
#[async_trait]
impl UserService for UserServiceImpl {
    async fn create(
        &self,
        c_apporg: CleanAppOrgByClientId,
        user: CreateUser,
    ) -> RepositoryResult<CreatedUser> {
        let cloned = user.clone();
        let id = self.repository.create(c_apporg, cloned).await?;
        return Ok(CreatedUser {
            user_id:id,
            username: user.username,
            email: user.email
        });
    }
}
