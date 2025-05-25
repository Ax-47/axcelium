use crate::{
    domain::errors::repositories_errors::RepositoryResult,
    infrastructure::{
        models::role::{SelectedRoleByIdModel, UpdateRoleModel}, repositories::database::roles::RoleDatabaseRepository,
    },
};
use async_trait::async_trait;
use std::{collections::HashSet, sync::Arc};
use uuid::Uuid;
pub struct UpdateUsersByRoleRepositoryImpl {
    database_repo: Arc<dyn RoleDatabaseRepository>,
}

impl UpdateUsersByRoleRepositoryImpl {
    pub fn new(database_repo: Arc<dyn RoleDatabaseRepository>) -> Self {
        Self { database_repo }
    }
}
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UpdateUsersByRoleRepository: Send + Sync {
    async fn get_role(
        &self,
        org_id: Uuid,
        app_id: Uuid,
        role_id: Uuid,
    ) -> RepositoryResult<Option<SelectedRoleByIdModel>>;
    fn new_model(
        organization_id: Uuid,
        application_id: Uuid,
        role_id: Uuid,
        name: String,
        description: String,
        permissions: HashSet<String>,
    ) -> UpdateRoleModel;
    async fn update_role(&self, update: &UpdateRoleModel) -> RepositoryResult<()>;
}

#[async_trait]
impl UpdateUsersByRoleRepository for UpdateUsersByRoleRepositoryImpl {
    fn new_model(
        organization_id: Uuid,
        application_id: Uuid,
        role_id: Uuid,
        name: String,
        description: String,
        permissions: HashSet<String>,
    ) -> UpdateRoleModel {
        UpdateRoleModel {
            organization_id,
            application_id,
            role_id,
            name,
            description,
            permissions,
        }
    }

    async fn get_role(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        role_id: Uuid,
    ) -> RepositoryResult<Option<SelectedRoleByIdModel>> {
        self.database_repo
            .get_role(organization_id, application_id, role_id).await
    }
    async fn update_role(&self, update: &UpdateRoleModel) -> RepositoryResult<()> {
        self.database_repo.update_role(update).await
    }
}
