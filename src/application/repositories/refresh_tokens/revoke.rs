use crate::domain::errors::repositories_errors::RepositoryResult;
use crate::infrastructure::repositories::database::refresh_token::RefreshTokenDatabaseRepository;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

pub struct RevokeRefreshTokenRepositoryImpl {
    database_repo: Arc<dyn RefreshTokenDatabaseRepository>,
}
impl RevokeRefreshTokenRepositoryImpl {
    pub fn new(database_repo: Arc<dyn RefreshTokenDatabaseRepository>) -> Self {
        Self { database_repo }
    }
}

#[async_trait]
pub trait RevokeRefreshTokenRepository: Send + Sync {
    async fn revoke_refresh_token(
        &self,
        org_id: Uuid,
        app_id: Uuid,
        token_id: Uuid,
    ) -> RepositoryResult<()>;
}

#[async_trait]
impl RevokeRefreshTokenRepository for RevokeRefreshTokenRepositoryImpl {
    async fn revoke_refresh_token(
        &self,
        org_id: Uuid,
        app_id: Uuid,
        token_id: Uuid,
    ) -> RepositoryResult<()> {
        self.database_repo
            .revoke_refresh_token(org_id, app_id, token_id)
            .await
    }
}
