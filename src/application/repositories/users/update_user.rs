use crate::{
    application::dto::payload::user::UpdateUserPayload,
    domain::errors::repositories_errors::RepositoryResult,
    infrastructure::{
        models::user::{UpdateUserModel, UserModel},
        repositories::database::user_repository::UserDatabaseRepository,
    },
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
pub struct UpdateUserRepositoryImpl {
    database_repo: Arc<dyn UserDatabaseRepository>,
}

impl UpdateUserRepositoryImpl {
    pub fn new(database_repo: Arc<dyn UserDatabaseRepository>) -> Self {
        Self { database_repo }
    }
}
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UpdateUserRepository: Send + Sync {
    async fn update_user(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
        update: UpdateUserPayload,
        user: UserModel,
    ) -> RepositoryResult<()>;
    async fn find_user(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<Option<UserModel>>;
}

#[async_trait]
impl UpdateUserRepository for UpdateUserRepositoryImpl {
    async fn find_user(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<Option<UserModel>> {
        self.database_repo
            .find_raw_user(application_id, organization_id, user_id).await
    }
    async fn update_user(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
        update: UpdateUserPayload,
        user: UserModel,
    ) -> RepositoryResult<()> {
        let update_user = UpdateUserModel::new(update.username, update.email, update.password);
        self.database_repo
            .update_user(update_user, user, organization_id, application_id, user_id)
            .await
    }
}
