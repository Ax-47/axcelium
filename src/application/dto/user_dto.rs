use serde::{Deserialize, Serialize};

use crate::domain::models::user_models::{CreateUser, CreatedUser};

#[derive(Serialize,Deserialize)]
pub struct CreateUserDTO {
    pub username: String,
    pub password: String,
    pub repassword: String,
}
impl From<CreateUserDTO> for CreateUser {
    fn from(dto: CreateUserDTO) -> Self {
        CreateUser {
            username: dto.username,
            password: dto.password,
            repassword: dto.repassword,
        }
    }
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: u64,
    pub username: String,
}
impl From<CreatedUser> for CreateUserResponse {
    fn from(created_user: CreatedUser) -> Self {
        CreateUserResponse {
            id: created_user.id,
            username: created_user.username,
        }
    }
}
