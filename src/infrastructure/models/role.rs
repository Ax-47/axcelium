use std::collections::HashSet;

use scylla::{DeserializeRow, SerializeRow, value::CqlTimestamp};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infrastructure::repositories::database::scylla_serialize::{
    deserialize_cql_timestamp, serialize_cql_timestamp,
};
#[derive(Debug, Clone, SerializeRow, DeserializeRow, Serialize, Deserialize)]
pub struct RoleModel {
    pub organization_id: Uuid,
    pub application_id: Uuid,
    pub role_id: Uuid,
    pub name: String,
    pub description: String,
    pub permissions: HashSet<String>,

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
}

#[derive(Debug, Clone, SerializeRow, DeserializeRow, Serialize, Deserialize)]
pub struct RoleAssignmentModel {
    pub organization_id: Uuid,
    pub application_id: Uuid,
    pub role_id: Uuid,
    pub user_id: Uuid,

    #[serde(
        serialize_with = "serialize_cql_timestamp",
        deserialize_with = "deserialize_cql_timestamp"
    )]
    pub assigned_at: CqlTimestamp,
}

#[derive(Debug, Clone, SerializeRow, DeserializeRow, Serialize, Deserialize)]
pub struct UserRoleModel {
    pub role_id: Uuid,
    pub role_name: String,
    pub role_description: String,
    pub role_permissions: HashSet<String>,
    #[serde(
        serialize_with = "serialize_cql_timestamp",
        deserialize_with = "deserialize_cql_timestamp"
    )]
    pub assigned_at: CqlTimestamp,
}

#[derive(Debug, Clone, SerializeRow, DeserializeRow, Serialize, Deserialize)]
pub struct RoleUserModel {
    pub user_id: Uuid,
    pub user_name: String,
    pub user_email: String,
    #[serde(
        serialize_with = "serialize_cql_timestamp",
        deserialize_with = "deserialize_cql_timestamp"
    )]
    pub assigned_at: CqlTimestamp,
}
#[derive(Debug, Clone, SerializeRow, DeserializeRow, Serialize, Deserialize)]
pub struct SelectedRoleByIdModel {
    pub name: String,
    pub description: String,
    pub permissions: HashSet<String>,

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
}
