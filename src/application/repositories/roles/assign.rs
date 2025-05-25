use crate::{
    domain::errors::repositories_errors::RepositoryResult,
    infrastructure::{models::role::RoleAssignmentModel, repositories::database::roles::RoleDatabaseRepository},
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
pub struct AssignRepositoryImpl {
    database_repo: Arc<dyn RoleDatabaseRepository>,
}

impl AssignRepositoryImpl {
    pub fn new(database_repo: Arc<dyn RoleDatabaseRepository>) -> Self {
        Self { database_repo }
    }
}
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait AssignRepository: Send + Sync {
    async fn assign(
        &self,
        org_id: Uuid,
        app_id: Uuid,
        role_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<()>;
}

#[async_trait]
impl AssignRepository for AssignRepositoryImpl {
    async fn assign(
        &self,
        org_id: Uuid,
        app_id: Uuid,
        role_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<()> {
        let moddel = RoleAssignmentModel{
            organization_id:org_id,
            application_id: app_id,
            role_id,
            user_id
        };
        self.database_repo
            .assign_user_to_role(&moddel)
            .await
    }
}
