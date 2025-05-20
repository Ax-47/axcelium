use crate::domain::errors::repositories_errors::RepositoryResult;
use crate::infrastructure::models::apporg_client_id::AppOrgModel;
use crate::infrastructure::repositories::cache::applications_organization_by_client_id_repository::ApplicationsOrganizationByClientIdCacheRepository;
use crate::infrastructure::repositories::database::applications_organization_by_client_id_repository::ApplicationsOrganizationByClientIdDatabaseRepository;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
pub struct RefreshTokenCacheLayerRepositoryImpl {
    cache_repo: Arc<dyn ApplicationsOrganizationByClientIdCacheRepository>,
    database_repo: Arc<dyn ApplicationsOrganizationByClientIdDatabaseRepository>,
}

impl RefreshTokenCacheLayerRepositoryImpl {
    pub fn new(
        cache_repo: Arc<dyn ApplicationsOrganizationByClientIdCacheRepository>,
        database_repo: Arc<dyn ApplicationsOrganizationByClientIdDatabaseRepository>,
    ) -> Self {
        Self {
            cache_repo,
            database_repo,
        }
    }
}
#[async_trait]
pub trait RefreshTokenCacheLayerRepository: Send + Sync {
    async fn find_apporg_by_client_id(
        &self,
        client_id: Uuid,
    ) -> RepositoryResult<Option<AppOrgModel>>;
    async fn create_apporg_by_client_id(&self, apporg: AppOrgModel) -> RepositoryResult<()>;
}

#[async_trait]
impl RefreshTokenCacheLayerRepository
    for RefreshTokenCacheLayerRepositoryImpl
{
    async fn find_apporg_by_client_id(&self, key: Uuid) -> RepositoryResult<Option<AppOrgModel>> {
        if let Some(cached_data) = self.cache_repo.get_cached_apporg_by_client_id(key).await? {
            return Ok(Some(cached_data));
        }
        let Some(data) = self.database_repo.find_apporg_by_client_id(key).await? else {
            return Ok(None);
        };
        self.cache_repo.cache_apporg_by_client_id(&data).await?;
        Ok(Some(data))
    }

    async fn create_apporg_by_client_id(&self, apporg: AppOrgModel) -> RepositoryResult<()> {
        self.database_repo.create_apporg_by_client_id(apporg).await
    }
}
