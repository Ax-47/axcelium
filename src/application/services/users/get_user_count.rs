use crate::{
    application::{
        dto::response::user::GetUserCountResponse,
        repositories::users::get_user_count::GetUserCountRepository,
    },
    domain::errors::repositories_errors::RepositoryResult,
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetUserCountServiceImpl {
    pub repository: Arc<dyn GetUserCountRepository>,
}
impl GetUserCountServiceImpl {
    pub fn new(repository: Arc<dyn GetUserCountRepository>) -> Self {
        Self { repository }
    }
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GetUserCountService: 'static + Sync + Send {
    async fn execute(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
    ) -> RepositoryResult<GetUserCountResponse>;
}
#[async_trait]
impl GetUserCountService for GetUserCountServiceImpl {
    async fn execute(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
    ) -> RepositoryResult<GetUserCountResponse> {
        let user_count = self
            .repository
            .get_user_count(organization_id, application_id)
            .await?;
        Ok(GetUserCountResponse { user_count })
    }
}
