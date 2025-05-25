use crate::{
    domain::errors::repositories_errors::RepositoryResult,
    infrastructure::{
        models::role::RoleUserModel, repositories::database::roles::RoleDatabaseRepository,
    },
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
pub struct GetUsersByRoleRepositoryImpl {
    database_repo: Arc<dyn RoleDatabaseRepository>,
}

impl GetUsersByRoleRepositoryImpl {
    pub fn new(database_repo: Arc<dyn RoleDatabaseRepository>) -> Self {
        Self { database_repo }
    }
}
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GetUsersByRoleRepository: Send + Sync {
    async fn get_users_by_role(
        &self,
        org_id: Uuid,
        app_id: Uuid,
        role_id: Uuid,
    ) -> RepositoryResult<Vec<RoleUserModel>>;
}

#[async_trait]
impl GetUsersByRoleRepository for GetUsersByRoleRepositoryImpl {
    async fn get_users_by_role(
        &self,
        org_id: Uuid,
        app_id: Uuid,
        role_id: Uuid,
    ) -> RepositoryResult<Vec<RoleUserModel>> {
        self.database_repo
            .get_users_by_role(org_id, app_id, role_id)
            .await
    }
}
