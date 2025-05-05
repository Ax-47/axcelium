use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
use crate::application::mappers::model::ModelMapper;
use crate::domain::entities::apporg_client_id::AppOrgByClientId;
use crate::domain::errors::repositories_errors::RepositoryResult;
use crate::infrastructure::repositories::{
    cache_layer::applications_organization_by_client_id_repository::ApplicationsOrganizationByClientIdCacheLayerRepository,
    cipher::{aes_gcm_repository::AesGcmCipherRepository, base64_repository::Base64Repository},
};
pub struct ValidateBearerAuthMiddlewareRepositoryImpl {
    apporg_cachelayer_repo: Arc<dyn ApplicationsOrganizationByClientIdCacheLayerRepository>,
    base64_repo: Arc<dyn Base64Repository>,
    aes_repo: Arc<dyn AesGcmCipherRepository>,
}
impl ValidateBearerAuthMiddlewareRepositoryImpl {
    pub fn new(
        apporg_cachelayer_repo: Arc<dyn ApplicationsOrganizationByClientIdCacheLayerRepository>,
        base64_repo: Arc<dyn Base64Repository>,
        aes_repo: Arc<dyn AesGcmCipherRepository>,
    ) -> Self {
        Self {
            apporg_cachelayer_repo,
            base64_repo,
            aes_repo,
        }
    }
}

#[async_trait]
pub trait ValidateBearerAuthMiddlewareRepository: Send + Sync {
    async fn decrypt_token(&self, token: Vec<String>) -> RepositoryResult<(Uuid, String, String)>;
    async fn fetch_apporg_by_client_id(
        &self,
        client_id: Uuid,
    ) -> RepositoryResult<Option<AppOrgByClientId>>;
    async fn decrypt_client_secret(
        &self,
        client_key: &str,
        encrypt_client_secret: &str,
    ) -> RepositoryResult<String>;
}

#[async_trait]
impl ValidateBearerAuthMiddlewareRepository for ValidateBearerAuthMiddlewareRepositoryImpl {
    async fn decrypt_token(&self, token: Vec<String>) -> RepositoryResult<(Uuid, String, String)> {
        let decoded_client_id = self.base64_repo.decode(&token[0])?;
        let decoded_client_secret = self.base64_repo.decode(&token[2])?;
        let client_id = Uuid::parse_str(
            String::from_utf8_lossy(&decoded_client_id)
                .into_owned()
                .as_str(),
        )?;
        let client_key = token[1].to_owned();
        let client_secret = String::from_utf8_lossy(&decoded_client_secret).into_owned();
        return Ok((client_id, client_key, client_secret));
    }
    async fn fetch_apporg_by_client_id(
        &self,
        client_id: Uuid,
    ) -> RepositoryResult<Option<AppOrgByClientId>> {
        let Some(fetched)=self.apporg_cachelayer_repo
            .find_apporg_by_client_id(client_id)
            .await? else{
                return Ok(None);
            };

        Ok(Some(fetched.to_entity()))
    }
    async fn decrypt_client_secret(
        &self,
        client_key: &str,
        encrypted_client_secret: &str,
    ) -> RepositoryResult<String> {
        self.aes_repo
            .decrypt(&client_key, &encrypted_client_secret)
            .await
    }
}
