use crate::{
    domain::errors::repositories_errors::RepositoryResult,
    infrastructure::models::refresh_token::{
        FoundRefreshTokenModel, FoundRefreshTokenModelByUser, PaginatedRefreshTokensByUserModel,
        RefreshTokenModel, UpdateRefreshTokenQuery,
    },
};
use async_trait::async_trait;
use scylla::{
    client::session::Session, response::PagingState, statement::prepared::PreparedStatement,
};
use std::{ops::ControlFlow, sync::Arc};
use uuid::Uuid;

use super::query::refresh_token::{
    FIND_REFRESH_TOKEN_BY_USER_PAGINATED, INSERT_REFRESH_TOKEN, QUERY_REFRESH_TOKEN,
    REVOKE_REFRESH_TOKEN, UPDATE_REFRESH_TOKEN,
};
pub struct RefreshTokenDatabaseRepositoryImpl {
    pub database: Arc<Session>,
    insert_refresh_token: PreparedStatement,
    prepared_update_refresh_token: PreparedStatement,
    query_refresh_token: PreparedStatement,
    revoke_refresh_token: PreparedStatement,
    find_refresh_token_by_user: PreparedStatement,
}

impl RefreshTokenDatabaseRepositoryImpl {
    pub async fn new(database: Arc<Session>) -> Self {
        let insert_refresh_token = database.prepare(INSERT_REFRESH_TOKEN).await.unwrap();
        let prepared_update_refresh_token = database.prepare(UPDATE_REFRESH_TOKEN).await.unwrap();
        let query_refresh_token = database.prepare(QUERY_REFRESH_TOKEN).await.unwrap();
        let revoke_refresh_token = database.prepare(REVOKE_REFRESH_TOKEN).await.unwrap();
        let find_refresh_token_by_user = database
            .prepare(FIND_REFRESH_TOKEN_BY_USER_PAGINATED)
            .await
            .unwrap();
        Self {
            database,
            insert_refresh_token,
            prepared_update_refresh_token,
            query_refresh_token,
            revoke_refresh_token,
            find_refresh_token_by_user,
        }
    }
}

#[async_trait]
pub trait RefreshTokenDatabaseRepository: Send + Sync {
    async fn create_refresh_token(&self, rt: &RefreshTokenModel) -> RepositoryResult<()>;

    async fn update_refresh_token(&self, rt: &RefreshTokenModel) -> RepositoryResult<()>;
    async fn find_refresh_token(
        &self,
        org_id: Uuid,
        app_id: Uuid,
        token_id: Uuid,
        token_version: &String,
    ) -> RepositoryResult<Option<FoundRefreshTokenModel>>;

    async fn revoke_refresh_token(
        &self,
        org_id: Uuid,
        app_id: Uuid,
        token_id: Uuid,
    ) -> RepositoryResult<()>;
    async fn find_refresh_token_by_user(
        &self,
        org_id: Uuid,
        app_id: Uuid,
        user_id: Uuid,
        page_size: i32,
        paging_state_u8: Option<Vec<u8>>,
    ) -> RepositoryResult<PaginatedRefreshTokensByUserModel>;
}

#[async_trait]
impl RefreshTokenDatabaseRepository for RefreshTokenDatabaseRepositoryImpl {
    async fn create_refresh_token(&self, rt: &RefreshTokenModel) -> RepositoryResult<()> {
        self.database
            .execute_unpaged(&self.insert_refresh_token, rt)
            .await?;
        Ok(())
    }

    async fn update_refresh_token(&self, rt: &RefreshTokenModel) -> RepositoryResult<()> {
        self.database
            .execute_unpaged(
                &self.prepared_update_refresh_token,
                UpdateRefreshTokenQuery::from(rt),
            )
            .await?;
        Ok(())
    }
    async fn find_refresh_token(
        &self,
        org_id: Uuid,
        app_id: Uuid,
        token_id: Uuid,
        token_version: &String,
    ) -> RepositoryResult<Option<FoundRefreshTokenModel>> {
        let result = self
            .database
            .execute_unpaged(
                &self.query_refresh_token,
                (org_id, app_id, token_id, token_version),
            )
            .await?
            .into_rows_result()?;

        Ok(result.maybe_first_row::<FoundRefreshTokenModel>()?)
    }

    async fn find_refresh_token_by_user(
        &self,
        org_id: Uuid,
        app_id: Uuid,
        user_id: Uuid,
        page_size: i32,
        paging_state_u8: Option<Vec<u8>>,
    ) -> RepositoryResult<PaginatedRefreshTokensByUserModel> {
        let mut prepared = self.find_refresh_token_by_user.clone();
        prepared.set_page_size(page_size);
        let paging_state = paging_state_u8
            .map(PagingState::new_from_raw_bytes)
            .unwrap_or_else(PagingState::start);
        let (res, paging_state_response) = self
            .database
            .execute_single_page(&prepared, &(org_id, app_id, user_id), paging_state)
            .await?;
        let refresh_tokens = res
            .into_rows_result()?
            .rows::<FoundRefreshTokenModelByUser>()?
            .map(|r: Result<FoundRefreshTokenModelByUser, scylla::errors::DeserializationError>| r)
            .collect::<Result<Vec<_>, _>>()?;
        let next_page_state = match paging_state_response.into_paging_control_flow() {
            ControlFlow::Break(()) => None,
            ControlFlow::Continue(state) => state.as_bytes_slice().map(|arc| arc.as_ref().to_vec()),
        };
        Ok(PaginatedRefreshTokensByUserModel {
            refresh_tokens,
            paging_state: next_page_state,
        })
    }
    async fn revoke_refresh_token(
        &self,
        org_id: Uuid,
        app_id: Uuid,
        token_id: Uuid,
    ) -> RepositoryResult<()> {
        self.database
            .execute_unpaged(&self.revoke_refresh_token, (org_id, app_id, token_id))
            .await?;
        Ok(())
    }
}
