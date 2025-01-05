use async_trait::async_trait;

use crate::{
    domain::{
        errors::repositories_errors::{CommonError, RepositoryResult},
        models::user_models::{CreateUser, CreatedUser, User},
    },
    infrastructure::repositories::user_repositories::UserRepository,
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
    async fn create(&self, user: CreateUser) -> Result<CreatedUser, CommonError>;
}
#[async_trait]
impl UserService for UserServiceImpl {
    async fn create(&self, user: CreateUser) -> Result<CreatedUser, CommonError> {
        let cloned = user.clone();
        let id = self
            .repository
            .create(cloned)
            .await
            .map_err(|e| -> CommonError { e.into() })?;
        return Ok(CreatedUser {
            id,
            username: user.username,
            role: user.role,
        });
    }
}
