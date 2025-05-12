use crate::{
    application::dto::payload::user::UpdateUserPayload,
    domain::errors::repositories_errors::RepositoryResult,
    infrastructure::{
        models::{user::UpdateUserModel, user_organization::UpdateUserOrganizationModel},
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
    ) -> RepositoryResult<()>;
}

#[async_trait]
impl UpdateUserRepository for UpdateUserRepositoryImpl {
    async fn update_user(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
        update: UpdateUserPayload,
    ) -> RepositoryResult<()> {
        let update_user = UpdateUserModel::new(
            update.username.clone(),
            update.email.clone(),
            update.password,
        );
        let update_user_org = UpdateUserOrganizationModel::new(update.username, update.email);
        self.database_repo
            .update_user(
                update_user,
                update_user_org,
                application_id,
                organization_id,
                user_id,
            )
            .await
    }
}
