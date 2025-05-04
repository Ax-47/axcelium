use std::sync::Arc;

use crate::infrastructure::services::validate_bearer_auth_service::{
    VaildateBearerAuthMiddlewareService, VaildateBearerAuthMiddlewareServiceImpl,
};

use super::repositories::Repositories;

pub fn create_validate_bearer_auth_service(
    repos: &Repositories,
) -> Arc<dyn VaildateBearerAuthMiddlewareService> {
    Arc::new(VaildateBearerAuthMiddlewareServiceImpl {
        repository: repos.auth_repo.clone(),
    })
}
