use super::super::services::users::create::CreateUserService;
use crate::domain::{
    entities::apporg_client_id::CleanAppOrgByClientId, errors::repositories_errors::ApiError,
};
use actix_web::HttpMessage;
use actix_web::{Result, web};
pub async fn create_refresh_token_handle(
    req: actix_web::HttpRequest,
    user_service: web::Data<dyn CreateUserService>,
    post_data: web::Json<()>,
) -> Result<web::Json<()>, ApiError> {
    let apporg = req
        .extensions()
        .get::<CleanAppOrgByClientId>()
        .ok_or_else(|| ApiError::new("Missing AppOrg data".to_string(), 500))
        .cloned()?;
    Ok(web::Json(()))
}
