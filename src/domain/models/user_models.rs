use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub password: String,
    pub role: String,
}
#[derive(Serialize, Clone)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub role: String,
}

#[derive(Serialize)]
pub struct CreatedUser {
    pub id: u64,
    pub username: String,
    pub role: String,
}
