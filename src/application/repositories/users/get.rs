use crate::infrastructure::repositories::database::user_repository::UserDatabaseRepository;
use async_trait::async_trait;
use std::sync::Arc;
pub struct GetUserRepositoryImpl {
    database_repo: Arc<dyn UserDatabaseRepository>,
}

impl GetUserRepositoryImpl {
    pub fn new(database_repo: Arc<dyn UserDatabaseRepository>) -> Self {
        Self { database_repo }
    }
}
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GetUserRepository: Send + Sync {
    async fn find_all_users_paginated(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        page_size: i32,
        paging_state_u8: Option<Vec<u8>>,
    ) -> RepositoryResult<PaginatedUsers>;
}

#[async_trait]
impl GetUserRepository for GetUserRepositoryImpl {
    async fn find_all_users_paginated(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        page_size: i32,
        paging_state_u8: Option<Vec<u8>>,
    ) -> RepositoryResult<PaginatedUsers> {
        self.database_repo
            .find_all_users_paginated(organization_id, application_id, page_size, paging_state)
            .await
    }
}
