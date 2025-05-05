use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq,Serialize)]
pub struct CreateUserResponse {
    pub user_id: String,
    pub username: String,
    pub email: Option<String>,
}
