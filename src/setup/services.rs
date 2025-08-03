use std::sync::Arc;

use crate::application::services::{
    cdc::{
        printer::{PrinterConsumerService, PrinterConsumerServiceImpl},
        replicator::{ReplicatorConsumerService, ReplicatorConsumerServiceImpl},
    },
    hello_service::{HelloService, HelloServiceImpl},
    initial_core_service::{InitialCoreService, InitialCoreServiceImpl},
    refresh_token::{
        create::{CreateRefreshTokenService, CreateRefreshTokenServiceImpl},
        get::{GetRefreshTokenService, GetRefreshTokenServiceImpl},
        revoke::{RevokeRefreshTokenService, RevokeRefreshTokenServiceImpl},
        rotate::{RotateRefreshTokenService, RotateRefreshTokenServiceImpl},
    },
    roles::{
        assign::{AssignService, AssignServiceImpl},
        create_roles::{CreateRoleService, CreateRoleServiceImpl},
        delete_role::{DeleteRoleService, DeleteRoleServiceImpl},
        get_role_by_app::{GetRoleByAppService, GetRoleByAppServiceImpl},
        get_roles_by_app::{GetRolesByAppService, GetRolesByAppServiceImpl},
        get_users_by_role::{GetUsersByRoleService, GetUsersByRoleServiceImpl},
        update_role::{UpdateRoleService, UpdateRoleServiceImpl},
    },
    users::{
        ban_user::{BanUserService, BanUserServiceImpl},
        create::{CreateUserService, CreateUserServiceImpl},
        delete::{DeleteUserService, DeleteUserServiceImpl},
        disable_mfa_user::{DisableMFAUserService, DisableMFAUserServiceImpl},
        get_user::{GetUserService, GetUserServiceImpl},
        get_user_count::{GetUserCountService, GetUserCountServiceImpl},
        get_users::{GetUsersService, GetUsersServiceImpl},
        unban_user::{UnbanUserService, UnbanUserServiceImpl},
        update_user::{UpdateUserService, UpdateUserServiceImpl},
    },
};
use tokio::sync::Mutex;

use super::repositories::Repositories;

pub fn create_hello_service() -> Arc<dyn HelloService> {
    Arc::new(HelloServiceImpl {})
}

pub fn create_create_user_service(repos: &Repositories) -> Arc<dyn CreateUserService> {
    Arc::new(CreateUserServiceImpl {
        repository: repos.create_user_repo.clone(),
    })
}

pub fn create_get_users_service(repos: &Repositories) -> Arc<dyn GetUsersService> {
    Arc::new(GetUsersServiceImpl {
        repository: repos.get_users_repo.clone(),
    })
}
pub fn create_get_user_service(repos: &Repositories) -> Arc<dyn GetUserService> {
    Arc::new(GetUserServiceImpl {
        repository: repos.get_user_repo.clone(),
    })
}

pub fn create_update_user_service(repos: &Repositories) -> Arc<dyn UpdateUserService> {
    Arc::new(UpdateUserServiceImpl {
        repository: repos.update_user_repo.clone(),
    })
}

pub fn create_delete_user_service(repos: &Repositories) -> Arc<dyn DeleteUserService> {
    Arc::new(DeleteUserServiceImpl {
        repository: repos.del_user_repo.clone(),
    })
}

pub fn create_get_user_count_service(repos: &Repositories) -> Arc<dyn GetUserCountService> {
    Arc::new(GetUserCountServiceImpl {
        repository: repos.get_user_count_repo.clone(),
    })
}

pub fn create_ban_user_service(repos: &Repositories) -> Arc<dyn BanUserService> {
    Arc::new(BanUserServiceImpl {
        repository: repos.ban_user_repo.clone(),
    })
}

pub fn create_unban_user_service(repos: &Repositories) -> Arc<dyn UnbanUserService> {
    Arc::new(UnbanUserServiceImpl {
        repository: repos.unban_user_repo.clone(),
    })
}

pub fn create_disble_mfa_user_service(repos: &Repositories) -> Arc<dyn DisableMFAUserService> {
    Arc::new(DisableMFAUserServiceImpl {
        repository: repos.disable_mfa_user_repo.clone(),
    })
}

pub fn create_create_refresh_token_service(
    repos: &Repositories,
) -> Arc<dyn CreateRefreshTokenService> {
    Arc::new(CreateRefreshTokenServiceImpl {
        repository: repos.create_refresh_token_repo.clone(),
    })
}

pub fn create_rotate_refresh_token_service(
    repos: &Repositories,
) -> Arc<dyn RotateRefreshTokenService> {
    Arc::new(RotateRefreshTokenServiceImpl {
        repository: repos.rotate_refresh_token_repo.clone(),
    })
}

pub fn create_revoke_refresh_token_service(
    repos: &Repositories,
) -> Arc<dyn RevokeRefreshTokenService> {
    Arc::new(RevokeRefreshTokenServiceImpl {
        repository: repos.revoke_refresh_token_repo.clone(),
    })
}

pub fn create_get_refresh_tokens_by_user_service(
    repos: &Repositories,
) -> Arc<dyn GetRefreshTokenService> {
    Arc::new(GetRefreshTokenServiceImpl {
        repository: repos.get_refresh_tokens_by_user.clone(),
    })
}
pub fn create_create_role_service(repos: &Repositories) -> Arc<dyn CreateRoleService> {
    Arc::new(CreateRoleServiceImpl {
        repository: repos.create_role_repo.clone(),
    })
}

pub fn create_get_role_by_app_service(repos: &Repositories) -> Arc<dyn GetRoleByAppService> {
    Arc::new(GetRoleByAppServiceImpl {
        repository: repos.get_role_by_app_repo.clone(),
    })
}

pub fn create_get_roles_by_app_service(repos: &Repositories) -> Arc<dyn GetRolesByAppService> {
    Arc::new(GetRolesByAppServiceImpl {
        repository: repos.get_roles_by_app_repo.clone(),
    })
}

pub fn create_get_users_by_role_service(repos: &Repositories) -> Arc<dyn GetUsersByRoleService> {
    Arc::new(GetUsersByRoleServiceImpl {
        repository: repos.get_users_by_role_repo.clone(),
    })
}

pub fn create_update_role_service(repos: &Repositories) -> Arc<dyn UpdateRoleService> {
    Arc::new(UpdateRoleServiceImpl {
        repository: repos.update_role_repo.clone(),
    })
}

pub fn create_delete_role_service(repos: &Repositories) -> Arc<dyn DeleteRoleService> {
    Arc::new(DeleteRoleServiceImpl {
        repository: repos.delete_role_repo.clone(),
    })
}

pub fn create_assign_service(repos: &Repositories) -> Arc<dyn AssignService> {
    Arc::new(AssignServiceImpl {
        repository: repos.assign_repo.clone(),
    })
}

pub fn create_printer_service(repos: &Repositories) -> Arc<Mutex<dyn PrinterConsumerService>> {
    Arc::new(Mutex::new(PrinterConsumerServiceImpl::new(
        repos.printer_repo.clone(),
    )))
}

pub fn create_replicator_service(
    repos: &Repositories,
) -> Arc<Mutex<dyn ReplicatorConsumerService>> {
    Arc::new(Mutex::new(ReplicatorConsumerServiceImpl::new(
        repos.replicator_repo.clone(),
    )))
}
pub fn create_init_service(repos: &Repositories) -> Arc<dyn InitialCoreService> {
    Arc::new(InitialCoreServiceImpl {
        repository: repos.core_repo.clone(),
    })
}
