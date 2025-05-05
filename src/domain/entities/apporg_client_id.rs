use chrono::Utc;
use scylla::value::CqlTimestamp;
use uuid::Uuid;

use super::{
    super::value_objects::app_config::AppConfig, application::Application,
    organization::Organization,
};

pub trait HasAppConfig {
    fn get_config(&self) -> Result<AppConfig, serde_json::Error>;
    fn set_config(&mut self, config: &AppConfig) -> Result<(), serde_json::Error>;
}

#[derive(Debug, Clone)]
pub struct CleanAppOrgByClientId {
    pub client_id: Uuid,
    pub application_id: Uuid,
    pub organization_id: Uuid,
    pub organization_name: String,
    pub organization_slug: String,
    pub application_name: String,
    pub application_description: String,
    pub application_config: String,
    pub contact_email: String,
    pub is_active: bool,
    pub created_at: CqlTimestamp,
    pub updated_at: CqlTimestamp,
}

impl HasAppConfig for CleanAppOrgByClientId {
    fn get_config(&self) -> Result<AppConfig, serde_json::Error> {
        serde_json::from_str(&self.application_config)
    }

    fn set_config(&mut self, config: &AppConfig) -> Result<(), serde_json::Error> {
        self.application_config = serde_json::to_string(config)?;
        self.updated_at = CqlTimestamp(Utc::now().timestamp_millis());
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct AppOrgByClientId {
    pub client_id: Uuid,
    pub application_id: Uuid,
    pub organization_id: Uuid,
    pub organization_name: String,
    pub organization_slug: String,
    pub application_name: String,
    pub application_description: String,
    pub application_config: String,
    pub contact_email: String,
    pub is_active: bool,
    pub encrypted_client_secret: String,
    pub created_at: CqlTimestamp,
    pub updated_at: CqlTimestamp,
}
impl AppOrgByClientId {
    pub fn new(org: Organization, app: Application) -> Self {
        let now = CqlTimestamp(Utc::now().timestamp_millis());

        Self {
            encrypted_client_secret: app.encrypted_client_secret,
            client_id: app.client_id,
            application_id: app.application_id,
            organization_id: org.organization_id,
            organization_name: org.name,
            organization_slug: org.slug,
            application_name: app.name,
            application_description: app.description,
            application_config: app.config,
            contact_email: org.contact_email,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }
}

impl HasAppConfig for AppOrgByClientId {
    fn get_config(&self) -> Result<AppConfig, serde_json::Error> {
        serde_json::from_str(&self.application_config)
    }

    fn set_config(&mut self, config: &AppConfig) -> Result<(), serde_json::Error> {
        self.application_config = serde_json::to_string(config)?;
        self.updated_at = CqlTimestamp(Utc::now().timestamp_millis());
        Ok(())
    }
}

impl From<AppOrgByClientId> for CleanAppOrgByClientId {
    fn from(full: AppOrgByClientId) -> Self {
        Self {
            client_id: full.client_id,
            application_id: full.application_id,
            organization_id: full.organization_id,
            organization_name: full.organization_name,
            organization_slug: full.organization_slug,
            application_name: full.application_name,
            application_description: full.application_description,
            application_config: full.application_config,
            contact_email: full.contact_email,
            is_active: full.is_active,
            created_at: full.created_at,
            updated_at: full.updated_at,
        }
    }
}
