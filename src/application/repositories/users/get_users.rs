use crate::{
    domain::errors::repositories_errors::RepositoryResult,
    infrastructure::{
        models::user::PaginatedUsersModel,
        repositories::{
            cipher::base64_repository::Base64Repository,
            database::user_repository::UserDatabaseRepository,
        },
    },
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
pub struct GetUsersRepositoryImpl {
    database_repo: Arc<dyn UserDatabaseRepository>,
    base64_repo: Arc<dyn Base64Repository>,
}

impl GetUsersRepositoryImpl {
    pub fn new(
        database_repo: Arc<dyn UserDatabaseRepository>,
        base64_repo: Arc<dyn Base64Repository>,
    ) -> Self {
        Self {
            database_repo,
            base64_repo,
        }
    }
}
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GetUsersRepository: Send + Sync {
    async fn find_all_users_paginated(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        page_size: i32,
        paging_state_u8: Option<Vec<u8>>,
    ) -> RepositoryResult<PaginatedUsersModel>;
    fn bytes_to_base64(&self, bytes: Vec<u8>) -> String;

    fn base64_to_bytes(&self, base64: String) -> RepositoryResult<Vec<u8>>;
}

#[async_trait]
impl GetUsersRepository for GetUsersRepositoryImpl {
    async fn find_all_users_paginated(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        page_size: i32,
        paging_state_u8: Option<Vec<u8>>,
    ) -> RepositoryResult<PaginatedUsersModel> {
        self.database_repo
            .find_all_users_paginated(organization_id, application_id, page_size, paging_state_u8)
            .await
    }

    fn bytes_to_base64(&self, bytes: Vec<u8>) -> String {
        self.base64_repo.encode(bytes.as_slice())
    }

    fn base64_to_bytes(&self, base64: String) -> RepositoryResult<Vec<u8>> {
        let decoded = self.base64_repo.decode(base64.as_str())?;
        Ok(decoded)
    }
}
