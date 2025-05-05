use crate::application::mappers::model::ModelMapper;
use super::scylla_serialize::{deserialize_cql_timestamp, serialize_cql_timestamp};
use crate::domain::entities::application::Application;
use scylla::value::CqlTimestamp;
use scylla::{DeserializeRow, SerializeRow};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, SerializeRow, DeserializeRow, Serialize, Deserialize)]
pub struct AppcalitionModel {
    pub organization_id: Uuid,
    pub application_id: Uuid,
    pub name: String,
    pub description: String,
    pub client_id: Uuid,
    pub encrypted_client_secret: String,
    pub config: String,
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

impl ModelMapper<Application> for AppcalitionModel {
    fn from_entity(entity: Application) -> Self {
        Self {
            client_id: entity.client_id,
            name: entity.name,
            config: entity.config,
            description: entity.description,
            application_id: entity.application_id,
            organization_id: entity.organization_id,
            encrypted_client_secret: entity.encrypted_client_secret,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
    fn to_entity(&self) -> Application {
        Application {
            client_id: self.client_id,
            name: self.name.clone(),
            config: self.config.clone(),
            description: self.description.clone(),
            application_id: self.application_id,
            organization_id: self.organization_id,
            encrypted_client_secret: self.encrypted_client_secret.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
