use super::app_config::AppConfig;
use super::application_models::Application;
use super::organization_models::Organization;
use chrono::Utc;
use scylla::value::CqlTimestamp;
use scylla::{DeserializeRow, SerializeRow};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;
fn serialize_cql_timestamp<S>(ts: &CqlTimestamp, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i64(ts.0) // timestamp in milliseconds
}
fn deserialize_cql_timestamp<'de, D>(deserializer: D) -> Result<CqlTimestamp, D::Error>
where
    D: Deserializer<'de>,
{
    let millis = i64::deserialize(deserializer)?;
    Ok(CqlTimestamp(millis))
}
#[derive(Debug, Clone, DeserializeRow, SerializeRow, Serialize, Deserialize)]
pub struct AppOrgByClientId {
    pub client_id: Uuid,
    pub application_id: Uuid,
    pub organization_id: Uuid,
    pub organization_name: String,
    pub organization_slug: String,
    pub application_name: String,
    pub application_description: String,
    pub encrypted_client_secret: String,
    pub application_config: String,
    pub contact_email:String,
    pub is_active: bool,
    #[serde(
        serialize_with = "serialize_cql_timestamp",
        deserialize_with = "deserialize_cql_timestamp"
    )]
    pub created_at: CqlTimestamp,
    #[serde(
        serialize_with = "serialize_cql_timestamp",
        deserialize_with = "deserialize_cql_timestamp"
    )]
    pub updated_at: CqlTimestamp,
}

impl AppOrgByClientId {
    pub fn new(org: Organization, app: Application) -> Self {
        let now = CqlTimestamp(Utc::now().timestamp_millis());

        Self {
            client_id: app.client_id,
            application_id: app.application_id,
            organization_id: org.organization_id,
            organization_name: org.name,
            organization_slug: org.slug,
            application_name: app.name,
            encrypted_client_secret: app.encrypted_client_secret,
            application_description: app.description,
            application_config: app.config,
            contact_email: org.contact_email,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }
}
impl AppOrgByClientId {
    pub fn set_config(&mut self, config: &AppConfig) -> Result<(), serde_json::Error> {
        self.application_config = serde_json::to_string(config)?;
        self.updated_at = CqlTimestamp(Utc::now().timestamp_millis());
        Ok(())
    }

    pub fn get_config(&self) -> Result<AppConfig, serde_json::Error> {
        serde_json::from_str::<AppConfig>(&self.application_config)
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
    pub application_config: String,
    pub contact_email:String,
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
            application_config: app_org.application_config,
            contact_email: app_org.contact_email,
            is_active: app_org.is_active,
            created_at: app_org.created_at,
            updated_at: app_org.updated_at,
        }
    }
}

impl CleanAppOrgByClientId {
    pub fn set_config(&mut self, config: &AppConfig) -> Result<(), serde_json::Error> {
        self.application_config = serde_json::to_string(config)?;
        self.updated_at = CqlTimestamp(Utc::now().timestamp_millis());
        Ok(())
    }

    pub fn get_config(&self) -> Result<AppConfig, serde_json::Error> {
        serde_json::from_str::<AppConfig>(&self.application_config)
    }
}
