use crate::{
    application::{
        dto::response::user::GetUserResponse, repositories::users::get::GetUserRepository,
    },
    domain::errors::repositories_errors::RepositoryResult,
};
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Clone)]
pub struct GetUserServiceImpl {
    pub repository: Arc<dyn GetUserRepository>,
}
impl GetUserServiceImpl {
    pub fn new(repository: Arc<dyn GetUserRepository>) -> Self {
        GetUserServiceImpl { repository }
    }
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GetUserService: 'static + Sync + Send {
    async fn execute(&self) -> RepositoryResult<GetUserResponse>;
}
#[async_trait]
impl GetUserService for GetUserServiceImpl {
    async fn execute(&self) -> RepositoryResult<GetUserResponse> {}
}
