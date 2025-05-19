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
    statement::{Consistency, SerialConsistency, batch::Batch, prepared::PreparedStatement},
    value::CqlValue,
};
use std::{collections::HashMap, ops::ControlFlow, sync::Arc};
use uuid::Uuid;

use super::query::users::{
    DECREASE_USER, DELETE_USER, INSERT_USER, QUERY_FIND_ALL_USERS_PAGINATED, QUERY_FIND_RAW_USER,
    QUERY_FIND_USER, QUERY_FIND_USER_BY_EMAIL, QUERY_FIND_USER_BY_USERNAME, UPDATE_USER_EMAIL,
    UPDATE_USER_PASSWORD, UPDATE_USER_PASSWORD_EMAIL, UPDATE_USER_USERNAME,
    UPDATE_USER_USERNAME_EMAIL, UPDATE_USER_USERNAME_PASSWORD, UPDATE_USER_USERNAME_PASSWORD_EMAIL,
};
pub struct UserDatabaseRepositoryImpl {
    database: Arc<Session>,
    insert_user: PreparedStatement,
    find_username: PreparedStatement,
    find_email: PreparedStatement,
    find_clean_user: PreparedStatement,
    find_all_users: PreparedStatement,
    find_raw_user: PreparedStatement,
    update_user_username: PreparedStatement,
    update_user_password: PreparedStatement,
    update_user_email: PreparedStatement,
    update_user_username_password: PreparedStatement,
    update_user_username_email: PreparedStatement,
    update_user_password_email: PreparedStatement,
    update_user_username_password_email: PreparedStatement,
    delete_user: PreparedStatement,
    increase_user: PreparedStatement,
    decrease_user: PreparedStatement,
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
        let mut find_all_users = database
            .prepare(QUERY_FIND_ALL_USERS_PAGINATED)
            .await
            .unwrap();
        find_all_users.set_consistency(Consistency::One);
        let mut update_user_username = database.prepare(UPDATE_USER_USERNAME).await.unwrap();
        update_user_username.set_consistency(Consistency::Quorum);

        let mut update_user_password = database.prepare(UPDATE_USER_PASSWORD).await.unwrap();
        update_user_password.set_consistency(Consistency::Quorum);

        let mut update_user_email = database.prepare(UPDATE_USER_EMAIL).await.unwrap();
        update_user_email.set_consistency(Consistency::Quorum);

        let mut update_user_username_password = database
            .prepare(UPDATE_USER_USERNAME_PASSWORD)
            .await
            .unwrap();
        update_user_username_password.set_consistency(Consistency::Quorum);

        let mut update_user_username_email =
            database.prepare(UPDATE_USER_USERNAME_EMAIL).await.unwrap();
        update_user_username_email.set_consistency(Consistency::Quorum);

        let mut update_user_password_email =
            database.prepare(UPDATE_USER_PASSWORD_EMAIL).await.unwrap();
        update_user_password_email.set_consistency(Consistency::Quorum);

        let mut update_user_username_password_email = database
            .prepare(UPDATE_USER_USERNAME_PASSWORD_EMAIL)
            .await
            .unwrap();
        update_user_username_password_email.set_consistency(Consistency::Quorum);

        let mut find_raw_user = database.prepare(QUERY_FIND_RAW_USER).await.unwrap();
        find_raw_user.set_consistency(Consistency::Quorum);

        let mut delete_user = database.prepare(DELETE_USER).await.unwrap();
        delete_user.set_consistency(Consistency::One);
        let mut increase_user = database.prepare(INSERT_USER).await.unwrap();
        increase_user.set_consistency(Consistency::Quorum);

        let mut decrease_user = database.prepare(DECREASE_USER).await.unwrap();
        decrease_user.set_consistency(Consistency::Quorum);

        Self {
            database,
            insert_user,
            find_username,
            find_email,
            find_clean_user,
            find_all_users,
            update_user_username,
            update_user_password,
            update_user_email,
            update_user_username_password,
            update_user_username_email,
            update_user_password_email,
            update_user_username_password_email,
            find_raw_user,
            delete_user,
            increase_user,
            decrease_user,
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
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<()>;

    async fn delete_user(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<()>;
}
#[async_trait]
impl UserDatabaseRepository for UserDatabaseRepositoryImpl {
    async fn create_user(&self, user: UserModel) -> RepositoryResult<()> {
        let mut batch: Batch = Default::default();
        batch.append_statement(self.insert_user.clone());
        batch.append_statement(self.increase_user.clone());
        self.database
            .batch(
                &batch,
                (
                    (user.organization_id.clone(), user.application_id.clone()),
                    &user,
                ),
            )
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
            .execute_unpaged(
                &self.find_raw_user,
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
        if user.username.is_some() && user.email.is_none() && user.hashed_password.is_none() {
            self.database
                .execute_unpaged(
                    &self.update_user_username,
                    (
                        user.username,
                        user.updated_at,
                        organization_id,
                        application_id,
                        user_id,
                    ),
                )
                .await?;
        } else if user.username.is_none() && user.email.is_some() && user.hashed_password.is_none()
        {
            self.database
                .execute_unpaged(
                    &self.update_user_email,
                    (
                        user.email,
                        user.updated_at,
                        organization_id,
                        application_id,
                        user_id,
                    ),
                )
                .await?;
        } else if user.username.is_none() && user.email.is_none() && user.hashed_password.is_some()
        {
            self.database
                .execute_unpaged(
                    &self.update_user_password,
                    (
                        user.hashed_password,
                        user.updated_at,
                        organization_id,
                        application_id,
                        user_id,
                    ),
                )
                .await?;
        } else if user.username.is_some() && user.email.is_some() && user.hashed_password.is_none()
        {
            self.database
                .execute_unpaged(
                    &self.update_user_username_email,
                    (
                        user.username,
                        user.email,
                        user.updated_at,
                        organization_id,
                        application_id,
                        user_id,
                    ),
                )
                .await?;
        } else if user.username.is_some() && user.email.is_none() && user.hashed_password.is_some()
        {
            self.database
                .execute_unpaged(
                    &self.update_user_username_password,
                    (
                        user.username,
                        user.hashed_password,
                        user.updated_at,
                        organization_id,
                        application_id,
                        user_id,
                    ),
                )
                .await?;
        } else if user.username.is_none() && user.email.is_some() && user.hashed_password.is_some()
        {
            self.database
                .execute_unpaged(
                    &self.update_user_password_email,
                    (
                        user.hashed_password,
                        user.email,
                        user.updated_at,
                        organization_id,
                        application_id,
                        user_id,
                    ),
                )
                .await?;
        } else if user.username.is_some() && user.email.is_some() && user.hashed_password.is_some()
        {
            self.database
                .execute_unpaged(
                    &self.update_user_username_password_email,
                    (
                        user.username,
                        user.hashed_password,
                        user.email,
                        user.updated_at,
                        organization_id,
                        application_id,
                        user_id,
                    ),
                )
                .await?;
        }
        Ok(())
    }

    async fn delete_user(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<()> {
        let mut del_user_bind: HashMap<&str, CqlValue> = HashMap::new();
        del_user_bind.insert("user_id", CqlValue::Uuid(user_id));
        del_user_bind.insert("organization_id", CqlValue::Uuid(organization_id.clone()));
        del_user_bind.insert("application_id", CqlValue::Uuid(application_id.clone()));
        let mut batch: Batch = Default::default();
        batch.append_statement(self.decrease_user.clone());
        batch.append_statement(self.delete_user.clone());
        self.database
            .batch(&batch, ((organization_id, application_id), &del_user_bind))
            .await?;
        Ok(())
    }
}
