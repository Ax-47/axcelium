use serde::Serialize;


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