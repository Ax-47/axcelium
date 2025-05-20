use uuid::Uuid;
use scylla::value::CqlTimestamp;
#[derive(Debug, Clone)]
pub struct RefreshToken {
    pub token_id: Uuid,
    pub application_id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub encrypted_token_secret: String,
    pub token_version: String,
    pub parent_version: Option<String>,
    pub issued_at: CqlTimestamp,
    pub expires_at: CqlTimestamp,
    pub revoked: bool,
}