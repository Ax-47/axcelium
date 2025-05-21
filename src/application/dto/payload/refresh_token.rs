use serde::Deserialize;
use uuid::Uuid;


#[derive(Deserialize)]
pub struct CreateTokenPayload {
    pub user_id: Uuid,
    pub paseto_key: String,
}

#[derive(Deserialize)]
pub struct RotateTokenPayload {
    pub refresh_token: String,
    pub public_key: String,
}