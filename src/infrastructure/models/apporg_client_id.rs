use crate::domain::entities::apporg_client_id::AppOrgByClientId;
use crate::application::mappers::model::ModelMapper;
use super::scylla_serialize::{deserialize_cql_timestamp, serialize_cql_timestamp};
use scylla::value::CqlTimestamp;
use scylla::{DeserializeRow, SerializeRow};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, SerializeRow, DeserializeRow, Serialize, Deserialize)]
pub struct AppOrgModel {
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

impl ModelMapper<AppOrgByClientId> for AppOrgModel {
    fn from_entity(entity: AppOrgByClientId) -> Self {
        Self {
            client_id: entity.client_id,
            application_id: entity.application_id,
            organization_id: entity.organization_id,
            organization_name: entity.organization_name,
            organization_slug: entity.organization_slug,
            application_name: entity.application_name,
            application_description: entity.application_description,
            application_config: entity.application_config,
            contact_email: entity.contact_email,
            encrypted_client_secret: entity.encrypted_client_secret,
            is_active: entity.is_active,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
    fn to_entity(&self) -> AppOrgByClientId {
        AppOrgByClientId {
            client_id: self.client_id,
            application_id: self.application_id,
            organization_id: self.organization_id,
            organization_name: self.organization_name.clone(),
            organization_slug: self.organization_slug.clone(),
            application_name: self.application_name.clone(),
            application_description: self.application_description.clone(),
            application_config: self.application_config.clone(),
            contact_email: self.contact_email.clone(),
            encrypted_client_secret: self.encrypted_client_secret.clone(),
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
