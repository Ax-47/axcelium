use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    domain::{
        errors::repositories_errors::RepositoryResult,
        models::user_models::{CreateUser, CreatedUser},
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
        app_id: Uuid,
        org_id: Uuid,
        user: CreateUser,
    ) -> RepositoryResult<CreatedUser>;
}
#[async_trait]
impl UserService for UserServiceImpl {
    async fn create(
        &self,
        app_id: Uuid,
        org_id: Uuid,
        user: CreateUser,
    ) -> RepositoryResult<CreatedUser> {
        let cloned = user.clone();
        let id = self.repository.create(app_id, org_id, cloned).await?;
        return Ok(CreatedUser {
            id,
            username: user.username,
        });
    }
}
