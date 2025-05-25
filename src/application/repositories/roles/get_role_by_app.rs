use crate::{
    domain::errors::repositories_errors::RepositoryResult,
    infrastructure::{
        models::role::SelectedRoleByIdModel, repositories::database::roles::RoleDatabaseRepository,
    },
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
pub struct GetRoleByAppRepositoryImpl {
    database_repo: Arc<dyn RoleDatabaseRepository>,
}

impl GetRoleByAppRepositoryImpl {
    pub fn new(database_repo: Arc<dyn RoleDatabaseRepository>) -> Self {
        Self { database_repo }
    }
}
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GetRoleByAppRepository: Send + Sync {
    async fn get_role(
        &self,
        org_id: Uuid,
        app_id: Uuid,
        role_id: Uuid,
    ) -> RepositoryResult<Option<SelectedRoleByIdModel>>;
}

#[async_trait]
impl GetRoleByAppRepository for GetRoleByAppRepositoryImpl {
    async fn get_role( &self,
        org_id: Uuid,
        app_id: Uuid,
        role_id: Uuid,
    ) -> RepositoryResult<Option<SelectedRoleByIdModel>> {
        self.database_repo.get_role(org_id, app_id, role_id).await
    }
}
