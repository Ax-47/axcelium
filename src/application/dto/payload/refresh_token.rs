use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateTokenPayload {
    pub user_id: Uuid,
    pub private_key: String,
}
#[derive(Deserialize)]
pub struct RotateTokenPayload {
    pub refresh_token: String,
    pub public_key: String,
    pub private_key: String,
}

#[derive(Deserialize)]
pub struct GetTokenQuery {
    pub token_id: Uuid,
}

#[derive(Deserialize)]
pub struct GetUserQuery {
    pub user_id: Uuid,
}

#[derive(Deserialize)]
pub struct PaginationRefreshTokensByUserQuery {
    pub page_size: Option<i32>,
    pub paging_state: Option<String>,
}