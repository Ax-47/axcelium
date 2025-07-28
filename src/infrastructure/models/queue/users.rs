use serde::{Deserialize, Serialize};

use crate::domain::entities::user::User;
#[derive(Serialize, Deserialize)]
pub struct QueueUser {
    pub operation: String,
    pub user: Option<User>,
}
