use serde::{Deserialize, Serialize};
use crate::domain::models::user_models::{CreateUser, CreatedUser};

#[derive(Serialize,Deserialize)]
pub struct CreateUserDTO {
    pub username: String,
    pub email: Option<String>,
    pub password: String,
}
impl From<CreateUserDTO> for CreateUser {
    fn from(dto: CreateUserDTO) -> Self {
        CreateUser {
            username: dto.username,
            email: dto.email,
            password: dto.password,
        }
    }
}

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
