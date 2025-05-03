use crate::domain::{
    errors::repositories_errors::RepositoryResult,
    models::apporg_client_id_models::AppOrgByClientId,
};
use async_trait::async_trait;
use scylla::client::session::Session;
use std::sync::Arc;
pub struct ApplicationsOrganizationByClientIdDatabaseRepositoryImpl {
    pub database: Arc<Session>,
}
impl ApplicationsOrganizationByClientIdDatabaseRepositoryImpl {
    pub fn new(database: Arc<Session>) -> Self {
        Self { database }
    }
}

#[async_trait]
pub trait ApplicationsOrganizationByClientIdDatabaseRepository: Send + Sync {
    async fn create_apporg_by_client_id(&self, apporg: AppOrgByClientId) -> RepositoryResult<()>;
}

#[async_trait]
impl ApplicationsOrganizationByClientIdDatabaseRepository
    for ApplicationsOrganizationByClientIdDatabaseRepositoryImpl
{
    async fn create_apporg_by_client_id(&self, apporg: AppOrgByClientId) -> RepositoryResult<()> {
        let query = "
         INSERT INTO axcelium.applications_organization_by_client_id (
                client_id,
                application_id,
                organization_id,
                encrypted_client_secret,
                organization_name,
                organization_slug,
                application_name,
                application_description,
                contact_email,
                application_config,
                is_active,
                created_at,
                updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);
    ";
        self.database.query_unpaged(query, &apporg).await?;
        Ok(())
    }
}
