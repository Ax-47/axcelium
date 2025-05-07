use crate::infrastructure::models::user::CleannedUserModel;
use crate::infrastructure::repositories::database::scylla_serialize::{
    serialize_cql_timestamp, serialize_optional_cql_timestamp,
};
use scylla::value::CqlTimestamp;
use serde::Serialize;
use uuid::Uuid;
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateUserResponse {
    pub user_id: String,
    pub username: String,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetUsersResponse {
    pub users: Vec<CleannedUserModel>,
    pub paging_state: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetUserResponse {
    pub user_id: Uuid,
    pub organization_id: Uuid,
    pub application_id: Uuid,
    pub username: String,
    pub email: Option<String>,
    #[serde(
        serialize_with = "serialize_cql_timestamp",
        deserialize_with = "deserialize_cql_timestamp"
    )]
    pub created_at: CqlTimestamp,
    #[serde(
        serialize_with = "serialize_cql_timestamp",
        deserialize_with = "deserialize_cql_timestamp"
    )]
    pub updated_at: CqlTimestamp,
    pub is_active: bool,
    pub is_verified: bool,
    pub is_locked: bool,
    #[serde(
        serialize_with = "serialize_optional_cql_timestamp",
        deserialize_with = "deserialize_optional_cql_timestamp"
    )]
    pub last_login: Option<CqlTimestamp>,
    pub mfa_enabled: bool,
    #[serde(
        serialize_with = "serialize_optional_cql_timestamp",
        deserialize_with = "deserialize_optional_cql_timestamp"
    )]
    pub deactivated_at: Option<CqlTimestamp>,
}
