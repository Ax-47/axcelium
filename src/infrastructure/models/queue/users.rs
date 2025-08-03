use serde::{Deserialize, Serialize};

use crate::{
    domain::entities::user::User, infrastructure::models::queue::queue_payload::QueueOperation,
};
#[derive(Serialize, Deserialize)]
pub struct QueueUser {
    pub operation: String,
    pub user: Option<User>,
}

impl QueueOperation for QueueUser {
    fn operation(&self) -> &str {
        &self.operation
    }
}
