use scylla::{DeserializeRow, SerializeRow};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Clone)]
pub struct CreateUser {
    pub username: String,
    pub email: Option<String>,
    pub password: String,
}

#[derive(Debug, Clone, DeserializeRow, SerializeRow)]
pub struct CreatedUser {
    pub user_id: Uuid,
    pub username: String,
    pub email: Option<String>,
}