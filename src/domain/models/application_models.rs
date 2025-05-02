use super::app_config::AppConfig;
use chrono::Utc;
use rand_core::{OsRng, TryRngCore};
use scylla::value::CqlTimestamp;
use scylla::{DeserializeRow, SerializeRow};
use uuid::Uuid;

#[derive(Debug, Clone, DeserializeRow, SerializeRow)]
pub struct Application {
    pub organization_id: Uuid,
    pub application_id: Uuid,
    pub name: String,
    pub description: String,
    pub client_id: Uuid,
    pub encrypted_client_secret: String,
    pub config: String,
    pub created_at: CqlTimestamp,
    pub updated_at: CqlTimestamp,
}

impl Application {
    pub fn new(
        organization_id: Uuid,
        name: String,
        description: String,
        encrypted_client_secret: String,
        config: &AppConfig,
    ) -> Self {
        let now = CqlTimestamp(Utc::now().timestamp_millis());
        Self {
            organization_id,
            application_id: Uuid::new_v4(),
            name,
            description,
            client_id: Uuid::new_v4(),
            encrypted_client_secret,
            config: config.to_string(),
            created_at: now,
            updated_at: now,
        }
    }
    pub fn gen_client_secret() -> Result<[u8; 32], rand_core::OsError> {
        let mut secret = [0u8; 32];
        OsRng.try_fill_bytes(&mut secret)?;
        Ok(secret)
    }
    pub fn set_config(&mut self, config: &str) {
        self.config = config.to_string();
        self.updated_at = CqlTimestamp(Utc::now().timestamp_millis());
    }

    pub fn get_config(&self) -> &str {
        &self.config
    }
}
