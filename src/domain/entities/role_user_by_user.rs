use uuid::Uuid;
use scylla::value::CqlTimestamp;

#[derive(Debug, Clone)]
pub struct UserRoleByUser {
    pub organization_id: Uuid,
    pub application_id: Uuid,
    pub user_id: Uuid,
    pub role_id: Uuid,
    pub assigned_at: CqlTimestamp,
}