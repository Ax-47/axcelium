use crate::{
    domain::errors::repositories_errors::RepositoryResult,
    infrastructure::models::{
        user::{
            CleannedUserModel, FoundUserModel, PaginatedUsersModel, UpdateUserModel, UserModel,
        },
        user_organization::{UpdateUserOrganizationModel, UserOrganizationModel},
    },
};
use async_trait::async_trait;
use scylla::{
    client::session::Session,
    response::PagingState,
    statement::{batch::Batch, Consistency, Statement},
};
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
    async fn find_user(
        &self,
        application_id: Uuid,
        organization_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<Option<CleannedUserModel>>;
    async fn find_all_users_paginated(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        page_size: i32,
        paging_state: Option<Vec<u8>>,
    ) -> RepositoryResult<PaginatedUsersModel>;

    async fn update_user(
        &self,
        user: UpdateUserModel,
        u_org: UpdateUserOrganizationModel,
    ) -> RepositoryResult<()>;
}
#[async_trait]
impl UserDatabaseRepository for UserDatabaseRepositoryImpl {
    async fn create_user(
        &self,
        user: UserModel,
        u_org: UserOrganizationModel,
    ) -> RepositoryResult<()> {
        let mut batch = Batch::default();
        batch.set_consistency(Consistency::Quorum);
        let query1 = r#"
        INSERT INTO axcelium.users (
            user_id, organization_id, application_id,
            username, email, hashed_password,
            created_at, updated_at,
            is_active, is_verified, is_locked, mfa_enabled,last_login,deactivated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,?,?)
    "#;
        batch.append_statement(query1);
        let use_email = user.email.is_some();
        if use_email {
            let query2 = r#"
                INSERT INTO axcelium.users_by_email (
                    email, organization_id, application_id,
                    user_id, username, hashed_password,
                    created_at, updated_at,
                    is_active, is_verified, is_locked,
                    last_login, mfa_enabled, deactivated_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#;
            batch.append_statement(query2);
        }
        let query3 = r#"
        INSERT INTO axcelium.users_by_username (
            username, organization_id, application_id,
            email, user_id, hashed_password,
            created_at, updated_at,
            is_active, is_verified, is_locked,
            last_login, mfa_enabled, deactivated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    "#;
        batch.append_statement(query3);
        let query4 = r#"
        INSERT INTO axcelium.user_organizations (
            organization_id, user_id, role,
            username, user_email,
            organization_name, organization_slug, contact_email,
            joined_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
    "#;
        batch.append_statement(query4);
        let query5 = r#"
        INSERT INTO axcelium.user_organizations_by_user (
            user_id, organization_id, role,
            username, user_email,
            organization_name, organization_slug, contact_email,
            joined_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
    "#;

        batch.append_statement(query5);
        if use_email {
            self.database
                .batch(&batch, ((&user), (&user), (&user), (&u_org), (&u_org)))
                .await?;
        } else {
            self.database
                .batch(&batch, ((&user), (&user), (&u_org), (&u_org)))
                .await?;
        }
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
            SELECT user_id,organization_id,application_id,username,
                email,created_at,updated_at,is_active,is_verified,is_locked,
                last_login,mfa_enabled,deactivated_at
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

    async fn find_user(
        &self,
        application_id: Uuid,
        organization_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<Option<CleannedUserModel>> {
        let query = r#"
            SELECT user_id,organization_id,application_id,username,
                email,created_at,updated_at,is_active,is_verified,is_locked,
                last_login,mfa_enabled,deactivated_at
            FROM axcelium.users
            WHERE organization_id = ? AND application_id = ? AND user_id =?;
        "#;
        let result = self
            .database
            .query_unpaged(query, (organization_id, application_id, user_id))
            .await?
            .into_rows_result()?;

        Ok(result.maybe_first_row::<CleannedUserModel>()?)
    }

    async fn update_user(
        &self,
        user: UpdateUserModel,
        u_org: UpdateUserOrganizationModel,
    ) -> RepositoryResult<()> {
        let mut sets = vec![];
        if user.username.is_some() {
            sets.push("username = ?");
        }

        if user.email.is_some() {
            sets.push("email = ?");
        }

        if user.hashed_password.is_some() {
            sets.push("hashed_password = ?");
        }
        sets.push("updated_at = ?");

        let mut sets2 = vec![];
        if u_org.role.is_some() {
            sets2.push("role = ?");
        }
        if u_org.username.is_some() {
            sets2.push("username = ?");
        }

        if u_org.user_email.is_some() {
            sets2.push("user_email = ?");
        }
        if u_org.organization_name.is_some() {
            sets2.push("organization_name = ?");
        }

        if u_org.organization_slug.is_some() {
            sets2.push("organization_slug = ?");
        }
        if u_org.contact_email.is_some() {
            sets2.push("contact_email = ?");
        }

        sets2.push("updated_at = ?");

        let mut batch = Batch::default();
        batch.set_consistency(Consistency::Quorum);
        let query1 = format!(
            r#"
            UPDATE axcelium.users
            SET {}
            WHERE organization_id = ? AND application_id = ? AND user_id = ?
            "#,
            sets.join(", ")
        );

        batch.append_statement(query1.as_str());
        let use_email = user.email.is_some();
        if use_email {
            let query2 = format!(
                r#"
                UPDATE axcelium.users_by_email
            SET {}
            WHERE organization_id = ? AND application_id = ? AND user_id = ?
            "#,
                sets.join(", ")
            );
            batch.append_statement(query2.as_str());
        }

        let query3 = format!(
            r#"
                UPDATE axcelium.users_by_username
            SET {}
            WHERE organization_id = ? AND application_id = ? AND user_id = ?
            "#,
            sets.join(", ")
        );
        batch.append_statement(query3.as_str());
        let query4 = format!(
            r#"
                UPDATE axcelium.user_organizations
            SET {}
            WHERE organization_id = ? AND application_id = ? AND user_id = ?
            "#,
            sets2.join(", ")
        );

        batch.append_statement(query4.as_str());

        let query5 = format!(
            r#"
                UPDATE axcelium.user_organizations_by_user
            SET {}
            WHERE organization_id = ? AND application_id = ? AND user_id = ?
            "#,
            sets2.join(", ")
        );
        batch.append_statement(query5.as_str());
        self.database
            .batch(&batch, ((&user), (&user), (&user), (&u_org), (&u_org)))
            .await?;
        Ok(())
    }
}
