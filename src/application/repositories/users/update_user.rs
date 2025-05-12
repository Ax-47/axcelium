use crate::{
    domain::errors::repositories_errors::RepositoryResult,
    infrastructure::{models::user::CleannedUserModel, repositories::database::user_repository::UserDatabaseRepository},
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
pub struct UpdateUserRepositoryImpl {
    database_repo: Arc<dyn UserDatabaseRepository>,
}

impl UpdateUserRepositoryImpl {
    pub fn new(database_repo: Arc<dyn UserDatabaseRepository>) -> Self {
        Self { database_repo }
    }
}
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UpdateUserRepository: Send + Sync {
    async fn find_user(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<Option<CleannedUserModel>>;
}

#[async_trait]
impl UpdateUserRepository for UpdateUserRepositoryImpl {
    async fn find_user(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<Option<CleannedUserModel>>{
        self.database_repo.find_user(application_id, organization_id, user_id).await
    }
}