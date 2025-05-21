use crate::{
    domain::errors::repositories_errors::RepositoryResult,
    infrastructure::models::refresh_token::RefreshTokenModel,
};
use async_trait::async_trait;
use scylla::{client::session::Session, statement::prepared::PreparedStatement};
use std::sync::Arc;

use super::query::refresh_token::INSERT_REFRESH_TOKEN;
pub struct RefreshTokenDatabaseRepositoryImpl {
    pub database: Arc<Session>,
    insert_refresh_token: PreparedStatement,
}

impl RefreshTokenDatabaseRepositoryImpl {
    pub async fn new(database: Arc<Session>) -> Self {
        let insert_refresh_token = database.prepare(INSERT_REFRESH_TOKEN).await.unwrap();
        Self {
            database,
            insert_refresh_token,
        }
    }
}

#[async_trait]
pub trait RefreshTokenDatabaseRepository: Send + Sync {
    async fn create_refresh_token(&self, rf: RefreshTokenModel) -> RepositoryResult<()>;
}

#[async_trait]
impl RefreshTokenDatabaseRepository for RefreshTokenDatabaseRepositoryImpl {
    async fn create_refresh_token(&self, rt: RefreshTokenModel) -> RepositoryResult<()> {
        self.database.execute_unpaged(&self.insert_refresh_token, &rt).await?;
        Ok(())
    }
}
