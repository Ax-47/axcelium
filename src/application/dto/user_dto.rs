use serde::Serialize;

use crate::domain::models::user_models::{CreateUser, CreatedUser};

#[derive(Serialize)]
pub struct CreateUserDTO {
    pub username: String,
    pub password: String,
    pub role: String,
}
impl Into<CreateUser> for CreateUserDTO {
    fn into(self) -> CreateUser {
        CreateUser {
            username: self.username,
            password: self.password,
            role: self.role,
        }
    }
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: u64,
    pub username: String,
    pub role: String,
}
impl Into<CreateUserResponse> for CreatedUser {
    fn into(self) -> CreateUserResponse {
        CreateUserResponse {
            id: self.id,
            username: self.username,
            role: self.role,
        }
    }
}
