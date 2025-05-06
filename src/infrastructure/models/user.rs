use crate::infrastructure::repositories::database::scylla_serialize::{
    deserialize_cql_timestamp, deserialize_optional_cql_timestamp, serialize_cql_timestamp,
    serialize_optional_cql_timestamp,
};
use crate::domain::entities::user::User;
use scylla::value::CqlTimestamp;
use scylla::{DeserializeRow, SerializeRow};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Clone, SerializeRow, DeserializeRow, Serialize, Deserialize)]
pub struct UserModel {
    pub user_id: Uuid,
    pub organization_id: Uuid,
    pub application_id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub hashed_password: String,
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
    pub is_active: bool,
    pub is_verified: bool,
    pub is_locked: bool,
    #[serde(
        serialize_with = "serialize_optional_cql_timestamp",
        deserialize_with = "deserialize_optional_cql_timestamp"
    )]
    pub last_login: Option<CqlTimestamp>,
    pub mfa_enabled: bool,
    #[serde(
        serialize_with = "serialize_optional_cql_timestamp",
        deserialize_with = "deserialize_optional_cql_timestamp"
    )]
    pub deactivated_at: Option<CqlTimestamp>,
}

impl UserModel {
    pub fn from_entity(entity: User) -> Self {
        Self {
            user_id: entity.user_id,
            organization_id: entity.organization_id,
            application_id: entity.application_id,
            username: entity.username,
            email: entity.email,
            hashed_password: entity.hashed_password,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            is_active: entity.is_active,
            is_verified: entity.is_verified,
            is_locked: entity.is_locked,
            last_login: entity.last_login,
            mfa_enabled: entity.mfa_enabled,
            deactivated_at: entity.deactivated_at,
        }
    }

    pub fn to_entity(&self) -> User {
        User {
            user_id: self.user_id,
            organization_id: self.organization_id,
            application_id: self.application_id,
            username: self.username.clone(),
            email: self.email.clone(),
            hashed_password: self.hashed_password.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            is_active: self.is_active,
            is_verified: self.is_verified,
            is_locked: self.is_locked,
            last_login: self.last_login,
            mfa_enabled: self.mfa_enabled,
            deactivated_at: self.deactivated_at,
        }
    }
}

#[derive(Debug, Clone, SerializeRow, DeserializeRow, Serialize, Deserialize)]
pub struct CleannedUserModel {
    pub user_id: Uuid,
    pub organization_id: Uuid,
    pub application_id: Uuid,
    pub username: String,
    pub email: Option<String>,
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
    pub is_active: bool,
    pub is_verified: bool,
    pub is_locked: bool,
    #[serde(
        serialize_with = "serialize_optional_cql_timestamp",
        deserialize_with = "deserialize_optional_cql_timestamp"
    )]
    pub last_login: Option<CqlTimestamp>,
    pub mfa_enabled: bool,
    #[serde(
        serialize_with = "serialize_optional_cql_timestamp",
        deserialize_with = "deserialize_optional_cql_timestamp"
    )]
    pub deactivated_at: Option<CqlTimestamp>,
}
#[derive(Debug, Clone, SerializeRow, DeserializeRow, Serialize, Deserialize)]
pub struct FoundUserModel {
    pub user_id: Uuid,
    pub username: String,
    pub email: Option<String>,
}

pub struct PaginatedUsers {
    pub users: Vec<CleannedUserModel>,
    pub paging_state: Option<Vec<u8>>,
}
