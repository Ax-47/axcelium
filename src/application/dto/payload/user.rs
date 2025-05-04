use crate::domain::models::user_models::CreateUser;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct CreateUserPayload {
    pub username: String,
    pub email: Option<String>,
    pub password: String,
}
impl From<CreateUserPayload> for CreateUser {
    fn from(dto: CreateUserPayload) -> Self {
        CreateUser {
            username: dto.username,
            email: dto.email,
            password: dto.password,
        }
    }
}
