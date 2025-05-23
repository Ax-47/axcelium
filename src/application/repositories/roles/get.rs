use crate::{
    domain::{entities::role_by_app::RoleByApp, errors::repositories_errors::RepositoryResult},
    infrastructure::{
        models::role::RoleModel, repositories::database::roles::RoleDatabaseRepository,
    },
};
use async_trait::async_trait;
use chrono::Utc;
use scylla::value::CqlTimestamp;
use std::{collections::HashSet, sync::Arc};
use uuid::Uuid;
pub struct GetRoleRepositoryImpl {
    database_repo: Arc<dyn RoleDatabaseRepository>,
}

impl GetRoleRepositoryImpl {
    pub fn new(database_repo: Arc<dyn RoleDatabaseRepository>) -> Self {
        Self { database_repo }
    }
}
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GetRoleRepository: Send + Sync {
}

#[async_trait]
impl GetRoleRepository for GetRoleRepositoryImpl {
}
