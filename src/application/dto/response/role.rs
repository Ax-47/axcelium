use crate::infrastructure::repositories::database::scylla_serialize::serialize_cql_timestamp;
use scylla::value::CqlTimestamp;
use serde::Serialize;
use std::collections::HashSet;
#[derive(Debug, Clone, Serialize)]
pub struct GetRoleResponse {
    pub name: String,
    pub description: String,
    pub permissions: HashSet<String>,
    #[serde(serialize_with = "serialize_cql_timestamp")]
    pub created_at: CqlTimestamp,
    #[serde(serialize_with = "serialize_cql_timestamp")]
    pub updated_at: CqlTimestamp,
}
