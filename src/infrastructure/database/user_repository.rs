use crate::domain::{
    errors::repositories_errors::RepositoryResult,
    models::user_models::{CreatedUser, User, UserOrganization},
};
use async_trait::async_trait;
use scylla::client::session::Session;
use std::sync::Arc;
use uuid::Uuid;
pub struct UserDatabaseRepositoryImpl {
    pub database: Arc<Session>,
}

impl UserDatabaseRepositoryImpl {
    pub fn new(database: Arc<Session>) -> Self {
        Self { database }
    }
}

#[async_trait]
pub trait UserDatabaseRepository: Send + Sync {
    async fn insert_into_user(&self, user: &User) -> RepositoryResult<()>;
    async fn insert_into_user_by_email(&self, user: &User) -> RepositoryResult<()>;
    async fn insert_into_user_by_username(&self, user: &User) -> RepositoryResult<()>;
    async fn insert_into_user_organizations(
        &self,
        user_org: &UserOrganization,
    ) -> RepositoryResult<()>;
    async fn insert_into_user_organizations_by_user(
        &self,
        user_org: &UserOrganization,
    ) -> RepositoryResult<()>;
    async fn find_user_by_email(
        &self,
        email: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<Option<CreatedUser>>;

    async fn find_user_by_username(
        &self,
        username: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<Option<CreatedUser>>;
}

#[async_trait]
impl UserDatabaseRepository for UserDatabaseRepositoryImpl {
    async fn insert_into_user(&self, user: &User) -> RepositoryResult<()> {
        let query = "INSERT INTO axcelium.users (
            user_id, organization_id, application_id,
            username, email, password_hash,
            created_at, updated_at,
            is_active, is_verified, is_locked, mfa_enabled
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
        self.database
            .query_unpaged(
                query,
                (
                    user.user_id,
                    user.organization_id,
                    user.application_id,
                    user.username.clone(),
                    user.prepared_email(),
                    user.password_hash.clone(),
                    user.created_at,
                    user.updated_at,
                    user.is_active,
                    user.is_verified,
                    user.is_locked,
                    user.mfa_enabled,
                ),
            )
            .await?;

        Ok(())
    }

    async fn insert_into_user_by_email(&self, user: &User) -> RepositoryResult<()> {
        if user.email.is_some() {
            let query = "INSERT INTO axcelium.users_by_email (
                    email, organization_id, application_id,
                    user_id, username, password_hash,
                    created_at, updated_at,
                    is_active, is_verified, is_locked,
                    last_login, mfa_enabled, deactivated_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
            self.database.query_unpaged(query, &user).await?;
        }
        Ok(())
    }

    async fn insert_into_user_by_username(&self, user: &User) -> RepositoryResult<()> {
        let query = "INSERT INTO axcelium.users_by_username (
                username, organization_id, application_id,
                email, user_id, password_hash,
                created_at, updated_at,
                is_active, is_verified, is_locked,
                last_login, mfa_enabled, deactivated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
        self.database.query_unpaged(query, &user).await?;
        Ok(())
    }

    async fn insert_into_user_organizations(
        &self,
        user_org: &UserOrganization,
    ) -> RepositoryResult<()> {
        let query = "INSERT INTO axcelium.user_organizations (
            organization_id, user_id, role,
            username, user_email,
            organization_name, organization_slug, contact_email,
            joined_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)";
        self.database.query_unpaged(query, &user_org).await?;
        Ok(())
    }

    async fn insert_into_user_organizations_by_user(
        &self,
        user_org: &UserOrganization,
    ) -> RepositoryResult<()> {
        let query = "INSERT INTO axcelium.user_organizations_by_user (
            user_id, organization_id, role,
            username, user_email,
            organization_name, organization_slug, contact_email,
            joined_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)";
        self.database.query_unpaged(query, &user_org).await?;
        Ok(())
    }
    async fn find_user_by_email(
        &self,
        email: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<Option<CreatedUser>> {
        let query = "SELECT username,user_id,email FROM axcelium.users_by_email \
                    WHERE email = ? AND application_id = ? AND organization_id = ?";

        let result = self
            .database
            .query_unpaged(query, (email, application_id, organization_id))
            .await?
            .into_rows_result()?;

        let row = result.maybe_first_row::<CreatedUser>()?;
        Ok(row)
    }

    async fn find_user_by_username(
        &self,
        username: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<Option<CreatedUser>> {
        let query = "SELECT username,user_id,email FROM axcelium.users_by_username \
                WHERE username = ? AND application_id = ? AND organization_id = ?";
        let result = self
            .database
            .query_unpaged(query, (username, application_id, organization_id))
            .await?
            .into_rows_result()?;

        let row = result.maybe_first_row::<CreatedUser>()?;
        Ok(row)
    }
}
