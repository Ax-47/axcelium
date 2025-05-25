use async_trait::async_trait;
use scylla::{
    client::session::Session,
    statement::{batch::Batch, prepared::PreparedStatement},
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    domain::errors::repositories_errors::RepositoryResult,
    infrastructure::models::role::{
        RoleAssignmentModel, RoleModel, RoleUserModel, SelectedRoleByIdModel, UpdateRoleModel,
        UserRoleModel,
    },
};

use super::query::{
    role_users_by_role::{ASSIGN_USER_TO_ROLE, LIST_USERS_IN_ROLE},
    roles_by_app::{
        DELETE_ROLE_BY_ID, INSERT_ROLE_BY_APP, SELECT_ROLE_BY_ID, SELECT_ROLES_BY_ID,
        UPDATE_ROLE_BY_APP,
    },
    user_roles_by_user::{ASSIGN_ROLE_TO_USER, LIST_ROLES_OF_USER},
};

pub struct RoleDatabaseRepositoryImpl {
    pub database: Arc<Session>,
    insert_role_stmt: PreparedStatement,
    assign_user_to_role_stmt: Batch,
    get_roles_by_user_stmt: PreparedStatement,
    get_users_by_role_stmt: PreparedStatement,
    get_role_stmt: PreparedStatement,
    get_roles_stmt: PreparedStatement,
    delete_role_stmt: PreparedStatement,
    update_role_stmt: PreparedStatement,
}

impl RoleDatabaseRepositoryImpl {
    pub async fn new(database: Arc<Session>) -> Self {
        let insert_role_stmt = database.prepare(INSERT_ROLE_BY_APP).await.unwrap();
        let get_role_stmt = database.prepare(SELECT_ROLE_BY_ID).await.unwrap();
        let get_roles_stmt = database.prepare(SELECT_ROLES_BY_ID).await.unwrap();
        let get_roles_by_user_stmt = database.prepare(LIST_ROLES_OF_USER).await.unwrap();
        let get_users_by_role_stmt = database.prepare(LIST_USERS_IN_ROLE).await.unwrap();
        let mut assign_batch: Batch = Default::default();
        assign_batch.append_statement(ASSIGN_ROLE_TO_USER);
        assign_batch.append_statement(ASSIGN_USER_TO_ROLE);
        let assign_user_to_role_stmt: Batch = database.prepare_batch(&assign_batch).await.unwrap();
        let update_role_stmt = database.prepare(UPDATE_ROLE_BY_APP).await.unwrap();
        let delete_role_stmt = database.prepare(DELETE_ROLE_BY_ID).await.unwrap();

        Self {
            database,
            insert_role_stmt,
            get_roles_stmt,
            assign_user_to_role_stmt,
            get_roles_by_user_stmt,
            get_users_by_role_stmt,
            get_role_stmt,
            update_role_stmt,
            delete_role_stmt,
        }
    }
}
#[async_trait]
pub trait RoleDatabaseRepository: Send + Sync {
    async fn create_role(&self, role: &RoleModel) -> RepositoryResult<()>;
    async fn update_role(&self, update: &UpdateRoleModel) -> RepositoryResult<()>;
    async fn delete_role(&self, org_id: Uuid, app_id: Uuid, role: Uuid) -> RepositoryResult<()>;
    async fn assign_user_to_role(&self, assignment: &RoleAssignmentModel) -> RepositoryResult<()>;
    async fn get_role(
        &self,
        org_id: Uuid,
        app_id: Uuid,
        role_id: Uuid,
    ) -> RepositoryResult<Option<SelectedRoleByIdModel>>;

    async fn get_roles(
        &self,
        org_id: Uuid,
        app_id: Uuid,
    ) -> RepositoryResult<Vec<SelectedRoleByIdModel>>;
    async fn get_roles_by_user(
        &self,
        org_id: Uuid,
        app_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<Vec<UserRoleModel>>;
    async fn get_users_by_role(
        &self,
        org_id: Uuid,
        app_id: Uuid,
        role_id: Uuid,
    ) -> RepositoryResult<Vec<RoleUserModel>>;
}
#[async_trait]
impl RoleDatabaseRepository for RoleDatabaseRepositoryImpl {
    async fn create_role(&self, role: &RoleModel) -> RepositoryResult<()> {
        self.database
            .execute_unpaged(&self.insert_role_stmt, role)
            .await?;
        Ok(())
    }

    async fn update_role(&self, update: &UpdateRoleModel) -> RepositoryResult<()> {
        self.database
            .execute_unpaged(&self.update_role_stmt, update)
            .await?;
        Ok(())
    }

    async fn delete_role(&self, org_id: Uuid, app_id: Uuid, role: Uuid) -> RepositoryResult<()> {
        self.database
            .execute_unpaged(&self.delete_role_stmt, (org_id,app_id,role))
            .await?;
        Ok(())
    }
    async fn assign_user_to_role(&self, assignment: &RoleAssignmentModel) -> RepositoryResult<()> {
        self.database
            .batch(&self.assign_user_to_role_stmt, (assignment, assignment))
            .await?;
        Ok(())
    }

    async fn get_role(
        &self,
        org_id: Uuid,
        app_id: Uuid,
        role_id: Uuid,
    ) -> RepositoryResult<Option<SelectedRoleByIdModel>> {
        let res = self
            .database
            .execute_unpaged(&self.get_role_stmt, (org_id, app_id, role_id))
            .await?;
        Ok(res.into_rows_result()?.maybe_first_row()?)
    }

    async fn get_roles(
        &self,
        org_id: Uuid,
        app_id: Uuid,
    ) -> RepositoryResult<Vec<SelectedRoleByIdModel>> {
        let res = self
            .database
            .execute_unpaged(&self.get_roles_stmt, (org_id, app_id))
            .await?;
        Ok(res
            .into_rows_result()?
            .rows::<SelectedRoleByIdModel>()?
            .collect::<Result<_, _>>()?)
    }
    async fn get_roles_by_user(
        &self,
        org_id: Uuid,
        app_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<Vec<UserRoleModel>> {
        let res = self
            .database
            .execute_unpaged(&self.get_roles_by_user_stmt, (org_id, app_id, user_id))
            .await?;
        Ok(res
            .into_rows_result()?
            .rows::<UserRoleModel>()?
            .collect::<Result<_, _>>()?)
    }

    async fn get_users_by_role(
        &self,
        org_id: Uuid,
        app_id: Uuid,
        role_id: Uuid,
    ) -> RepositoryResult<Vec<RoleUserModel>> {
        let res = self
            .database
            .execute_unpaged(&self.get_users_by_role_stmt, (org_id, app_id, role_id))
            .await?;
        Ok(res
            .into_rows_result()?
            .rows::<RoleUserModel>()?
            .collect::<Result<_, _>>()?)
    }
}
