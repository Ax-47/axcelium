use uuid::Uuid;
use scylla::value::CqlTimestamp;
use scylla::DeserializeRow;
#[derive(Debug,DeserializeRow)]
pub struct Organization {
    pub organization_id: Uuid,
    pub name: String,
    pub slug: String,
    pub contact_email: String,
    pub is_active: bool,
    pub created_at: CqlTimestamp,
    pub updated_at: CqlTimestamp,
}
