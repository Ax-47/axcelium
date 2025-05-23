use uuid::Uuid;
use std::collections::HashSet;
use scylla::value::CqlTimestamp;

#[derive(Debug, Clone)]
pub struct RoleByApp {
    pub organization_id: Uuid,
    pub application_id: Uuid,
    pub role_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub permissions: HashSet<String>,
    pub created_at: CqlTimestamp,
    pub updated_at: CqlTimestamp,
}
