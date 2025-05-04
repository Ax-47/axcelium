use crate::{
    domain::{
        errors::repositories_errors::{RepositoryError, RepositoryResult},
        models::apporg_client_id_models::CleanAppOrgByClientId,
    },
    infrastructure::repositories::validate_bearer_auth_repository::ValidateBearerAuthMiddlewareRepository,
};
use async_trait::async_trait;
use std::sync::Arc;
#[derive(Clone)]
pub struct ValidateBearerAuthMiddlewareServiceImpl {
    pub repository: Arc<dyn ValidateBearerAuthMiddlewareRepository>,
}
impl ValidateBearerAuthMiddlewareServiceImpl {
    pub fn new(repository: Arc<dyn ValidateBearerAuthMiddlewareRepository>) -> Self {
        Self { repository }
    }
}
impl ValidateBearerAuthMiddlewareServiceImpl {
    fn parse_header(&self, header: Option<String>) -> RepositoryResult<Vec<String>> {
        let header =
            header.ok_or_else(|| RepositoryError::new("Missing Authorization".to_string(), 400))?;
        let value = header
            .strip_prefix("axcelium-core: ")
            .ok_or_else(|| RepositoryError::new("Invalid Prefix".to_string(), 400))?;
        let parts: Vec<String> = value.split('.').map(|v| v.to_string()).collect();
        if parts.len() != 3 {
            return Err(RepositoryError {
                message: "invalid credential format".to_string(),
                code: 400,
            });
        }
        Ok(parts)
    }
}
#[async_trait]
pub trait ValidateBearerAuthMiddlewareService: 'static + Send + Sync {
    async fn authentication(
        &self,
        header: Option<String>,
    ) -> RepositoryResult<CleanAppOrgByClientId>;
}

#[async_trait]
impl ValidateBearerAuthMiddlewareService for ValidateBearerAuthMiddlewareServiceImpl {
    async fn authentication(
        &self,
        header: Option<String>,
    ) -> RepositoryResult<CleanAppOrgByClientId> {
        let token = self.parse_header(header)?;
        let (client_id, client_key, client_secret) = self.repository.decrypt_token(token).await?;
        let Some(apporg) = self.repository.fetch_apporg_by_client_id(client_id).await? else {
            return Err(RepositoryError {
                message: "no found".to_string(),
                code: 404,
            });
        };
        let decrypted = self
            .repository
            .decrypt_client_secret(&client_key, &apporg.encrypted_client_secret)
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
}
