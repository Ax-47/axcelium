use serde::{Deserialize, Serialize};
use super::timestamp::from_iso8601_to_timestamp;
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub jti: String,
    pub secret: String,
    pub secret_key: String,
    pub version: String,
    #[serde(deserialize_with = "from_iso8601_to_timestamp")]
    pub iat: i64,

    #[serde(deserialize_with = "from_iso8601_to_timestamp")]
    pub exp: i64,

    #[serde(deserialize_with = "from_iso8601_to_timestamp")]
    pub nbf: i64,
}
