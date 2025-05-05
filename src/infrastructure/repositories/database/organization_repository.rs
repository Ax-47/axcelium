use crate::domain::{
    errors::repositories_errors::RepositoryResult, models::organization_models::Organization,
};
use async_trait::async_trait;
use scylla::client::session::Session;
use std::sync::Arc;
use uuid::Uuid;
pub struct OrganizationDatabaseRepositoryImpl {
    pub database: Arc<Session>,
}

impl OrganizationDatabaseRepositoryImpl {
    pub fn new(database: Arc<Session>) -> Self {
        Self { database }
    }
}

#[async_trait]
pub trait OrganizationDatabaseRepository: Send + Sync {
    async fn find_organization(&self, name: String) -> RepositoryResult<Option<Uuid>>;
    async fn create_organization(&self, org: Organization) -> RepositoryResult<()>;
}

#[async_trait]
impl OrganizationDatabaseRepository for OrganizationDatabaseRepositoryImpl {
    /// # to slow
    ///
    /// ## use only initial
    async fn find_organization(&self, name: String) -> RepositoryResult<Option<Uuid>> {
        let query = "
    SELECT organization_id FROM axcelium.organizations
    WHERE name = ? ALLOW FILTERING;";
        let res = self
            .database
            .query_unpaged(query, (name,))
            .await?
            .into_rows_result()?
            .maybe_first_row::<(Uuid,)>()?
            .map(|row| row.0);
        Ok(res)
    }
    async fn create_organization(&self, org: Organization) -> RepositoryResult<()> {
        let query = "
        INSERT INTO axcelium.organizations (
            organization_id,
            name,
            slug,
            contact_email,
            is_active,
            created_at,
            updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?);
    ";
        self.database.query_unpaged(query, &org).await?;
        Ok(())
    }
}
