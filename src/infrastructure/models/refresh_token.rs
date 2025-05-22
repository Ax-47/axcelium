use crate::domain::entities::refresh_token::RefreshToken;
use crate::infrastructure::repositories::database::scylla_serialize::{
    deserialize_cql_timestamp, serialize_cql_timestamp,
};
use scylla::value::CqlTimestamp;
use scylla::{DeserializeRow, SerializeRow};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Clone, SerializeRow, DeserializeRow, Serialize, Deserialize)]
pub struct RefreshTokenModel {
    pub token_id: Uuid,
    pub application_id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub encrypted_token_secret: String,
    pub token_version: String,
    pub parent_version: Option<String>,

    #[serde(
        serialize_with = "serialize_cql_timestamp",
        deserialize_with = "deserialize_cql_timestamp"
    )]
    pub issued_at: CqlTimestamp,

    #[serde(
        serialize_with = "serialize_cql_timestamp",
        deserialize_with = "deserialize_cql_timestamp"
    )]
    pub expires_at: CqlTimestamp,
    pub revoked: bool,
}
impl From<RefreshToken> for RefreshTokenModel {
    fn from(token: RefreshToken) -> Self {
        Self {
            token_id: token.token_id,
            application_id: token.application_id,
            organization_id: token.organization_id,
            user_id: token.user_id,
            encrypted_token_secret: token.encrypted_token_secret,
            token_version: token.token_version,
            parent_version: token.parent_version,
            issued_at: token.issued_at,
            expires_at: token.expires_at,
            revoked: token.revoked,
        }
    }
}

impl From<RefreshTokenModel> for RefreshToken {
    fn from(model: RefreshTokenModel) -> Self {
        Self {
            token_id: model.token_id,
            application_id: model.application_id,
            organization_id: model.organization_id,
            user_id: model.user_id,
            encrypted_token_secret: model.encrypted_token_secret,
            token_version: model.token_version,
            parent_version: model.parent_version,
            issued_at: model.issued_at,
            expires_at: model.expires_at,
            revoked: model.revoked,
        }
    }
}
impl From<&RefreshToken> for RefreshTokenModel {
    fn from(token: &RefreshToken) -> Self {
        Self {
            token_id: token.token_id,
            application_id: token.application_id,
            organization_id: token.organization_id,
            user_id: token.user_id,
            encrypted_token_secret: token.encrypted_token_secret.clone(),
            token_version: token.token_version.clone(),
            parent_version: token.parent_version.clone(),
            issued_at: token.issued_at,
            expires_at: token.expires_at,
            revoked: token.revoked,
        }
    }
}
#[derive(Debug, Clone, SerializeRow, DeserializeRow, Serialize, Deserialize)]
pub struct FoundRefreshTokenModel {
    pub application_id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub encrypted_token_secret: String,
    pub parent_version: Option<String>,

    #[serde(
        serialize_with = "serialize_cql_timestamp",
        deserialize_with = "deserialize_cql_timestamp"
    )]
    pub issued_at: CqlTimestamp,

    #[serde(
        serialize_with = "serialize_cql_timestamp",
        deserialize_with = "deserialize_cql_timestamp"
    )]
    pub expires_at: CqlTimestamp,
    pub revoked: bool,
}
#[derive(Debug, Clone, SerializeRow, DeserializeRow, Serialize, Deserialize)]
pub struct UpdateRefreshTokenQuery {
    pub token_id: Uuid,
    pub application_id: Uuid,
    pub organization_id: Uuid,
    pub token_version: String,
    pub parent_version: Option<String>,
    #[serde(
        serialize_with = "serialize_cql_timestamp",
        deserialize_with = "deserialize_cql_timestamp"
    )]
    pub issued_at: CqlTimestamp,
    #[serde(
        serialize_with = "serialize_cql_timestamp",
        deserialize_with = "deserialize_cql_timestamp"
    )]
    pub expires_at: CqlTimestamp,
}

impl From<&RefreshTokenModel> for UpdateRefreshTokenQuery {
    fn from(token: &RefreshTokenModel) -> Self {
        Self {
            token_id: token.token_id,
            application_id: token.application_id,
            organization_id: token.organization_id,
            token_version: token.token_version.clone(),
            parent_version: token.parent_version.clone(),
            issued_at: token.issued_at,
            expires_at: token.expires_at,
        }
    }
}
