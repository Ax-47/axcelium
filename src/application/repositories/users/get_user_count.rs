use crate::{
    domain::errors::repositories_errors::RepositoryResult,
    infrastructure::{
        repositories::database::user_repository::UserDatabaseRepository,
    },
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
pub struct GetUserCountRepositoryImpl {
    database_repo: Arc<dyn UserDatabaseRepository>,
}

impl GetUserCountRepositoryImpl {
    pub fn new(database_repo: Arc<dyn UserDatabaseRepository>) -> Self {
        Self { database_repo }
    }
}
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GetUserCountRepository: Send + Sync {
    async fn get_user_count(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
    ) -> RepositoryResult<i64>;
}

#[async_trait]
impl GetUserCountRepository for GetUserCountRepositoryImpl {
    async fn get_user_count(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
    ) -> RepositoryResult<i64> {
        self.database_repo
            .get_user_count(organization_id, application_id)
            .await
    }
}
