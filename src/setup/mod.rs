use crate::{
    application::{
        middlewares::bearer_auth::ValidateBearerAuth,
        services::{
            hello_service::HelloService,
            refresh_token::{
                create::CreateRefreshTokenService, get::GetRefreshTokenService,
                revoke::RevokeRefreshTokenService, rotate::RotateRefreshTokenService,
            },
            roles::{create_roles::CreateRoleService, get_role_by_app::GetRoleByAppService},
            users::{
                ban_user::BanUserService, create::CreateUserService, delete::DeleteUserService,
                disable_mfa_user::DisableMFAUserService, get_user::GetUserService,
                get_user_count::GetUserCountService, get_users::GetUsersService,
                unban_user::UnbanUserService, update_user::UpdateUserService,
            },
        },
    },
    config,
};
use redis::Client;
use scylla::client::session::Session;
use std::sync::Arc;
mod middlewares;
mod repositories;
mod services;
pub struct Container {
    pub hello_service: Arc<dyn HelloService>,
    pub create_user_service: Arc<dyn CreateUserService>,
    pub get_users_service: Arc<dyn GetUsersService>,
    pub get_user_service: Arc<dyn GetUserService>,
    pub update_user_service: Arc<dyn UpdateUserService>,
    pub del_user_service: Arc<dyn DeleteUserService>,
    pub validate_bearer_auth_middleware_service: Arc<ValidateBearerAuth>,
    pub get_user_count_service: Arc<dyn GetUserCountService>,
    pub ban_user_count_service: Arc<dyn BanUserService>,
    pub unban_user_count_service: Arc<dyn UnbanUserService>,
    pub disable_mfa_user_service: Arc<dyn DisableMFAUserService>,
    pub create_refresh_token_service: Arc<dyn CreateRefreshTokenService>,
    pub rotate_refresh_token_service: Arc<dyn RotateRefreshTokenService>,
    pub revoke_refresh_token_service: Arc<dyn RevokeRefreshTokenService>,
    pub get_refresh_tokens_by_user_service: Arc<dyn GetRefreshTokenService>,
    pub create_role_service: Arc<dyn CreateRoleService>,
    pub get_role_by_role_id_service: Arc<dyn GetRoleByAppService>,
}

impl Container {
    pub async fn new(cfg: config::Config, cache: Arc<Client>, database: Arc<Session>) -> Self {
        let secret = cfg.core.secret.clone();
        let cache_ttl = cfg.core.cache_ttl.clone();
        let (repos, core_service) =
            repositories::create_all(database.clone(), cache, &secret, cache_ttl).await;

        core_service.lunch(cfg).await;

        let hello_service = services::create_hello_service();
        let create_user_service = services::create_create_user_service(&repos);
        let get_users_service = services::create_get_users_service(&repos);
        let get_user_service = services::create_get_user_service(&repos);
        let update_user_service = services::create_update_user_service(&repos);
        let del_user_service = services::create_delete_user_service(&repos);
        let get_user_count_service = services::create_get_user_count_service(&repos);
        let ban_user_count_service = services::create_ban_user_service(&repos);
        let unban_user_count_service = services::create_unban_user_service(&repos);
        let disable_mfa_user_service = services::create_disble_mfa_user_service(&repos);
        let create_refresh_token_service = services::create_create_refresh_token_service(&repos);
        let rotate_refresh_token_service = services::create_rotate_refresh_token_service(&repos);
        let revoke_refresh_token_service = services::create_revoke_refresh_token_service(&repos);
        let get_refresh_tokens_by_user_service =
            services::create_get_refresh_tokens_by_user_service(&repos);
        let get_role_by_role_id_service = services::create_get_role_by_app_service(&repos);
        let validate_bearer_auth_middleware_service = Arc::new(ValidateBearerAuth::new(
            middlewares::create_validate_bearer_auth_service(&repos),
        ));
        let create_role_service = services::create_create_role_service(&repos);

        Self {
            hello_service,
            create_user_service,
            get_users_service,
            get_user_service,
            update_user_service,
            validate_bearer_auth_middleware_service,
            del_user_service,
            get_user_count_service,
            ban_user_count_service,
            unban_user_count_service,
            disable_mfa_user_service,
            create_refresh_token_service,
            rotate_refresh_token_service,
            revoke_refresh_token_service,
            get_refresh_tokens_by_user_service,
            create_role_service,
            get_role_by_role_id_service,
        }
    }
}
