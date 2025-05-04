use chrono::Utc;
use scylla::value::CqlTimestamp;
use scylla::{DeserializeRow, SerializeRow};
use uuid::Uuid;

use crate::domain::models::apporg_client_id_models::CleanAppOrgByClientId;
use crate::domain::entities::user::User;

#[derive(Debug, Clone,SerializeRow,DeserializeRow)]
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
