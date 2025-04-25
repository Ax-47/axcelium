use argon2::{Argon2, PasswordHash};
use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use argon2::PasswordVerifier;
use redis::Client as RedisClient;
use scylla::client::session::Session;
use uuid::Uuid;
use std::sync::Arc;

use crate::domain::{errors::middleware_errors::{MiddelwareError, MiddelwareResult}, models::apporg_client_id_models::{AppOrgByClientId, CleanAppOrgByClientId}};
pub struct VaildateBearerAuthMiddlewareRepositoryImpl {
    pub cache: Arc<RedisClient>,
    pub database: Arc<Session>,
}
impl VaildateBearerAuthMiddlewareRepositoryImpl {
    pub fn new(cache: Arc<RedisClient>, database: Arc<Session>) -> Self {
        Self { cache, database }
    }
}

#[async_trait]
pub trait VaildateBearerAuthMiddlewareRepository: Send + Sync {
    async fn authentication(&self, token: String)-> MiddelwareResult<CleanAppOrgByClientId>;
    fn decode_base64_to_string(&self, message: String) -> MiddelwareResult<String>;
    fn parse_axcelium_credentials(&self,input: String) -> MiddelwareResult<(String, String)>;
    async fn find_apporg_by_client_id(&self,client:String)->MiddelwareResult<AppOrgByClientId>;

    fn verify_password(&self, password: String, hash: String) -> MiddelwareResult<bool>;
}

#[async_trait]
impl VaildateBearerAuthMiddlewareRepository for VaildateBearerAuthMiddlewareRepositoryImpl {
    async fn authentication(&self, token: String) -> MiddelwareResult<CleanAppOrgByClientId>{
        let decoded_token = self.decode_base64_to_string(token)?;
        let (client_id, client_secret) = self.parse_axcelium_credentials(decoded_token)?;
        let apporg=self.find_apporg_by_client_id(client_id).await?;
        if !self.verify_password(client_secret, apporg.client_secret.clone())?{
            return Err(MiddelwareError { message: "unauth".to_string(), code: 401 })
        }
        let clean_apporg= CleanAppOrgByClientId::from(apporg);
        Ok(clean_apporg)
    }
    fn verify_password(&self, password: String, hash: String) -> MiddelwareResult<bool> {
        let parsed_hash = PasswordHash::new(&hash)?;
        Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
    async fn find_apporg_by_client_id(&self,client:String)-> MiddelwareResult<AppOrgByClientId>{
        let client_id = Uuid::parse_str(&client).map_err(|_| MiddelwareError {
            message: "invalid UUID for client_id".to_string(),
            code: 400,
        })?;

        let query = "
            SELECT client_id, application_id, organization_id, client_secret,
                    organization_name, organization_slug,
                    application_name, application_description,application_config,contact_email,
                    is_active, created_at, updated_at
            FROM axcelium.applications_organization_by_client_id
            WHERE client_id = ?;
        ";

        let row = self
            .database
            .query_unpaged(query, (client_id,))
            .await
            .map_err(|e| MiddelwareError {
                message: format!("DB query failed: {}", e),
                code: 500,
            })?.into_rows_result().map_err(|e| MiddelwareError {
                message: format!("DB query failed: {}", e),
                code: 500,
            })?.first_row::<AppOrgByClientId>()?;

        Ok(row)

    }
    fn decode_base64_to_string(&self, message: String) -> MiddelwareResult<String> {
        let decoded_message = STANDARD.decode(message)?;
        let converted = String::from_utf8(decoded_message)?;
        Ok(converted)
    }
    fn parse_axcelium_credentials(&self,input: String) -> MiddelwareResult<(String, String)> {
        let without_prefix = input
            .strip_prefix("axcelium-core:")
            .ok_or_else(|| MiddelwareError {
                message: "missing axcelium-core prefix".to_string(),
                code: 400,
            })?;
        let parts: Vec<&str> = without_prefix.splitn(2, '_').collect();
        if parts.len() != 2 {
            return Err(MiddelwareError {
                message: "invalid credential format".to_string(),
                code: 400,
            });
        }
        Ok((parts[0].to_string(), parts[1].to_string()))
    }
}
