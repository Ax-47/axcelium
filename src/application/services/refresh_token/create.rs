use std::sync::Arc;

use async_trait::async_trait;

use crate::application::repositories::refresh_tokens::create::CreateRefreshTokenRepository;
#[derive(Clone)]
pub struct CreateRefreshTokenServiceImpl {
    repo: Arc<dyn CreateRefreshTokenRepository>,
}
impl CreateRefreshTokenServiceImpl {
    pub fn new(repo: Arc<dyn CreateRefreshTokenRepository>) -> Self {
        Self { repo }
    }
}
#[async_trait]
pub trait CreateRefreshTokenService: 'static + Sync + Send {
    async fn execute(&self) -> String;
}
#[async_trait]
impl CreateRefreshTokenService for CreateRefreshTokenServiceImpl {
    async fn execute(&self) -> String {
        "Hello, World".to_string()
    }
}
