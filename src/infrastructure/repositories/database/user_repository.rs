use crate::{
    domain::errors::repositories_errors::RepositoryResult,
    infrastructure::models::{
        user::{CleannedUserModel, FoundUserModel, PaginatedUsersModel, UserModel},
        user_organization::UserOrganizationModel,
    },
};
use async_trait::async_trait;
use scylla::{client::session::Session, response::PagingState, statement::Statement};
use std::{ops::ControlFlow, sync::Arc};
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
    async fn create_user(
        &self,
        user: UserModel,
        u_org: UserOrganizationModel,
    ) -> RepositoryResult<()>;
    async fn insert_into_user(&self, user: &UserModel) -> RepositoryResult<()>;
    async fn insert_into_user_by_email(&self, user: &UserModel) -> RepositoryResult<()>;
    async fn insert_into_user_by_username(&self, user: &UserModel) -> RepositoryResult<()>;
    async fn insert_into_user_organizations(
        &self,
        user_org: &UserOrganizationModel,
    ) -> RepositoryResult<()>;
    async fn insert_into_user_organizations_by_user(
        &self,
        user_org: &UserOrganizationModel,
    ) -> RepositoryResult<()>;
    async fn find_user_by_email(
        &self,
        email: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<Option<FoundUserModel>>;

    async fn find_user_by_username(
        &self,
        username: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<Option<FoundUserModel>>;
    async fn find_all_users_paginated(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        page_size: i32,
        paging_state: Option<Vec<u8>>,
    ) -> RepositoryResult<PaginatedUsersModel>;
}
#[async_trait]
impl UserDatabaseRepository for UserDatabaseRepositoryImpl {
    async fn create_user(
        &self,
        user: UserModel,
        u_org: UserOrganizationModel,
    ) -> RepositoryResult<()> {
        let inserts = vec![
            self.insert_into_user(&user),
            self.insert_into_user_by_email(&user),
            self.insert_into_user_by_username(&user),
            self.insert_into_user_organizations(&u_org),
            self.insert_into_user_organizations_by_user(&u_org),
        ];

        for result in futures::future::join_all(inserts).await {
            if let Err(err) = result {
                return Err(err);
            }
        }

        Ok(())
    }

    async fn insert_into_user(&self, user: &UserModel) -> RepositoryResult<()> {
        let query = r#"
            INSERT INTO axcelium.users (
                user_id, organization_id, application_id,
                username, email, hashed_password,
                created_at, updated_at,
                is_active, is_verified, is_locked, mfa_enabled
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;

        self.database
            .query_unpaged(
                query,
                (
                    user.user_id,
                    user.organization_id,
                    user.application_id,
                    user.username.clone(),
                    user.to_entity().prepared_email(),
                    user.hashed_password.clone(),
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

    async fn insert_into_user_by_email(&self, user: &UserModel) -> RepositoryResult<()> {
        if let Some(_) = &user.email {
            let query = r#"
                INSERT INTO axcelium.users_by_email (
                    email, organization_id, application_id,
                    user_id, username, password_hash,
                    created_at, updated_at,
                    is_active, is_verified, is_locked,
                    last_login, mfa_enabled, deactivated_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#;

            self.database.query_unpaged(query, user).await?;
        }

        Ok(())
    }

    async fn insert_into_user_by_username(&self, user: &UserModel) -> RepositoryResult<()> {
        let query = r#"
            INSERT INTO axcelium.users_by_username (
                username, organization_id, application_id,
                email, user_id, password_hash,
                created_at, updated_at,
                is_active, is_verified, is_locked,
                last_login, mfa_enabled, deactivated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;

        self.database.query_unpaged(query, user).await?;
        Ok(())
    }

    async fn insert_into_user_organizations(
        &self,
        user_org: &UserOrganizationModel,
    ) -> RepositoryResult<()> {
        let query = r#"
            INSERT INTO axcelium.user_organizations (
                organization_id, user_id, role,
                username, user_email,
                organization_name, organization_slug, contact_email,
                joined_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;

        self.database.query_unpaged(query, user_org).await?;
        Ok(())
    }

    async fn insert_into_user_organizations_by_user(
        &self,
        user_org: &UserOrganizationModel,
    ) -> RepositoryResult<()> {
        let query = r#"
            INSERT INTO axcelium.user_organizations_by_user (
                user_id, organization_id, role,
                username, user_email,
                organization_name, organization_slug, contact_email,
                joined_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;

        self.database.query_unpaged(query, user_org).await?;
        Ok(())
    }

    async fn find_user_by_email(
        &self,
        email: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<Option<FoundUserModel>> {
        let query = r#"
            SELECT username, user_id, email
            FROM axcelium.users_by_email
            WHERE email = ? AND application_id = ? AND organization_id = ?
        "#;

        let result = self
            .database
            .query_unpaged(query, (email, application_id, organization_id))
            .await?
            .into_rows_result()?;

        Ok(result.maybe_first_row::<FoundUserModel>()?)
    }

    async fn find_user_by_username(
        &self,
        username: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<Option<FoundUserModel>> {
        let query = r#"
            SELECT username, user_id, email
            FROM axcelium.users_by_username
            WHERE username = ? AND application_id = ? AND organization_id = ?
        "#;

        let result = self
            .database
            .query_unpaged(query, (username, application_id, organization_id))
            .await?
            .into_rows_result()?;

        Ok(result.maybe_first_row::<FoundUserModel>()?)
    }

    async fn find_all_users_paginated(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        page_size: i32,
        paging_state_u8: Option<Vec<u8>>,
    ) -> RepositoryResult<PaginatedUsersModel> {
        let query_str = r#"
            SELECT user_id,
                    organization_id,
                    application_id,
                    username,
                    email,
                    created_at,
                    updated_at,
                    is_active,
                    is_verified,
                    is_locked,
                    last_login,
                    mfa_enabled,
                    deactivated_at
            FROM axcelium.users
            WHERE organization_id = ? AND application_id = ?
        "#;

        let paged_prepared = self
            .database
            .prepare(Statement::new(query_str).with_page_size(page_size))
            .await?;
        let paging_state = paging_state_u8
            .map(PagingState::new_from_raw_bytes)
            .unwrap_or_else(PagingState::start);
        let (res, paging_state_response) = self
            .database
            .execute_single_page(
                &paged_prepared,
                &(organization_id, application_id),
                paging_state,
            )
            .await?;

        let users = res
            .into_rows_result()?
            .rows::<CleannedUserModel>()?
            .map(|r: Result<CleannedUserModel, scylla::errors::DeserializationError>| r)
            .collect::<Result<Vec<_>, _>>()?;

        let next_page_state = match paging_state_response.into_paging_control_flow() {
            ControlFlow::Break(()) => None,
            ControlFlow::Continue(state) => state.as_bytes_slice().map(|arc| arc.as_ref().to_vec()),
        };

        Ok(PaginatedUsersModel {
            users,
            paging_state: next_page_state,
        })
    }
}
