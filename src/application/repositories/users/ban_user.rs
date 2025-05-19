use crate::{
    domain::errors::repositories_errors::RepositoryResult,
    infrastructure::{
        repositories::database::user_repository::UserDatabaseRepository,
    },
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
pub struct BanUserRepositoryImpl {
    database_repo: Arc<dyn UserDatabaseRepository>,
}

impl BanUserRepositoryImpl {
    pub fn new(database_repo: Arc<dyn UserDatabaseRepository>) -> Self {
        Self { database_repo }
    }
}
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait BanUserRepository: Send + Sync {
    async fn ban_user(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
        application_id: Uuid,
    ) -> RepositoryResult<()>;
}

#[async_trait]
impl BanUserRepository for BanUserRepositoryImpl {
    async fn ban_user(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
        application_id: Uuid,
    ) -> RepositoryResult<()> {
        self.database_repo
            .ban_user(user_id ,organization_id, application_id)
            .await
    }
}
