use serde::Serialize;

use crate::domain::models::user_models::{CreateUser, CreatedUser};

#[derive(Serialize)]
pub struct CreateUserDTO {
    pub username: String,
    pub password: String,
    pub role: String,
}
impl From<CreateUserDTO> for CreateUser {
    fn from(dto: CreateUserDTO) -> Self {
        CreateUser {
            username: dto.username,
            password: dto.password,
            role: dto.role,
        }
    }
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: u64,
    pub username: String,
    pub role: String,
}
impl From<CreatedUser> for CreateUserResponse {
    fn from(created_user: CreatedUser) -> Self {
        CreateUserResponse {
            id: created_user.id,
            username: created_user.username,
            role: created_user.role,
        }
    }
}
