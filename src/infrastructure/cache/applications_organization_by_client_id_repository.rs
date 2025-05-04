use crate::domain::{
    errors::repositories_errors::RepositoryResult,
    models::apporg_client_id_models::AppOrgByClientId,
};
use async_trait::async_trait;
use redis::{AsyncCommands, Client};
use std::sync::Arc;
use uuid::Uuid;
pub struct ApplicationsOrganizationByClientIdCacheImpl {
    cache: Arc<Client>,
    ttl: u64,
}

impl ApplicationsOrganizationByClientIdCacheImpl {
    pub fn new(cache: Arc<Client>,ttl:u64) -> Self {
        Self { cache,ttl }
    }
}
#[async_trait]
pub trait ApplicationsOrganizationByClientIdCacheRepository: Send+Sync {
    async fn cache_apporg_by_client_id(&self, apporg: &AppOrgByClientId) -> RepositoryResult<()>;
    async fn get_cached_apporg_by_client_id(
        &self,
        client_id: Uuid,
    ) -> RepositoryResult<Option<AppOrgByClientId>>;
    async fn invalidate_cache(&self, client_id: Uuid) -> RepositoryResult<()>;
}

#[async_trait]
impl ApplicationsOrganizationByClientIdCacheRepository
    for ApplicationsOrganizationByClientIdCacheImpl
{
    async fn cache_apporg_by_client_id(&self, apporg: &AppOrgByClientId) -> RepositoryResult<()> {
        let mut conn = self.cache.get_multiplexed_tokio_connection().await?;
        let key = format!("apporg:client_id:{}", apporg.client_id);
        let value = serde_json::to_string(apporg)?;
        let _: () = conn.set_ex(key, value, self.ttl).await?;
        Ok(())
    }

    async fn get_cached_apporg_by_client_id(
        &self,
        client_id: Uuid,
    ) -> RepositoryResult<Option<AppOrgByClientId>> {
        let mut conn = self.cache.get_multiplexed_tokio_connection().await?;
        let key = format!("apporg:client_id:{}", client_id);
        let result: Option<String> = conn.get(key).await?;
        match result {
            Some(json) => {
                let apporg = serde_json::from_str(&json)?;
                Ok(Some(apporg))
            }
            None => Ok(None),
        }
    }

    async fn invalidate_cache(&self, client_id: Uuid) -> RepositoryResult<()> {
        let mut conn = self.cache.get_multiplexed_tokio_connection().await?;
        let key = format!("apporg:client_id:{}", client_id);
        let _: () =conn.del(key).await?;
        Ok(())
    }
}
