use uuid::Uuid;
use scylla::value::CqlTimestamp;
use scylla::{DeserializeRow, SerializeRow};
use chrono::Utc;

#[derive(Debug, Clone, DeserializeRow, SerializeRow)]
pub struct Application {
    pub organization_id: Uuid,
    pub application_id: Uuid,
    pub name: String,
    pub description: String,
    pub client_id: Uuid,
    pub client_secret: String,
    pub created_at: CqlTimestamp,
    pub updated_at: CqlTimestamp,
}

impl Application {
    pub fn new(organization_id:Uuid,name:String,description:String,hashed_client_secret: String) -> Self {
        let now = CqlTimestamp(Utc::now().timestamp_millis());
        Self {
            organization_id ,
            application_id: Uuid::new_v4(),
            name,
            description,
            client_id: Uuid::new_v4(),
            client_secret: hashed_client_secret,
            created_at: now,
            updated_at: now,
        }
    }
}
