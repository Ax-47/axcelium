use crate::domain::errors::repositories_errors::RepositoryResult;
use crate::infrastructure::models::refresh_token::PaginatedRefreshTokensByUserModel;
use crate::infrastructure::repositories::cipher::base64_repository::Base64Repository;
use crate::infrastructure::repositories::database::refresh_token::RefreshTokenDatabaseRepository;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

pub struct GetRefreshTokenRepositoryImpl {
    database_repo: Arc<dyn RefreshTokenDatabaseRepository>,
    base64_repo: Arc<dyn Base64Repository>,
}
impl GetRefreshTokenRepositoryImpl {
    pub fn new(
        database_repo: Arc<dyn RefreshTokenDatabaseRepository>,
        base64_repo: Arc<dyn Base64Repository>,
    ) -> Self {
        Self {
            database_repo,
            base64_repo,
        }
    }
}

#[async_trait]
pub trait GetRefreshTokenRepository: Send + Sync {
    async fn get_refresh_tokens_by_user(
        &self,
        org_id: Uuid,
        app_id: Uuid,
        user_id: Uuid,
        page_size: i32,
        paging_state_u8: Option<Vec<u8>>,
    ) -> RepositoryResult<PaginatedRefreshTokensByUserModel>;
    fn bytes_to_base64(&self, bytes: Vec<u8>) -> String;
    fn base64_to_bytes(&self, base64: String) -> RepositoryResult<Vec<u8>>;
}

#[async_trait]
impl GetRefreshTokenRepository for GetRefreshTokenRepositoryImpl {
    async fn get_refresh_tokens_by_user(
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

    fn bytes_to_base64(&self, bytes: Vec<u8>) -> String {
        self.base64_repo.encode(bytes.as_slice())
    }

    fn base64_to_bytes(&self, base64: String) -> RepositoryResult<Vec<u8>> {
        let decoded = self.base64_repo.decode(base64.as_str())?;
        Ok(decoded)
    }
}
