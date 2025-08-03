use serde::{Deserialize, Serialize};

use crate::{
    domain::entities::user::User, infrastructure::models::queue::queue_payload::QueueOperation,
};
#[derive(Serialize, Deserialize)]
pub struct QueueUser {
    pub operation: String,
    pub user: Option<User>,
}
impl QueueUser {
    pub fn new(op: &str, user: User) -> Self {
        Self {
            operation: op.to_string(),
            user: Some(user),
        }
    }

    pub fn delete(op: &str) -> Self {
        Self {
            operation: op.to_string(),
            user: None,
        }
    }
}
impl QueueOperation for QueueUser {
    fn operation(&self) -> &str {
        &self.operation
    }
}
