use crate::{
    application::{
        dto::response::user::GetUsersResponse, repositories::users::get_users::GetUsersRepository,
    },
    domain::errors::repositories_errors::RepositoryResult,
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetUsersServiceImpl {
    pub repository: Arc<dyn GetUsersRepository>,
}
impl GetUsersServiceImpl {
    pub fn new(repository: Arc<dyn GetUsersRepository>) -> Self {
        Self { repository }
    }
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GetUsersService: 'static + Sync + Send {
    async fn execute(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        page_size: i32,
        paging_state_u8: Option<String>,
    ) -> RepositoryResult<GetUsersResponse>;
}
#[async_trait]
impl GetUsersService for GetUsersServiceImpl {
    async fn execute(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        page_size: i32,
        paging_state: Option<String>,
    ) -> RepositoryResult<GetUsersResponse> {
        let paging_state_u8 = match paging_state {
            Some(state) => Some(self.repository.base64_to_bytes(state)?),
            None => None,
        };
        let users = self
            .repository
            .find_all_users_paginated(organization_id, application_id, page_size, paging_state_u8)
            .await?;
        let encrypted_paging_state = match users.paging_state {
            Some(state) => Some(self.repository.bytes_to_base64(state)),
            None => None,
        };
        Ok(GetUsersResponse {
            users: users.users,
            paging_state: encrypted_paging_state,
        })
    }
}
