use crate::{
    domain::{entities::role_by_app::RoleByApp, errors::repositories_errors::RepositoryResult},
    infrastructure::{
        models::role::{RoleModel, SelectedRoleByIdModel}, repositories::database::roles::RoleDatabaseRepository,
    },
};
use async_trait::async_trait;
use chrono::Utc;
use scylla::value::CqlTimestamp;
use std::{collections::HashSet, sync::Arc};
use uuid::Uuid;
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
    fn new_role(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        name: String,
        description: Option<String>,
        permissions: HashSet<String>,
    ) -> RoleByApp;
    async fn create_role(&self, role: &RoleModel) -> RepositoryResult<()>;
    async fn get_role(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        role_id: Uuid,
    ) -> RepositoryResult<Option<SelectedRoleByIdModel>>;
}

#[async_trait]
impl CreateRoleRepository for CreateRoleRepositoryImpl {
    fn new_role(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        name: String,
        description: Option<String>,
        permissions: HashSet<String>,
    ) -> RoleByApp {
        let now = Utc::now().timestamp_millis();
        RoleByApp {
            organization_id,
            application_id,
            role_id: Uuid::new_v4(),
            name,
            description,
            permissions,
            created_at: CqlTimestamp(now),
            updated_at: CqlTimestamp(now),
        }
    }
    async fn create_role(&self, role: &RoleModel) -> RepositoryResult<()> {
        self.database_repo.create_role(role).await
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
}
