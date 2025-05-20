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
