use crate::{
    domain::errors::repositories_errors::RepositoryResult,
    infrastructure::models::application::AppcalitionModel,
};
use async_trait::async_trait;
use scylla::client::session::Session;
use std::sync::Arc;
pub struct ApplicationDatabaseRepositoryImpl {
    pub database: Arc<Session>,
}

impl ApplicationDatabaseRepositoryImpl {
    pub fn new(database: Arc<Session>) -> Self {
        Self { database }
    }
}

#[async_trait]
pub trait ApplicationDatabaseRepository: Send + Sync {
    async fn create_application(&self, app: AppcalitionModel) -> RepositoryResult<()>;
}

#[async_trait]
impl ApplicationDatabaseRepository for ApplicationDatabaseRepositoryImpl {
    async fn create_application(&self, app: AppcalitionModel) -> RepositoryResult<()> {
        let query = "
        INSERT INTO axcelium.applications (
            organization_id,
            application_id,
            name,
            description,
            client_id,
            encrypted_client_secret,
            config,
            created_at,
            updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);
    ";
        self.database.query_unpaged(query, &app).await?;
        Ok(())
    }
}
