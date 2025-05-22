use crate::domain::errors::repositories_errors::RepositoryResult;
use crate::infrastructure::models::refresh_token::PaginatedRefreshTokensByUserModel;
use crate::infrastructure::repositories::database::refresh_token::RefreshTokenDatabaseRepository;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

pub struct GetRefreshTokenRepositoryImpl {
    database_repo: Arc<dyn RefreshTokenDatabaseRepository>,
}
impl GetRefreshTokenRepositoryImpl {
    pub fn new(database_repo: Arc<dyn RefreshTokenDatabaseRepository>) -> Self {
        Self { database_repo }
    }
}

#[async_trait]
pub trait GetRefreshTokenRepository: Send + Sync {
    async fn get_refresh_token(
        &self,
        org_id: Uuid,
        app_id: Uuid,
        user_id: Uuid,
        page_size: i32,
        paging_state_u8: Option<Vec<u8>>,
    ) -> RepositoryResult<PaginatedRefreshTokensByUserModel>;
}

#[async_trait]
impl GetRefreshTokenRepository for GetRefreshTokenRepositoryImpl {
    async fn get_refresh_token(
        &self,
        org_id: Uuid,
        app_id: Uuid,
        user_id: Uuid,
        page_size: i32,
        paging_state_u8: Option<Vec<u8>>,
    ) -> RepositoryResult<PaginatedRefreshTokensByUserModel> {
        self.database_repo
            .find_refresh_token_by_user(org_id, app_id, user_id, page_size, paging_state_u8)
            .await
    }
}
