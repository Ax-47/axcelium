use crate::{
    domain::errors::repositories_errors::RepositoryResult,
    infrastructure::models::user::{
        CleannedUserModel, FoundUserModel, PaginatedUsersModel, UpdateUserModel, UserModel,
    },
};
use async_trait::async_trait;
use scylla::{
    client::session::Session,
    response::PagingState,
    statement::{batch::Batch, prepared::PreparedStatement, Consistency, SerialConsistency},
    value::CqlValue,
};
use std::{collections::HashMap, ops::ControlFlow, sync::Arc};
use uuid::Uuid;

use super::query::users::{
    update_user_org_by_user_query, update_user_org_query, update_users_by_email,
    update_users_by_username, update_users_query, DELETE_USERS, DELETE_USERS_BY_EMAIL,
    DELETE_USERS_BY_USERNAME, DELETE_USER_ORG, DELETE_USER_ORG_BY_USER, INSERT_USER,
    INSERT_USERS_BY_EMAIL_SEC, INSERT_USERS_BY_USERNAME_SEC, QUERY_FIND_ALL_USERS_PAGINATED,
    QUERY_FIND_RAW_USER, QUERY_FIND_USER, QUERY_FIND_USER_BY_EMAIL, QUERY_FIND_USER_BY_USERNAME,
};
pub struct UserDatabaseRepositoryImpl {
    pub database: Arc<Session>,
    insert_user: PreparedStatement,
    find_username: PreparedStatement,
    find_email: PreparedStatement,
    find_clean_user: PreparedStatement,
    find_all_users: PreparedStatement,
}

impl UserDatabaseRepositoryImpl {
    pub async fn new(database: Arc<Session>) -> Self {
        let mut insert_user = database.prepare(INSERT_USER).await.unwrap();
        insert_user.set_consistency(Consistency::Quorum);
        insert_user.set_serial_consistency(Some(SerialConsistency::Serial));

        let mut find_username = database.prepare(QUERY_FIND_USER_BY_USERNAME).await.unwrap();
        find_username.set_consistency(Consistency::Quorum);

        let mut find_email = database.prepare(QUERY_FIND_USER_BY_EMAIL).await.unwrap();
        find_email.set_consistency(Consistency::Quorum);

        let mut find_clean_user = database.prepare(QUERY_FIND_USER).await.unwrap();
        find_clean_user.set_consistency(Consistency::One);
        let find_all_users = database
            .prepare(QUERY_FIND_ALL_USERS_PAGINATED)
            .await
            .unwrap();
        Self {
            database,
            insert_user,
            find_username,
            find_email,
            find_clean_user,
            find_all_users,
        }
    }
}
#[async_trait]
pub trait UserDatabaseRepository: Send + Sync {
    async fn create_user(&self, user: UserModel) -> RepositoryResult<()>;
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
        old_user: UserModel,
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
    async fn create_user(&self, user: UserModel) -> RepositoryResult<()> {
        self.database
            .execute_unpaged(&self.insert_user, user)
            .await?;
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
            .execute_unpaged(&self.find_email, (email, application_id, organization_id))
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
            .execute_unpaged(
                &self.find_username,
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
        let mut prepared = self.find_all_users.clone();
        prepared.set_page_size(page_size);
        let paging_state = paging_state_u8
            .map(PagingState::new_from_raw_bytes)
            .unwrap_or_else(PagingState::start);
        let (res, paging_state_response) = self
            .database
            .execute_single_page(&prepared, &(organization_id, application_id), paging_state)
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
            .execute_unpaged(
                &self.find_clean_user,
                (organization_id, application_id, user_id),
            )
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
        old_user: UserModel,
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<()> {
        let mut batch = Batch::default();
        batch.set_consistency(Consistency::Quorum);
        let mut user_bind: HashMap<&str, CqlValue> = HashMap::new();
        let mut user_email_bind: HashMap<&str, CqlValue> = HashMap::new();
        let mut user_username_bind: HashMap<&str, CqlValue> = HashMap::new();
        let mut org_bind: HashMap<&str, CqlValue> = HashMap::new();
        let mut del_username_bind: HashMap<&str, CqlValue> = HashMap::new();
        let mut del_email_bind: HashMap<&str, CqlValue> = HashMap::new();
        let mut set_clauses_main: Vec<&'static str> = vec![];
        let mut set_clauses_org: Vec<&'static str> = vec![];
        let mut binds: Vec<&HashMap<&str, CqlValue>> = vec![];
        let mut update_user = old_user.clone();
        let has_username = user.username.is_some();
        let has_email = user.email.is_some();
        if let Some(ref username) = user.username {
            set_clauses_main.push("username = :username");
            set_clauses_org.push("username = :username");
            update_user.username = username.clone();
            let cql = CqlValue::Text(username.clone());
            user_bind.insert("username", cql.clone());
            user_email_bind.insert("username", cql.clone());
            user_username_bind.insert("username", cql.clone());
            org_bind.insert("username", cql);
        }
        if let Some(email) = user.email {
            set_clauses_main.push("email = :email");
            set_clauses_org.push("user_email = :user_email");
            update_user.email = Some(email.clone());
            user_bind.insert("email", CqlValue::Text(email.clone()));
            user_email_bind.insert("email", CqlValue::Text(email.clone()));
            user_username_bind.insert("email", CqlValue::Text(email.clone()));
            org_bind.insert("user_email", CqlValue::Text(email));
        }
        if let Some(pwd) = user.hashed_password {
            set_clauses_main.push("hashed_password = :hashed_password");
            update_user.hashed_password = pwd.clone();
            user_bind.insert("hashed_password", CqlValue::Text(pwd.clone()));
            user_email_bind.insert("hashed_password", CqlValue::Text(pwd.clone()));
            user_username_bind.insert("hashed_password", CqlValue::Text(pwd));
        }
        user_bind.insert("updated_at", CqlValue::Timestamp(user.updated_at));
        user_bind.insert("organization_id", CqlValue::Uuid(organization_id));
        user_bind.insert("application_id", CqlValue::Uuid(application_id));
        user_bind.insert("user_id", CqlValue::Uuid(user_id));
        if let Some(email) = old_user.email.clone() {
            user_email_bind.insert("updated_at", CqlValue::Timestamp(user.updated_at));
            user_email_bind.insert("organization_id", CqlValue::Uuid(organization_id));
            user_email_bind.insert("application_id", CqlValue::Uuid(application_id));
            user_email_bind.insert("email", CqlValue::Text(email));
            user_email_bind.insert("user_id", CqlValue::Uuid(user_id));
            user_username_bind.insert("updated_at", CqlValue::Timestamp(user.updated_at));
            user_username_bind.insert("organization_id", CqlValue::Uuid(organization_id));
            user_username_bind.insert("application_id", CqlValue::Uuid(application_id));
            user_username_bind.insert("username", CqlValue::Text(old_user.username.clone()));
            user_username_bind.insert("user_id", CqlValue::Uuid(user_id));
        }
        org_bind.insert("organization_id", CqlValue::Uuid(organization_id));
        org_bind.insert("user_id", CqlValue::Uuid(user_id));

        batch.append_statement(update_users_query(&set_clauses_main).as_str());
        binds.push(&user_bind);
        let insert_user_bind = update_user.to_bind_map();

        if has_email {
            if let Some(email) = old_user.email.clone() {
                del_email_bind.insert("organization_id", CqlValue::Uuid(organization_id));
                del_email_bind.insert("application_id", CqlValue::Uuid(application_id));
                del_email_bind.insert("email", CqlValue::Text(email));
                batch.append_statement(DELETE_USERS_BY_EMAIL);
                binds.push(&del_email_bind);
            }
            batch.append_statement(INSERT_USERS_BY_EMAIL_SEC);
            binds.push(&insert_user_bind);
            batch.append_statement(update_users_by_username(&set_clauses_main).as_str());
            binds.push(&user_username_bind);
        }
        if has_username {
            del_username_bind.insert("organization_id", CqlValue::Uuid(organization_id));
            del_username_bind.insert("application_id", CqlValue::Uuid(application_id));
            del_username_bind.insert("username", CqlValue::Text(old_user.username.clone()));
            batch.append_statement(DELETE_USERS_BY_USERNAME);
            binds.push(&del_username_bind);
            batch.append_statement(INSERT_USERS_BY_USERNAME_SEC);
            binds.push(&insert_user_bind);
            if old_user.email.is_some() {
                batch.append_statement(update_users_by_email(&set_clauses_main).as_str());
                binds.push(&user_email_bind);
            }
        }

        batch.append_statement(update_user_org_query(&set_clauses_org).as_str());
        binds.push(&org_bind);

        batch.append_statement(update_user_org_by_user_query(&set_clauses_org).as_str());
        binds.push(&org_bind);
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
        let mut del_user_bind: HashMap<&str, CqlValue> = HashMap::new();
        let mut del_email_bind: HashMap<&str, CqlValue> = HashMap::new();
        let mut del_username_bind: HashMap<&str, CqlValue> = HashMap::new();
        let mut del_user_org_bind: HashMap<&str, CqlValue> = HashMap::new();
        let mut binds: Vec<&HashMap<&str, CqlValue>> = vec![];
        batch.append_statement(DELETE_USERS);
        del_user_bind.insert("user_id", CqlValue::Uuid(user_id));
        del_user_bind.insert("organization_id", CqlValue::Uuid(organization_id));
        del_user_bind.insert("application_id", CqlValue::Uuid(application_id));
        binds.push(&del_user_bind);
        if let Some(email) = user.email {
            batch.append_statement(DELETE_USERS_BY_EMAIL);
            del_email_bind.insert("email", CqlValue::Text(email));
            del_email_bind.insert("organization_id", CqlValue::Uuid(organization_id));
            del_email_bind.insert("application_id", CqlValue::Uuid(application_id));

            binds.push(&del_email_bind);
        }
        batch.append_statement(DELETE_USERS_BY_USERNAME);
        del_username_bind.insert("username", CqlValue::Text(user.username));
        del_username_bind.insert("organization_id", CqlValue::Uuid(organization_id));
        del_username_bind.insert("application_id", CqlValue::Uuid(application_id));

        binds.push(&del_username_bind);
        batch.append_statement(DELETE_USER_ORG);

        del_user_org_bind.insert("organization_id", CqlValue::Uuid(organization_id));
        del_user_org_bind.insert("user_id", CqlValue::Uuid(user_id));

        binds.push(&del_user_org_bind);

        binds.push(&del_user_org_bind);
        batch.append_statement(DELETE_USER_ORG_BY_USER);
        self.database.batch(&batch, &binds).await?;
        Ok(())
    }
}
