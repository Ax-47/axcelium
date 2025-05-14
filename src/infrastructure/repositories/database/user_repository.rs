use crate::{
    domain::errors::repositories_errors::{RepositoryError, RepositoryResult},
    infrastructure::models::{
        user::{
            CleannedUserModel, FoundUserModel, PaginatedUsersModel, UpdateUserModel, UserModel,
        },
        user_organization::UserOrganizationModel,
    },
};
use async_trait::async_trait;
use scylla::{
    client::session::Session,
    response::PagingState,
    statement::{batch::Batch, Consistency, Statement},
    value::CqlValue,
};
use std::{collections::HashMap, ops::ControlFlow, sync::Arc};
use uuid::Uuid;

use super::query::users::{
    update_user_org_by_user_query, update_user_org_query, update_users_query, DELETE_USERS_BY_EMAIL, DELETE_USERS_BY_USERNAME, INSERT_USER, INSERT_USERS_BY_EMAIL_SEC, INSERT_USERS_BY_USERNAME_SEC, INSERT_USER_BY_EMAIL, INSERT_USER_BY_USERNAME, INSERT_USER_ORGANIZATION, INSERT_USER_ORG_BY_USER, QUERY_FIND_ALL_USERS_PAGINATED, QUERY_FIND_RAW_USER, QUERY_FIND_USER, QUERY_FIND_USER_BY_EMAIL, QUERY_FIND_USER_BY_USERNAME
};
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

    async fn find_raw_user(
        &self,
        application_id: Uuid,
        organization_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<Option<UserModel>>;
    async fn update_user(
        &self,
        user: UpdateUserModel,
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<()>;

    async fn delete_user(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
        user: CleannedUserModel,
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
        batch.append_statement(INSERT_USER);
        let use_email = user.email.is_some();
        if use_email {
            batch.append_statement(INSERT_USER_BY_EMAIL);
        }
        batch.append_statement(INSERT_USER_BY_USERNAME);
        batch.append_statement(INSERT_USER_ORGANIZATION);
        batch.append_statement(INSERT_USER_ORG_BY_USER);
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
        let result = self
            .database
            .query_unpaged(
                QUERY_FIND_USER_BY_EMAIL,
                (email, application_id, organization_id),
            )
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
        let result = self
            .database
            .query_unpaged(
                QUERY_FIND_USER_BY_USERNAME,
                (username, application_id, organization_id),
            )
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
        let paged_prepared = self
            .database
            .prepare(Statement::new(QUERY_FIND_ALL_USERS_PAGINATED).with_page_size(page_size))
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
        let result = self
            .database
            .query_unpaged(QUERY_FIND_USER, (organization_id, application_id, user_id))
            .await?
            .into_rows_result()?;

        Ok(result.maybe_first_row::<CleannedUserModel>()?)
    }

    async fn find_raw_user(
        &self,
        application_id: Uuid,
        organization_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<Option<UserModel>> {
        let result = self
            .database
            .query_unpaged(
                QUERY_FIND_RAW_USER,
                (organization_id, application_id, user_id),
            )
            .await?
            .into_rows_result()?;

        Ok(result.maybe_first_row::<UserModel>()?)
    }
    async fn update_user(
        &self,
        user: UpdateUserModel,
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<()> {
        let mut batch = Batch::default();
        batch.set_consistency(Consistency::Quorum);
        let Some(mut fetched_user) = self
            .find_raw_user(application_id, organization_id, user_id)
            .await?
        else {
            return Err(RepositoryError::new("not found user".to_string(), 400));
        }; // BL
        let mut userbind: HashMap<&str, CqlValue> = HashMap::new();
        let mut userorgbind: HashMap<&str, CqlValue> = HashMap::new();
        let mut delusernamebind: HashMap<&str, CqlValue> = HashMap::new();
        let mut set_clauses_main: Vec<&'static str> = vec![];
        let mut set_clauses_org: Vec<&'static str> = vec![];
        let mut binds: Vec<&HashMap<&str, CqlValue>> = vec![];

        let has_email = user.email.is_some();
        if let Some(ref u) = user.username {
            set_clauses_main.push("username = :username");
            set_clauses_org.push("username = :username");
            fetched_user.username = u.clone();
            userbind.insert("username", CqlValue::Text(u.clone()));
            userorgbind.insert("username", CqlValue::Text(u.clone()));
        }
        if let Some(e) = user.email {
            set_clauses_main.push("email = :email");
            set_clauses_org.push("user_email = :user_email");
            fetched_user.email = Some(e.clone());
            userbind.insert("email", CqlValue::Text(e.clone()));
            userorgbind.insert("user_email", CqlValue::Text(e));
        }
        if let Some(p) = user.hashed_password {
            set_clauses_main.push("hashed_password = :hashed_password");
            fetched_user.hashed_password = p.clone();
            userbind.insert("hashed_password", CqlValue::Text(p));
        }

        userbind.insert("updated_at", CqlValue::Timestamp(user.updated_at));
        userbind.insert("organization_id", CqlValue::Uuid(organization_id));
        userbind.insert("application_id", CqlValue::Uuid(application_id));
        userbind.insert("user_id", CqlValue::Uuid(user_id));

        delusernamebind.insert("organization_id", CqlValue::Uuid(organization_id));
        delusernamebind.insert("application_id", CqlValue::Uuid(application_id));
        delusernamebind.insert("username", CqlValue::Text(fetched_user.username.clone()));

        userorgbind.insert("organization_id", CqlValue::Uuid(organization_id));
        userorgbind.insert("user_id", CqlValue::Uuid(user_id));

        batch.append_statement(update_users_query(&set_clauses_main).as_str());
        binds.push(&userbind);
        let insert_user_bind = fetched_user.to_bind_map();

        if has_email {
            batch.append_statement(DELETE_USERS_BY_EMAIL);
            binds.push(&delusernamebind);
            batch.append_statement(INSERT_USERS_BY_EMAIL_SEC);
            binds.push(&insert_user_bind);
        }
        if user.username.is_some() {
            batch.append_statement(DELETE_USERS_BY_USERNAME);
            binds.push(&delusernamebind);
            batch.append_statement(INSERT_USERS_BY_USERNAME_SEC);
            binds.push(&insert_user_bind);
        }

        batch.append_statement(update_user_org_query(&set_clauses_org).as_str());
        binds.push(&userorgbind);

        batch.append_statement(update_user_org_by_user_query(&set_clauses_org).as_str());
        binds.push(&userorgbind);
        self.database.batch(&batch, &binds).await?;
        Ok(())
    }

    async fn delete_user(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
        user: CleannedUserModel,
    ) -> RepositoryResult<()> {
        let mut batch = Batch::default();
        batch.set_consistency(Consistency::Quorum);
        let query1 = r#"
        DELETE FROM axcelium.users 
        WHERE user_id = ? AND organization_id = ? AND application_id = ?
    "#;
        batch.append_statement(query1);

        if user.email.is_some() {
            let query2 = r#"
            DELETE FROM axcelium.users_by_email 
            WHERE email = ? AND organization_id = ? AND application_id = ?
        "#;
            batch.append_statement(query2);
        }

        let query3 = r#"
        DELETE FROM axcelium.users_by_username 
        WHERE username = ? AND organization_id = ? AND application_id = ?
    "#;
        batch.append_statement(query3);

        let query4 = r#"
        DELETE FROM axcelium.user_organizations 
        WHERE organization_id = ? AND user_id = ?
    "#;
        batch.append_statement(query4);

        let query5 = r#"
        DELETE FROM axcelium.user_organizations_by_user 
        WHERE  organization_id = ? AND user_id = ? 
    "#;
        batch.append_statement(query5);
        if user.email.is_some() {
            self.database
                .batch(
                    &batch,
                    (
                        (user_id, organization_id, application_id),
                        (user.email, organization_id, application_id),
                        (user.username, organization_id, application_id),
                        (organization_id, user_id),
                        (organization_id, user_id),
                    ),
                )
                .await?;
        } else {
            self.database
                .batch(
                    &batch,
                    (
                        (user_id, organization_id, application_id),
                        (user.username, organization_id, application_id),
                        (organization_id, user_id),
                        (organization_id, user_id),
                    ),
                )
                .await?;
        }

        Ok(())
    }
}
