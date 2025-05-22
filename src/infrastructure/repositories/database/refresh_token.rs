use crate::{
    domain::errors::repositories_errors::RepositoryResult,
    infrastructure::models::refresh_token::{FoundRefreshTokenModel, RefreshTokenModel},
};
use async_trait::async_trait;
use scylla::{client::session::Session, statement::prepared::PreparedStatement};
use std::sync::Arc;
use uuid::Uuid;

use super::query::refresh_token::{
    INSERT_REFRESH_TOKEN, QUERY_REFRESH_TOKEN, REVOKE_REFRESH_TOKEN, UPDATE_REFRESH_TOKEN,
};
pub struct RefreshTokenDatabaseRepositoryImpl {
    pub database: Arc<Session>,
    insert_refresh_token: PreparedStatement,
    prepared_update_refresh_token: PreparedStatement,
    query_refresh_token: PreparedStatement,
    revoke_refresh_token: PreparedStatement,
}

impl RefreshTokenDatabaseRepositoryImpl {
    pub async fn new(database: Arc<Session>) -> Self {
        let insert_refresh_token = database.prepare(INSERT_REFRESH_TOKEN).await.unwrap();
        let prepared_update_refresh_token = database.prepare(UPDATE_REFRESH_TOKEN).await.unwrap();
        let query_refresh_token = database.prepare(QUERY_REFRESH_TOKEN).await.unwrap();
        let revoke_refresh_token = database.prepare(REVOKE_REFRESH_TOKEN).await.unwrap();
        Self {
            database,
            insert_refresh_token,
            prepared_update_refresh_token,
            query_refresh_token,
            revoke_refresh_token,
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
            .execute_unpaged(&self.prepared_update_refresh_token, rt)
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
