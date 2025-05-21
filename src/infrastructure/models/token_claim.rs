use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub token_id: String,
    pub secret: String,
    pub secret_key: String,
    pub version: String,
    pub iat: i64,
    pub exp: i64,
    pub nbf: i64,
}