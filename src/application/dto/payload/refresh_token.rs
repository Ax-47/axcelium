use serde::Deserialize;
use uuid::Uuid;


#[derive(Deserialize)]
pub struct CreateTokenPayload {
    pub user_id: Uuid,
    pub paseto_key: String,
}