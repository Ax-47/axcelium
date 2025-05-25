use std::collections::HashSet;
use serde::Deserialize;
use uuid::Uuid;


#[derive(Debug, Deserialize)]
pub struct CreateRolePayload {
    pub name: String,
    pub description: Option<String>,
    pub permissions: HashSet<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRolePayload {
    pub name: String,
    pub description: Option<String>,
    pub permissions: HashSet<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetRoleIdQuery{
    pub role_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct GetRolesByUserPayload {
    pub user_id: Uuid,
}