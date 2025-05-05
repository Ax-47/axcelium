use serde::Serialize;

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub user_id: String,
    pub username: String,
    pub email: Option<String>,
}
