use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::errors::repositories_errors::{RepositoryError, RepositoryResult};
use crate::infrastructure::cache_layer::applications_organization_by_client_id_repository::ApplicationsOrganizationByClientIdCacheLayerRepository;
use crate::{
    domain::models::apporg_client_id_models::CleanAppOrgByClientId,
    infrastructure::cipher::{
        aes_gcm_repository::AesGcmCipherRepository, base64_repository::Base64Repository,
    },
};
pub struct VaildateBearerAuthMiddlewareRepositoryImpl {
    apporg_cachelayer_repo: Arc<dyn ApplicationsOrganizationByClientIdCacheLayerRepository>,
    base64_repo: Arc<dyn Base64Repository>,
    aes_repo: Arc<dyn AesGcmCipherRepository>,
}
impl VaildateBearerAuthMiddlewareRepositoryImpl {
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
pub trait VaildateBearerAuthMiddlewareRepository: Send + Sync {
    async fn authentication(&self, token: String) -> RepositoryResult<CleanAppOrgByClientId>;
    fn parse_axcelium_credentials(&self, input: String)
        -> RepositoryResult<(Uuid, String, String)>;
}

#[async_trait]
impl VaildateBearerAuthMiddlewareRepository for VaildateBearerAuthMiddlewareRepositoryImpl {
    async fn authentication(&self, token: String) -> RepositoryResult<CleanAppOrgByClientId> {
        let (client_id, client_key, client_secret) = self.parse_axcelium_credentials(token)?;
        let Some(apporg) = self
            .apporg_cachelayer_repo
            .find_apporg_by_client_id(client_id)
            .await?
        else {
            return Err(RepositoryError {
                message: "no found".to_string(),
                code: 404,
            });
        };

        let decrypted = self
            .aes_repo
            .decrypt(&client_key, &apporg.encrypted_client_secret)
            .await?;
        if decrypted != client_secret {
            return Err(RepositoryError {
                message: "unauth".to_string(),
                code: 401,
            });
        }
        let clean_apporg = CleanAppOrgByClientId::from(apporg);
        Ok(clean_apporg)
    }

    fn parse_axcelium_credentials(
        &self,
        input: String,
    ) -> RepositoryResult<(Uuid, String, String)> {
        let without_prefix =
            input
                .strip_prefix("axcelium-core: ")
                .ok_or_else(|| RepositoryError {
                    message: "missing axcelium-core prefix".to_string(),
                    code: 400,
                })?;
        let parts: Vec<&str> = without_prefix.split('.').collect();
        if parts.len() != 3 {
            return Err(RepositoryError {
                message: "invalid credential format".to_string(),
                code: 400,
            });
        }

        let decoded_client_id = self.base64_repo.decode(parts[0])?;
        let decoded_client_secret = self.base64_repo.decode(parts[2])?;
        Ok((
            Uuid::parse_str(
                String::from_utf8_lossy(&decoded_client_id)
                    .into_owned()
                    .as_str(),
            )?,
            parts[1].to_owned(),
            String::from_utf8_lossy(&decoded_client_secret).into_owned(),
        ))
    }
}
