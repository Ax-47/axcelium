use chrono::Utc;
use scylla::value::CqlTimestamp;
use uuid::Uuid;

pub trait UserValidation {
    fn validate_name(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct User {
    pub user_id: Uuid,
    pub organization_id: Uuid,
    pub application_id: Uuid,

    pub username: String,
    pub email: Option<String>,
    pub hashed_password: String,

    pub created_at: CqlTimestamp,
    pub updated_at: CqlTimestamp,

    pub is_active: bool,
    pub is_verified: bool,
    pub is_locked: bool,
    pub last_login: Option<CqlTimestamp>,
    pub mfa_enabled: bool,
    pub deactivated_at: Option<CqlTimestamp>,
    pub locked_at: Option<CqlTimestamp>,
}
impl User {
    pub fn new(
        application_id: Uuid,
        organization_id: Uuid,
        username: String,
        hashed_password: String,
        email: Option<String>,
    ) -> Self {
        let now = CqlTimestamp(Utc::now().timestamp_millis());
        Self {
            user_id: Uuid::new_v4(),
            application_id,
            organization_id,
            username,
            email,
            hashed_password,
            created_at: now,
            updated_at: now,
            is_active: true,
            is_verified: false,
            is_locked: false,
            last_login: None,
            mfa_enabled: false,
            deactivated_at: None,
            locked_at:None
        }
    }

    pub fn validate_name(&self) -> bool {
        !(self.username.len() <= 2 || self.username.len() >= 50)
    }

    pub fn prepared_email(&self) -> String {
        self.email.clone().unwrap_or_default()
    }

    pub fn touch_updated(&mut self) {
        self.updated_at = CqlTimestamp(Utc::now().timestamp_millis());
    }
}
