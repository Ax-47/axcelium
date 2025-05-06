use crate::infrastructure::models::user::CleannedUserModel;
use serde::Serialize;
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateUserResponse {
    pub user_id: String,
    pub username: String,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetUsersResponse {
    pub users: Vec<CleannedUserModel>,
    pub paging_state: Option<String>,
}
