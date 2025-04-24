use scylla::{DeserializeRow, SerializeRow};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub password: String,
    pub repassword: String,
}
#[derive(Serialize, Clone)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, DeserializeRow, SerializeRow)]
pub struct CreatedUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}
