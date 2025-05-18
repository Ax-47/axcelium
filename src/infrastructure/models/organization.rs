use crate::application::mappers::model::ModelMapper;
use crate::domain::entities::organization::Organization;
use crate::infrastructure::repositories::database::scylla_serialize::{
    deserialize_cql_timestamp, serialize_cql_timestamp,
};
use scylla::value::CqlTimestamp;
use scylla::{DeserializeRow, SerializeRow};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, SerializeRow, DeserializeRow, Serialize, Deserialize)]
pub struct OrganizationModel {
    pub organization_id: Uuid,
    pub name: String,
    pub slug: String,
    pub contact_email: String,
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

impl ModelMapper<Organization> for OrganizationModel {
    fn from_entity(entity: Organization) -> Self {
        Self {
            name: entity.name,
            slug: entity.slug,
            contact_email: entity.contact_email,
            is_active: entity.is_active,
            organization_id: entity.organization_id,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
    fn to_entity(&self) -> Organization {
        Organization {
            name: self.name.clone(),
            slug: self.slug.clone(),
            contact_email: self.contact_email.clone(),
            is_active: self.is_active,
            organization_id: self.organization_id,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
