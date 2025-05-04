use crate::domain::{
    errors::repositories_errors::RepositoryResult,
    models::apporg_client_id_models::AppOrgByClientId,
};
use crate::infrastructure::cache::applications_organization_by_client_id_repository::ApplicationsOrganizationByClientIdCacheRepository;
use crate::infrastructure::database::applications_organization_by_client_id_repository::ApplicationsOrganizationByClientIdDatabaseRepository;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
pub struct ApplicationsOrganizationByClientIdCacheLayerImpl {
    cache_repo: Arc<dyn ApplicationsOrganizationByClientIdCacheRepository>,
    database_repo: Arc<dyn ApplicationsOrganizationByClientIdDatabaseRepository>,
}

impl ApplicationsOrganizationByClientIdCacheLayerImpl {
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
pub trait ApplicationsOrganizationByClientIdCacheLayerRepository: Send + Sync {
    async fn find_apporg_by_client_id(
        &self,
        client_id: Uuid,
    ) -> RepositoryResult<Option<AppOrgByClientId>>;
    async fn create_apporg_by_client_id(&self, apporg: AppOrgByClientId) -> RepositoryResult<()>;
}

#[async_trait]
impl ApplicationsOrganizationByClientIdCacheLayerRepository
    for ApplicationsOrganizationByClientIdCacheLayerImpl
{
    async fn find_apporg_by_client_id(&self, key: Uuid) -> RepositoryResult<Option<AppOrgByClientId>> {
        if let Some(cached_data) = self.cache_repo.get_cached_apporg_by_client_id(key).await? {
            return Ok(Some(cached_data));
        }
        let Some(data) = self.database_repo.find_apporg_by_client_id(key).await? else{
            return Ok(None);
        };
        self.cache_repo.cache_apporg_by_client_id(&data).await?;
        Ok(Some(data))
    }

    async fn create_apporg_by_client_id(&self, apporg: AppOrgByClientId) -> RepositoryResult<()>{
        self.database_repo.create_apporg_by_client_id(apporg).await
    }
}
