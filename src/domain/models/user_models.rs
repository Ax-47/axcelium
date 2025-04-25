use chrono::Utc;
use scylla::value::CqlTimestamp;
use scylla::{DeserializeRow, SerializeRow};
use serde::Serialize;
use uuid::Uuid;

use super::apporg_client_id_models::CleanAppOrgByClientId;
#[derive(Serialize, Clone)]
pub struct CreateUser {
    pub username: String,
    pub email: Option<String>,
    pub password: String,
}

#[derive(Debug, Clone, DeserializeRow, SerializeRow)]
pub struct CreatedUser {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
}
#[derive(Debug, Clone, DeserializeRow, SerializeRow)]
pub struct User {
    pub user_id: Uuid,
    pub organization_id: Uuid,
    pub application_id: Uuid,

    pub username: String,
    pub email: Option<String>,
    pub password_hash: String,

    pub created_at: CqlTimestamp,
    pub updated_at: CqlTimestamp,

    pub is_active: bool,
    pub is_verified: bool,
    pub is_locked: bool,
    pub last_login: Option<CqlTimestamp>,
    pub mfa_enabled: bool,
    pub deactivated_at: Option<CqlTimestamp>,
}
impl User {
    pub fn new(
        apporg: CleanAppOrgByClientId,
        username: String,
        hashed_password: String,
        email: Option<String>,
    ) -> Self {
        let user_id = Uuid::new_v4();
        let now = CqlTimestamp(Utc::now().timestamp_millis());
        Self {
            user_id,
            application_id: apporg.application_id,
            organization_id: apporg.organization_id,
            username,
            email,
            password_hash: hashed_password,
            created_at: now,
            updated_at: now,
            is_active: true,
            is_verified: false,
            is_locked: false,
            last_login: None,
            mfa_enabled: false,
            deactivated_at: None,
        }
    }
    pub fn prepared_email(&self) -> String {
        self.email.clone().unwrap_or_default()
    }
}
#[derive(Debug, Clone, DeserializeRow, SerializeRow)]
pub struct UserOrganization {
    pub user_id: Uuid,
    pub organization_id: Uuid,

    pub role: String,
    pub username: String,
    pub user_email: Option<String>,

    pub organization_name: String,
    pub organization_slug: String,
    pub contact_email: String,

    pub joined_at: CqlTimestamp,
}

impl UserOrganization {
    pub fn new(apporg: CleanAppOrgByClientId, user: User) -> Self {
        let now = CqlTimestamp(Utc::now().timestamp_millis());
        Self {
            user_id: user.user_id,
            organization_id: apporg.organization_id,
            username: user.username,
            user_email: user.email,
            role: "MEMBER".to_string(),

            organization_name: apporg.organization_name,
            organization_slug: apporg.organization_slug,
            contact_email: apporg.contact_email,

            joined_at: now,
        }
    }
}
