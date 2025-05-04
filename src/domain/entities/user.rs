use chrono::Utc;
use scylla::value::CqlTimestamp;
use uuid::Uuid;

use crate::domain::models::apporg_client_id_models::CleanAppOrgByClientId;

#[derive(Debug, Clone)]
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
        let now = CqlTimestamp(Utc::now().timestamp_millis());

        Self {
            user_id: Uuid::new_v4(),
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
