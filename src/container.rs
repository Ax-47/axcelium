use crate::infrastructure::{
    cache::applications_organization_by_client_id_repository::ApplicationsOrganizationByClientIdCacheImpl,
    cache_layer::applications_organization_by_client_id_repository::ApplicationsOrganizationByClientIdCacheLayerImpl,
    cipher::{aes_gcm_repository::AesGcmCipherImpl, base64_repository::Base64RepositoryImpl},
    database::{
        application_repository::ApplicationDatabaseRepositoryImpl,
        applications_organization_by_client_id_repository::ApplicationsOrganizationByClientIdDatabaseRepositoryImpl,
        organization_repository::OrganizationDatabaseRepositoryImpl,
        user_repository::UserDatabaseRepositoryImpl,
    },
    repositories::{
        hello_repository::{HelloRepository, HelloRepositoryImpl},
        initial_core::InitialCoreImpl,
        user_repository::{UserRepository, UserRepositoryImpl},
        validate_bearer_auth_repository::{
            VaildateBearerAuthMiddlewareRepository, VaildateBearerAuthMiddlewareRepositoryImpl,
        },
    },
    rule_checker::user_rule::UserRuleCheckerImpl,
    security::argon2_repository::PasswordHasherImpl,
    services::{
        hello_service::{HelloService, HelloServiceImpl},
        initial_core_service::{InitialCoreService, InitialCoreServiceImpl},
        user_service::{UserService, UserServiceImpl},
        validate_bearer_auth_service::{
            VaildateBearerAuthMiddlewareService, VaildateBearerAuthMiddlewareServiceImpl,
        },
    },
};
use redis::cluster::ClusterClient;
use scylla::client::session::Session;
use std::env;
use std::sync::Arc;
pub struct Container {
    pub hello_service: Arc<dyn HelloService>,
    pub user_service: Arc<dyn UserService>,
    pub validate_bearer_auth_middleware_service: Arc<dyn VaildateBearerAuthMiddlewareService>,
}

impl Container {
    fn get_env(key: &str) -> String {
        env::var(key).unwrap()
    }

    fn get_env_bool(key: &str) -> bool {
        env::var(key).map(|v| v == "true").unwrap_or(false)
    }

    fn get_env_u64(key: &str) -> u64 {
        env::var(key).map(|u| u.parse::<u64>().unwrap()).unwrap()
    }
    // todo split them into fn create_repo, fn create_service, fn create_middleware
    pub async fn new(cache: Arc<ClusterClient>, database: Arc<Session>) -> Self {
        let secret = Self::get_env("CORE_SECRET");
        let cache_ttl = Self::get_env_u64("APPLICATIONS_ORGANIZATION_CACHE_TTL");
        //repo
        let user_database_repository = Arc::new(UserDatabaseRepositoryImpl::new(database.clone()));
        let password_hasher = Arc::new(PasswordHasherImpl::new());
        let user_rule_chacker =
            Arc::new(UserRuleCheckerImpl::new(user_database_repository.clone()));
        let aes_repo = Arc::new(AesGcmCipherImpl::new(secret.as_bytes()));
        let base64_repo = Arc::new(Base64RepositoryImpl);

        let org_db_repo = Arc::new(OrganizationDatabaseRepositoryImpl::new(database.clone()));
        let app_db_repo = Arc::new(ApplicationDatabaseRepositoryImpl::new(database.clone()));
        let apporg_by_client_id_db_repo = Arc::new(
            ApplicationsOrganizationByClientIdDatabaseRepositoryImpl::new(database.clone()),
        );

        let apporg_by_client_id_cache_repo = Arc::new(
            ApplicationsOrganizationByClientIdCacheImpl::new(cache, cache_ttl),
        );
        let apporg_by_client_id_cachelayer_repo =
            Arc::new(ApplicationsOrganizationByClientIdCacheLayerImpl::new(
                apporg_by_client_id_cache_repo,
                apporg_by_client_id_db_repo,
            ));
        let hello_repository: Arc<dyn HelloRepository> = Arc::new(HelloRepositoryImpl::new());
        let user_repository: Arc<dyn UserRepository> = Arc::new(UserRepositoryImpl::new(
            user_database_repository,
            password_hasher,
            user_rule_chacker,
        ));
        let validate_bearer_auth_middleware_repository: Arc<
            dyn VaildateBearerAuthMiddlewareRepository,
        > = Arc::new(VaildateBearerAuthMiddlewareRepositoryImpl::new(
            apporg_by_client_id_cachelayer_repo.clone(),
            base64_repo.clone(),
            aes_repo.clone(),
        ));
        let initial_core_repository = Arc::new(InitialCoreImpl::new(
            aes_repo,
            base64_repo,
            org_db_repo,
            app_db_repo,
            apporg_by_client_id_cachelayer_repo.clone(),
        ));
        let initial_core_service = Arc::new(InitialCoreServiceImpl::new(initial_core_repository));
        let do_gen_core = Self::get_env_bool("CORE_GENRATE_CORE_ORG_APP");
        initial_core_service.lunch(do_gen_core).await;
        //controllers
        let hello_service = Arc::new(HelloServiceImpl {
            repository: hello_repository,
        });
        let user_service = Arc::new(UserServiceImpl {
            repository: user_repository,
        });
        //middlewares
        let validate_bearer_auth_middleware_service =
            Arc::new(VaildateBearerAuthMiddlewareServiceImpl {
                repository: validate_bearer_auth_middleware_repository,
            });
        Container {
            hello_service,
            user_service,
            validate_bearer_auth_middleware_service,
        }
    }
}
