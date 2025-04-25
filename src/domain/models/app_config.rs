use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub is_must_name_unique: bool,
    pub can_allow_email_nullable: bool,
}
impl AppConfig {
    pub fn new(is_must_name_unique:bool,can_allow_email_nullable:bool) -> Self {
        Self {
            is_must_name_unique,
            can_allow_email_nullable,
        }
    }
}
impl ToString for AppConfig {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string())
    }
}
