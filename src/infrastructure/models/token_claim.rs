use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub jti: String,
    pub secret: String,
    pub secret_key: String,
    pub version: String,
    pub iat: String,
    pub exp: String,
    pub nbf: String,
}
