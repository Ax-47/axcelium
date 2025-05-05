use std::sync::Arc;

use redis::Client;
use scylla::client::session::Session;

use crate::application::{
    services::initial_core_service::{InitialCoreService, InitialCoreServiceImpl},
    repositories::{
        initial_core::InitialCoreImpl, user_repository::UserRepositoryImpl,
        validate_bearer_auth_repository::ValidateBearerAuthMiddlewareRepositoryImpl,
    },
};
use crate::infrastructure::repositories::{
    cache::applications_organization_by_client_id_repository::ApplicationsOrganizationByClientIdCacheImpl,
    cache_layer::applications_organization_by_client_id_repository::ApplicationsOrganizationByClientIdCacheLayerImpl,
    cipher::{aes_gcm_repository::AesGcmCipherImpl, base64_repository::Base64RepositoryImpl},
    database::{
        application_repository::ApplicationDatabaseRepositoryImpl,
        applications_organization_by_client_id_repository::ApplicationsOrganizationByClientIdDatabaseRepositoryImpl,
        organization_repository::OrganizationDatabaseRepositoryImpl,
        user_repository::UserDatabaseRepositoryImpl,
    },
    security::argon2_repository::PasswordHasherImpl,
};

pub struct Repositories {
    pub user_repo: Arc<UserRepositoryImpl>,
    pub auth_repo: Arc<ValidateBearerAuthMiddlewareRepositoryImpl>,
}

pub fn create_all(
    database: Arc<Session>,
    cache: Arc<Client>,
    secret: &str,
    cache_ttl: u64,
) -> (Repositories, Arc<dyn InitialCoreService>) {
    let user_db = Arc::new(UserDatabaseRepositoryImpl::new(database.clone()));
    let password_hasher = Arc::new(PasswordHasherImpl::new());

    let aes_repo = Arc::new(AesGcmCipherImpl::new(secret.as_bytes()));
    let base64_repo = Arc::new(Base64RepositoryImpl);

    let org_db_repo = Arc::new(OrganizationDatabaseRepositoryImpl::new(database.clone()));
    let app_db_repo = Arc::new(ApplicationDatabaseRepositoryImpl::new(database.clone()));
    let apporg_db_repo =
        Arc::new(ApplicationsOrganizationByClientIdDatabaseRepositoryImpl::new(database.clone()));

    let apporg_cache_repo = Arc::new(ApplicationsOrganizationByClientIdCacheImpl::new(
        cache, cache_ttl,
    ));

    let apporg_cache_layer = Arc::new(ApplicationsOrganizationByClientIdCacheLayerImpl::new(
        apporg_cache_repo,
        apporg_db_repo.clone(),
    ));

    let user_repo = Arc::new(UserRepositoryImpl::new(user_db, password_hasher));

    let auth_repo = Arc::new(ValidateBearerAuthMiddlewareRepositoryImpl::new(
        apporg_cache_layer.clone(),
        base64_repo.clone(),
        aes_repo.clone(),
    ));

    let core_repo = Arc::new(InitialCoreImpl::new(
        aes_repo,
        base64_repo,
        org_db_repo,
        app_db_repo,
        apporg_cache_layer.clone(),
    ));

    let core_service = Arc::new(InitialCoreServiceImpl::new(core_repo));

    (
        Repositories {
            user_repo,
            auth_repo,
        },
        core_service,
    )
}
