use crate::domain::models::user_models::CreatedUser;
use serde::Serialize;

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
}
impl From<CreatedUser> for CreateUserResponse {
    fn from(created_user: CreatedUser) -> Self {
        CreateUserResponse {
            id: created_user.user_id.to_string(),
            username: created_user.username,
            email: created_user.email,
        }
    }
}
