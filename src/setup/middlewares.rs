use std::sync::Arc;

use crate::application::services::validate_bearer_auth_service::{
    ValidateBearerAuthMiddlewareService, ValidateBearerAuthMiddlewareServiceImpl,
};

use super::repositories::Repositories;

pub fn create_validate_bearer_auth_service(
    repos: &Repositories,
) -> Arc<dyn ValidateBearerAuthMiddlewareService> {
    Arc::new(ValidateBearerAuthMiddlewareServiceImpl {
        repository: repos.auth_repo.clone(),
    })
}
