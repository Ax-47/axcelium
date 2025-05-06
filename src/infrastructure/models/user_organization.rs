use crate::application::mappers::model::ModelMapper;
use crate::infrastructure::repositories::database::scylla_serialize::{
    deserialize_cql_timestamp, serialize_cql_timestamp,
};
use crate::domain::entities::user_organization::UserOrganization;
use scylla::value::CqlTimestamp;
use scylla::{DeserializeRow, SerializeRow};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Clone, SerializeRow, DeserializeRow, Serialize, Deserialize)]
pub struct UserOrganizationModel {
    pub user_id: Uuid,
    pub organization_id: Uuid,
    pub role: String,
    pub username: String,
    pub user_email: Option<String>,
    pub organization_name: String,
    pub organization_slug: String,
    pub contact_email: String,
    #[serde(
        serialize_with = "serialize_cql_timestamp",
        deserialize_with = "deserialize_cql_timestamp"
    )]
    pub joined_at: CqlTimestamp,
}

impl ModelMapper<UserOrganization> for UserOrganizationModel {
    fn from_entity(entity: UserOrganization) -> Self {
        Self {
            user_id: entity.user_id,
            organization_id: entity.organization_id,
            role: entity.role,
            username: entity.username,
            user_email: entity.user_email,
            organization_name: entity.organization_name,
            organization_slug: entity.organization_slug,
            contact_email:entity.contact_email,
            joined_at: entity.joined_at,
        }
    }

    fn to_entity(&self) -> UserOrganization {
        UserOrganization {
            user_id: self.user_id,
            organization_id: self.organization_id,
            role: self.role.clone(),
            username: self.username.clone(),
            user_email: self.user_email.clone(),
            organization_name: self.organization_name.clone(),
            organization_slug: self.organization_slug.clone(),
            contact_email:self.contact_email.clone(),
            joined_at: self.joined_at,
        }
    }
}
