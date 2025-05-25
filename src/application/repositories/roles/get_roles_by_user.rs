use crate::{
    domain::errors::repositories_errors::RepositoryResult,
    infrastructure::{
        models::role::UserRoleModel, repositories::database::roles::RoleDatabaseRepository,
    },
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
pub struct GetRolesByUserRepositoryImpl {
    database_repo: Arc<dyn RoleDatabaseRepository>,
}

impl GetRolesByUserRepositoryImpl {
    pub fn new(database_repo: Arc<dyn RoleDatabaseRepository>) -> Self {
        Self { database_repo }
    }
}
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GetRolesByUserRepository: Send + Sync {
    async fn get_roles_by_user(
        &self,
        org_id: Uuid,
        app_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<Vec<UserRoleModel>>;
}

#[async_trait]
impl GetRolesByUserRepository for GetRolesByUserRepositoryImpl {
    async fn get_roles_by_user(
        &self,
        org_id: Uuid,
        app_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<Vec<UserRoleModel>> {
        self.database_repo
            .get_roles_by_user(org_id, app_id, user_id)
            .await
    }
}
