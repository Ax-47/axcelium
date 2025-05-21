use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub struct CreateTokenResponse {
    pub refresh_token: String,
}