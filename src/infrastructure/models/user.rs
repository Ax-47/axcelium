use std::collections::HashMap;

use crate::domain::entities::user::User;
use crate::infrastructure::repositories::database::scylla_serialize::{
    deserialize_cql_timestamp, deserialize_optional_cql_timestamp, serialize_cql_timestamp,
    serialize_optional_cql_timestamp,
};
use chrono::Utc;
use scylla::value::{CqlTimestamp, CqlValue};
use scylla::{DeserializeRow, SerializeRow, SerializeValue};
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
    pub fn to_bind_map(&self) -> HashMap<&'static str, CqlValue> {
        let mut map = HashMap::new();

        map.insert("user_id", CqlValue::Uuid(self.user_id));
        map.insert("organization_id", CqlValue::Uuid(self.organization_id));
        map.insert("application_id", CqlValue::Uuid(self.application_id));
        map.insert("username", CqlValue::Text(self.username.clone()));
        if let Some(ref email) = self.email {
            map.insert("email", CqlValue::Text(email.clone()));
        } else {
            map.insert("email", CqlValue::Empty);
        }
        map.insert(
            "hashed_password",
            CqlValue::Text(self.hashed_password.clone()),
        );
        map.insert("created_at", CqlValue::Timestamp(self.created_at));
        map.insert("updated_at", CqlValue::Timestamp(self.updated_at));
        map.insert("is_active", CqlValue::Boolean(self.is_active));
        map.insert("is_verified", CqlValue::Boolean(self.is_verified));
        map.insert("is_locked", CqlValue::Boolean(self.is_locked));

        if let Some(ts) = self.last_login {
            map.insert("last_login", CqlValue::Timestamp(ts));
        } else {
            map.insert("last_login", CqlValue::Empty);
        }

        map.insert("mfa_enabled", CqlValue::Boolean(self.mfa_enabled));

        if let Some(ts) = self.deactivated_at {
            map.insert("deactivated_at", CqlValue::Timestamp(ts));
        } else {
            map.insert("deactivated_at", CqlValue::Empty);
        }

        map
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

pub struct PaginatedUsersModel {
    pub users: Vec<CleannedUserModel>,
    pub paging_state: Option<Vec<u8>>,
}

#[derive(Debug, Clone, SerializeRow, SerializeValue, DeserializeRow, Serialize, Deserialize)]
pub struct UpdateUserModel {
    pub username: Option<String>,
    pub email: Option<String>,
    pub hashed_password: Option<String>,
    #[serde(
        serialize_with = "serialize_cql_timestamp",
        deserialize_with = "deserialize_cql_timestamp"
    )]
    pub updated_at: CqlTimestamp,
}
impl UpdateUserModel {
    pub fn new(
        username: Option<String>,
        email: Option<String>,
        hashed_password: Option<String>,
    ) -> Self {
        let updated_at = CqlTimestamp(Utc::now().timestamp_millis());
        Self {
            username,
            email,
            hashed_password,
            updated_at,
        }
    }
}
