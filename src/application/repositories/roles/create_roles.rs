use crate::{
    domain::errors::repositories_errors::RepositoryResult,
    infrastructure::{
        models::role::RoleModel, repositories::database::roles::RoleDatabaseRepository,
    },
};
use async_trait::async_trait;
use std::sync::Arc;
pub struct CreateRoleRepositoryImpl {
    database_repo: Arc<dyn RoleDatabaseRepository>,
}

impl CreateRoleRepositoryImpl {
    pub fn new(database_repo: Arc<dyn RoleDatabaseRepository>) -> Self {
        Self { database_repo }
    }
}
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait CreateRoleRepository: Send + Sync {
    async fn create_role(&self, role: &RoleModel) -> RepositoryResult<()>;
}

#[async_trait]
impl CreateRoleRepository for CreateRoleRepositoryImpl {
    async fn create_role(&self, role: &RoleModel) -> RepositoryResult<()> {
        self.database_repo.create_role(role).await
    }
}
