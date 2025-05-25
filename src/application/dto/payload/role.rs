use serde::Deserialize;
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateRolePayload {
    pub name: String,
    pub description: Option<String>,
    pub permissions: HashSet<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRolePayload {
    pub name: Option<String>,
    pub description: Option<String>,
    pub permissions: Option<HashSet<String>>,
}

#[derive(Debug, Deserialize)]
pub struct GetRoleIdQuery {
    pub role_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct GetRolesByUserPayload {
    pub user_id: Uuid,
}


#[derive(Debug, Deserialize)]
pub struct AssignPayload {
    pub name: Uuid,
}