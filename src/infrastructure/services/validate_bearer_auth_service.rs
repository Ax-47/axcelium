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
pub trait VaildateBearerAuthMiddlewareService: 'static +Send + Sync {
    async fn authentication(&self,token:String);
}

#[async_trait]
impl VaildateBearerAuthMiddlewareService for VaildateBearerAuthMiddlewareServiceImpl {
    async fn authentication(&self,token:String){
        println!("token: {:?}",token);
    }
}