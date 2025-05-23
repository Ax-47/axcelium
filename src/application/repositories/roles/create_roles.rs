use crate::{
    domain::errors::repositories_errors::RepositoryResult,
    infrastructure::{
        models::user::CleannedUserModel,
        repositories::database::user_repository::UserDatabaseRepository,
    },
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
pub struct CreateRoleRepositoryImpl {
    database_repo: Arc<dyn UserDatabaseRepository>,
}

impl CreateRoleRepositoryImpl {
    pub fn new(database_repo: Arc<dyn UserDatabaseRepository>) -> Self {
        Self { database_repo }
    }
}
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait CreateRoleRepository: Send + Sync {
    async fn find_user(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<Option<CleannedUserModel>>;
}

#[async_trait]
impl CreateRoleRepository for CreateRoleRepositoryImpl {
    async fn find_user(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<Option<CleannedUserModel>> {
        self.database_repo
            .find_user(application_id, organization_id, user_id)
            .await
    }
}
