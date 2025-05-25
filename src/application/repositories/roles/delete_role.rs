use crate::{
    domain::errors::repositories_errors::RepositoryResult,
    infrastructure::repositories::database::roles::RoleDatabaseRepository,
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
pub struct DeleteRoleRepositoryImpl {
    database_repo: Arc<dyn RoleDatabaseRepository>,
}

impl DeleteRoleRepositoryImpl {
    pub fn new(database_repo: Arc<dyn RoleDatabaseRepository>) -> Self {
        Self { database_repo }
    }
}
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait DeleteRoleRepository: Send + Sync {
    async fn delete_role(&self, org_id: Uuid, app_id: Uuid, role_id: Uuid) -> RepositoryResult<()>;
}

#[async_trait]
impl DeleteRoleRepository for DeleteRoleRepositoryImpl {
    async fn delete_role(&self, org_id: Uuid, app_id: Uuid, role_id: Uuid) -> RepositoryResult<()> {
        self.database_repo
            .delete_role(org_id, app_id, role_id)
            .await
    }
}
