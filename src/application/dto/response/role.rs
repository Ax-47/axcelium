use crate::infrastructure::{models::role::{RoleUserModel, SelectedRoleByAppModel, UserRoleModel}, repositories::database::scylla_serialize::serialize_cql_timestamp};
use scylla::value::CqlTimestamp;
use serde::Serialize;
use std::collections::HashSet;
#[derive(Debug, Clone, Serialize)]
pub struct GetRoleResponse {
    pub name: String,
    pub description: Option<String>,
    pub permissions: HashSet<String>,
    #[serde(serialize_with = "serialize_cql_timestamp")]
    pub created_at: CqlTimestamp,
    #[serde(serialize_with = "serialize_cql_timestamp")]
    pub updated_at: CqlTimestamp,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetRolesByUserResponse {
    pub roles: Vec<UserRoleModel>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetRolesByAppResponse {
    pub roles: Vec<SelectedRoleByAppModel>,
}
#[derive(Debug, Clone, Serialize)]
pub struct GetUsersByRoleResponse {
    pub users: Vec<RoleUserModel>,
}