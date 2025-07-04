use crate::{
    domain::errors::repositories_errors::RepositoryResult,
    infrastructure::models::user::{
        CleannedUserModel, FoundUserModel, PaginatedUsersModel, UserModel,
    },
};
use async_trait::async_trait;
use scylla::{
    client::session::Session,
    response::PagingState,
    statement::{Consistency, SerialConsistency, prepared::PreparedStatement},
    value::{self, CqlValue},
};
use std::{collections::HashMap, ops::ControlFlow, sync::Arc};
use uuid::Uuid;

use super::query::users::{
    BAN_USER, DECREASE_USER, DELETE_USER, DISABLE_MFA_USER, INCREASE_USER, INSERT_USER,
    QUERY_FIND_ALL_USERS_PAGINATED, QUERY_FIND_RAW_USER, QUERY_FIND_USER, QUERY_FIND_USER_BY_EMAIL,
    QUERY_FIND_USER_BY_USERNAME, SELECT_USER_COUNT, UNBAN_USER, UPDATE_USER,
};
pub struct UserDatabaseRepositoryImpl {
    database: Arc<Session>,
    insert_user: PreparedStatement,
    find_username: PreparedStatement,
    find_email: PreparedStatement,
    find_clean_user: PreparedStatement,
    find_all_users: PreparedStatement,
    find_raw_user: PreparedStatement,
    update_user: PreparedStatement,
    delete_user: PreparedStatement,
    increase_user: PreparedStatement,
    decrease_user: PreparedStatement,
    select_user_count: PreparedStatement,
    ban_user: PreparedStatement,
    unban_user: PreparedStatement,
    disable_mfa_user: PreparedStatement,
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
        let mut update_user = database.prepare(UPDATE_USER).await.unwrap();
        update_user.set_consistency(Consistency::Quorum);

        let mut find_raw_user = database.prepare(QUERY_FIND_RAW_USER).await.unwrap();
        find_raw_user.set_consistency(Consistency::Quorum);

        let mut delete_user = database.prepare(DELETE_USER).await.unwrap();
        delete_user.set_consistency(Consistency::One);
        let mut increase_user = database.prepare(INCREASE_USER).await.unwrap();
        increase_user.set_consistency(Consistency::Quorum);

        let mut decrease_user = database.prepare(DECREASE_USER).await.unwrap();
        decrease_user.set_consistency(Consistency::Quorum);

        let mut select_user_count = database.prepare(SELECT_USER_COUNT).await.unwrap();
        select_user_count.set_consistency(Consistency::Quorum);

        let mut ban_user = database.prepare(BAN_USER).await.unwrap();
        ban_user.set_consistency(Consistency::Quorum);

        let mut unban_user = database.prepare(UNBAN_USER).await.unwrap();
        unban_user.set_consistency(Consistency::Quorum);

        let mut disable_mfa_user = database.prepare(DISABLE_MFA_USER).await.unwrap();
        disable_mfa_user.set_consistency(Consistency::Quorum);
        Self {
            database,
            insert_user,
            find_username,
            find_email,
            find_clean_user,
            find_all_users,
            update_user,
            find_raw_user,
            delete_user,
            increase_user,
            decrease_user,
            select_user_count,
            ban_user,
            unban_user,
            disable_mfa_user,
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
    async fn update_user(&self, user: UserModel) -> RepositoryResult<()>;

    async fn delete_user(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<()>;

    async fn get_user_count(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
    ) -> RepositoryResult<i64>;

    async fn ban_user(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
        application_id: Uuid,
    ) -> RepositoryResult<()>;

    async fn unban_user(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
        application_id: Uuid,
    ) -> RepositoryResult<()>;

    async fn disable_mfa_user(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
        application_id: Uuid,
    ) -> RepositoryResult<()>;
}
#[async_trait]
impl UserDatabaseRepository for UserDatabaseRepositoryImpl {
    async fn create_user(&self, user: UserModel) -> RepositoryResult<()> {
        self.database
            .execute_unpaged(&self.insert_user, &user)
            .await?;
        self.database
            .execute_unpaged(
                &self.increase_user,
                (user.organization_id, user.application_id),
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
    async fn update_user(&self, user: UserModel) -> RepositoryResult<()> {
        self.database
            .execute_unpaged(&self.update_user, &user)
            .await?;
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
        del_user_bind.insert("organization_id", CqlValue::Uuid(organization_id));
        del_user_bind.insert("application_id", CqlValue::Uuid(application_id));
        self.database
            .execute_unpaged(&self.delete_user, &del_user_bind)
            .await?;
        self.database
            .execute_unpaged(&self.decrease_user, (organization_id, application_id))
            .await?;

        Ok(())
    }
    async fn get_user_count(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
    ) -> RepositoryResult<i64> {
        let result = self
            .database
            .execute_unpaged(&self.select_user_count, (organization_id, application_id))
            .await?
            .into_rows_result()?
            .maybe_first_row::<(value::Counter,)>()?
            .unwrap_or((value::Counter(0),));
        Ok(result.0.0)
    }

    async fn ban_user(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
        application_id: Uuid,
    ) -> RepositoryResult<()> {
        self.database
            .execute_unpaged(&self.ban_user, (organization_id, application_id, user_id))
            .await?;

        Ok(())
    }

    async fn unban_user(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
        application_id: Uuid,
    ) -> RepositoryResult<()> {
        self.database
            .execute_unpaged(&self.unban_user, (organization_id, application_id, user_id))
            .await?;
        Ok(())
    }
    async fn disable_mfa_user(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
        application_id: Uuid,
    ) -> RepositoryResult<()> {
        self.database
            .execute_unpaged(
                &self.disable_mfa_user,
                (organization_id, application_id, user_id),
            )
            .await?;
        Ok(())
    }
}
