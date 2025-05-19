use crate::{
    domain::errors::repositories_errors::RepositoryResult,
    infrastructure::{
        repositories::database::user_repository::UserDatabaseRepository,
    },
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
pub struct DisableMFAUserRepositoryImpl {
    database_repo: Arc<dyn UserDatabaseRepository>,
}

impl DisableMFAUserRepositoryImpl {
    pub fn new(database_repo: Arc<dyn UserDatabaseRepository>) -> Self {
        Self { database_repo }
    }
}
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait DisableMFAUserRepository: Send + Sync {
    async fn disable_mfa_user(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
        application_id: Uuid,
    ) -> RepositoryResult<()>;
}

#[async_trait]
impl DisableMFAUserRepository for DisableMFAUserRepositoryImpl {
    async fn disable_mfa_user(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
        application_id: Uuid,
    ) -> RepositoryResult<()> {
        self.database_repo
            .disable_mfa_user(user_id ,organization_id, application_id)
            .await
    }
}
