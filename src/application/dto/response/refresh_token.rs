use serde::Serialize;

use crate::infrastructure::models::refresh_token::FoundRefreshTokenModelByUser;


#[derive(Debug, Clone, Serialize)]
pub struct CreateTokenResponse {
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RotateTokenResponse {
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SimpleResponse {
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetRefreshTokensResponse {
    pub refresh_tokens: Vec<FoundRefreshTokenModelByUser>,
    pub paging_state: Option<String>,
}