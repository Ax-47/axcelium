use crate::domain::errors::repositories_errors::RepositoryResult;
use crate::infrastructure::models::refresh_token::RefreshTokenModel;
use crate::infrastructure::repositories::cache::refresh_token_repository::RefreshTokenCacheRepository;
use crate::infrastructure::repositories::database::refresh_token::RefreshTokenDatabaseRepository;
use async_trait::async_trait;
use std::sync::Arc;
pub struct RefreshTokenCacheLayerRepositoryImpl {
    cache_repo: Arc<dyn RefreshTokenCacheRepository>,
    database_repo: Arc<dyn RefreshTokenDatabaseRepository>,
}

impl RefreshTokenCacheLayerRepositoryImpl {
    pub fn new(
        cache_repo: Arc<dyn RefreshTokenCacheRepository>,
        database_repo: Arc<dyn RefreshTokenDatabaseRepository>,
    ) -> Self {
        Self {
            cache_repo,
            database_repo,
        }
    }
}
#[async_trait]
pub trait RefreshTokenCacheLayerRepository: Send + Sync {
    async fn create_refresh_token(&self, rt: RefreshTokenModel) -> RepositoryResult<()>;
}

#[async_trait]
impl RefreshTokenCacheLayerRepository
    for RefreshTokenCacheLayerRepositoryImpl
{

    async fn create_refresh_token(&self, rt: RefreshTokenModel) -> RepositoryResult<()> {
        self.database_repo.create_refresh_token(rt.clone()).await?;
        self.cache_repo.cache_refresh_token(&rt).await
    }
}
