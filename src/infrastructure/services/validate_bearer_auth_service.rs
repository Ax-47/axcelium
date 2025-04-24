use async_trait::async_trait;

use std::sync::Arc;

use crate::infrastructure::repositories::validate_bearer_auth_repository::VaildateBearerAuthMiddlewareRepository;
#[derive(Clone)]
pub struct VaildateBearerAuthMiddlewareServiceImpl {
    pub repository: Arc<dyn VaildateBearerAuthMiddlewareRepository>,
}
impl VaildateBearerAuthMiddlewareServiceImpl {
    pub fn new(repository: Arc<dyn VaildateBearerAuthMiddlewareRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
pub trait VaildateBearerAuthMiddlewareService: Send + Sync {}
impl VaildateBearerAuthMiddlewareService for VaildateBearerAuthMiddlewareServiceImpl {
    
}