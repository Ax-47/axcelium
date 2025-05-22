use crate::application::dto::response::refresh_token::GetRefreshTokensResponse;
use crate::application::repositories::refresh_tokens::get::GetRefreshTokenRepository;
use crate::domain::entities::apporg_client_id::CleanAppOrgByClientId;
use crate::domain::errors::repositories_errors::RepositoryResult;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

pub struct GetRefreshTokenServiceImpl {
    pub repository: Arc<dyn GetRefreshTokenRepository>,
}
impl GetRefreshTokenServiceImpl {
    pub fn new(repository: Arc<dyn GetRefreshTokenRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
pub trait GetRefreshTokenService: Send + Sync {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        user_id: Uuid,
        page_size: i32,
        paging_state: Option<String>,
    ) -> RepositoryResult<GetRefreshTokensResponse>;
}

#[async_trait]
impl GetRefreshTokenService for GetRefreshTokenServiceImpl {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        user_id: Uuid,
        page_size: i32,
        paging_state: Option<String>,
    ) -> RepositoryResult<GetRefreshTokensResponse> {
        let paging_state_u8 = match paging_state {
            Some(state) => Some(self.repository.base64_to_bytes(state)?),
            None => None,
        };
        let refresh_tokens = self
            .repository
            .get_refresh_tokens_by_user(
                c_apporg.organization_id,
                c_apporg.application_id,
                user_id,
                page_size,
                paging_state_u8,
            )
            .await?;

        let encrypted_paging_state = match refresh_tokens.paging_state {
            Some(state) => Some(self.repository.bytes_to_base64(state)),
            None => None,
        };

        Ok(GetRefreshTokensResponse {
            refresh_tokens: refresh_tokens.refresh_tokens,
            paging_state: encrypted_paging_state,
        })
    }
}
