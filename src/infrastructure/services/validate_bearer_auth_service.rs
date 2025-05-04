use async_trait::async_trait;

use std::sync::Arc;

use crate::{
    domain::{
        errors::repositories_errors::{RepositoryError, RepositoryResult},
        models::apporg_client_id_models::CleanAppOrgByClientId,
    },
    infrastructure::repositories::validate_bearer_auth_repository::VaildateBearerAuthMiddlewareRepository,
};
#[derive(Clone)]
pub struct VaildateBearerAuthMiddlewareServiceImpl {
    pub repository: Arc<dyn VaildateBearerAuthMiddlewareRepository>,
}
impl VaildateBearerAuthMiddlewareServiceImpl {
    pub fn new(repository: Arc<dyn VaildateBearerAuthMiddlewareRepository>) -> Self {
        Self { repository }
    }
}
impl VaildateBearerAuthMiddlewareServiceImpl {
    fn parse_header(&self, header: Option<String>) -> RepositoryResult<String> {
        let header = header.ok_or_else(|| RepositoryError::new("Missing Authorization".to_string(), 400))?;
        let value = header.strip_prefix("axcelium-core: ").ok_or_else(|| RepositoryError::new("Invalid Prefix".to_string(), 400))?;
        Ok(value.to_string())
    } 
}
#[async_trait]
pub trait VaildateBearerAuthMiddlewareService: 'static + Send + Sync {
    async fn authentication(&self, header: Option<String>) -> RepositoryResult<CleanAppOrgByClientId>;
}

#[async_trait]
impl VaildateBearerAuthMiddlewareService for VaildateBearerAuthMiddlewareServiceImpl {
    async fn authentication(&self, header: Option<String>) -> RepositoryResult<CleanAppOrgByClientId> {
        let token = self.parse_header(header)?;
        self.repository.authentication(token).await
    }
}
