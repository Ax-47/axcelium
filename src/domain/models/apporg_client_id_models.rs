use chrono::Utc;
use scylla::value::CqlTimestamp;
use scylla::{DeserializeRow, SerializeRow};
use uuid::Uuid;
use super::application_models::Application;
use super::organization_models::Organization;

#[derive(Debug, Clone, DeserializeRow, SerializeRow)]
pub struct AppOrgByClientId {
    pub client_id: Uuid,
    pub application_id: Uuid,
    pub organization_id: Uuid,
    pub client_secret: String,
    pub organization_name: String,
    pub organization_slug: String,
    pub application_name: String,
    pub application_description: String,
    pub is_active: bool,
    pub created_at: CqlTimestamp,
    pub updated_at: CqlTimestamp,
}

impl AppOrgByClientId {
    pub fn new(org: Organization, app: Application) -> Self {
        let now = CqlTimestamp(Utc::now().timestamp_millis());
        
        Self {
            client_id: app.client_id,
            application_id: app.application_id,
            organization_id: org.organization_id,
            client_secret: app.client_secret,
            organization_name: org.name,
            organization_slug: org.slug,
            application_name: app.name,
            application_description: app.description,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }
}
#[derive(Debug, Clone, DeserializeRow, SerializeRow)]
pub struct CleanAppOrgByClientId {
    pub client_id: Uuid,
    pub application_id: Uuid,
    pub organization_id: Uuid,
    pub organization_name: String,
    pub organization_slug: String,
    pub application_name: String,
    pub application_description: String,
    pub is_active: bool,
    pub created_at: CqlTimestamp,
    pub updated_at: CqlTimestamp,
}
impl From<AppOrgByClientId> for CleanAppOrgByClientId {
    fn from(app_org: AppOrgByClientId) -> Self {
        CleanAppOrgByClientId {
            client_id: app_org.client_id,
            application_id: app_org.application_id,
            organization_id: app_org.organization_id,
            organization_name: app_org.organization_name,
            organization_slug: app_org.organization_slug,
            application_name: app_org.application_name,
            application_description: app_org.application_description,
            is_active: app_org.is_active,
            created_at: app_org.created_at,
            updated_at: app_org.updated_at,
        }
    }
}