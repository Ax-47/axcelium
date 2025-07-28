use crate::{
    domain::entities::user::User, infrastructure::errors::fulltext_search::FulltextSearchError,
};
use async_trait::async_trait;
use elasticsearch::Elasticsearch;
use std::sync::Arc;

pub struct UserFulltextSearchRepositoryImpl {
    fulltext_search_client: Arc<Elasticsearch>,
    index: String,
}
impl UserFulltextSearchRepositoryImpl {
    pub fn new(fulltext_search_client: Arc<Elasticsearch>) -> Self {
        Self {
            fulltext_search_client,
            index: "users".to_owned(),
        }
    }
}
#[async_trait]
pub trait UserFulltextSearchRepository: Send + Sync {
    async fn create(&self, user: User) -> Result<(), FulltextSearchError>;
}

#[async_trait]
impl UserFulltextSearchRepository for UserFulltextSearchRepositoryImpl {
    async fn create(&self, user: User) -> Result<(), FulltextSearchError> {
        let res = self
            .fulltext_search_client
            .index(elasticsearch::IndexParts::IndexId(
                self.index.as_str(),
                &user.user_id.to_string(),
            ))
            .body(user)
            .send()
            .await?;
        if !res.status_code().is_success() {
            return Err(FulltextSearchError::IndexingFailed(
                res.status_code().as_str().to_string(),
            ));
        };
        Ok(())
    }
}
