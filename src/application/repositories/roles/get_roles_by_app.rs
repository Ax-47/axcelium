use crate::{
    domain::errors::repositories_errors::RepositoryResult,
    infrastructure::{
        models::role::SelectedRoleByAppModel, repositories::database::roles::RoleDatabaseRepository,
    },
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
pub struct GetRolesByAppRepositoryImpl {
    database_repo: Arc<dyn RoleDatabaseRepository>,
}

impl GetRolesByAppRepositoryImpl {
    pub fn new(database_repo: Arc<dyn RoleDatabaseRepository>) -> Self {
        Self { database_repo }
    }
}
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GetRolesByAppRepository: Send + Sync {
    async fn get_roles_by_app(
        &self,
        org_id: Uuid,
        app_id: Uuid,
    ) -> RepositoryResult<Vec<SelectedRoleByAppModel>>;
}

#[async_trait]
impl GetRolesByAppRepository for GetRolesByAppRepositoryImpl {
    async fn get_roles_by_app(
        &self,
        org_id: Uuid,
        app_id: Uuid,
    ) -> RepositoryResult<Vec<SelectedRoleByAppModel>> {
        self.database_repo.get_roles(org_id, app_id).await
    }
}
