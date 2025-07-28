use crate::domain::entities::user::User;
use async_trait::async_trait;
use elasticsearch::Elasticsearch;
use std::sync::Arc;

pub struct UserFulltextSearchRepositoryImpl {
    fulltext_search_client: Arc<Elasticsearch>,
    index: String,
}
impl UserFulltextSearchRepositoryImpl {
    pub fn new(&self, fulltext_search_client: Arc<Elasticsearch>) -> Self {
        Self {
            fulltext_search_client,
            index: "users".to_owned(),
        }
    }
}
#[async_trait]
pub trait UserFulltextSearchRepository: Send + Sync {
    async fn create(&self, user: User);
}

#[async_trait]
impl UserFulltextSearchRepository for UserFulltextSearchRepositoryImpl {
    async fn create(&self, user: User) {
        let _ = self
            .fulltext_search_client
            .index(elasticsearch::IndexParts::IndexId(
                self.index.as_str(),
                &user.user_id.to_string(),
            ))
            .body(user)
            .send()
            .await;
    }
}
