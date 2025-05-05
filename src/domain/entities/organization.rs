use uuid::Uuid;
use scylla::value::CqlTimestamp;
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct Organization {
    pub organization_id: Uuid,
    pub name: String,
    pub slug: String,
    pub contact_email: String,
    pub is_active: bool,
    pub created_at: CqlTimestamp,
    pub updated_at: CqlTimestamp,
}

impl Organization {
    pub fn new(name:String,slug:String,contact_email:String) -> Self {
        let now = CqlTimestamp(Utc::now().timestamp_millis());

        Self {
            organization_id: Uuid::new_v4(),
            name,
            slug,
            contact_email,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }
}