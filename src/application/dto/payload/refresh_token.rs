use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct GetUserIdQuery {
    pub user_id: Uuid,
}

#[derive(Deserialize)]
pub struct CreateTokenPayload {
    pub paseto_key: String,
}