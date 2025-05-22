use crate::application::dto::payload::refresh_token::{CreateTokenPayload, RotateTokenPayload};
use crate::application::dto::response::refresh_token::CreateTokenResponse;
use crate::application::services::refresh_token::create::CreateRefreshTokenService;
use crate::application::services::refresh_token::rotate::RotateRefreshTokenService;
use crate::domain::{
    entities::apporg_client_id::CleanAppOrgByClientId, errors::repositories_errors::ApiError,
};
use actix_web::HttpMessage;
use actix_web::{Result, web};
pub async fn create_refresh_token_handle(
    req: actix_web::HttpRequest,
    post_data: web::Json<CreateTokenPayload>,
    token_service: web::Data<dyn CreateRefreshTokenService>,
) -> Result<web::Json<CreateTokenResponse>, ApiError> {
    let apporg = req
        .extensions()
        .get::<CleanAppOrgByClientId>()
        .ok_or_else(|| ApiError::new("Missing AppOrg data".to_string(), 500))
        .cloned()?;
    let token = token_service
        .execute(
            apporg,
            post_data.user_id.clone(),
            post_data.private_key.clone(),
        )
        .await?;
    Ok(web::Json(token))
}
pub async fn rotate_refresh_token_handle(
    req: actix_web::HttpRequest,
    post_data: web::Json<RotateTokenPayload>,
    token_service: web::Data<dyn RotateRefreshTokenService>,
) -> Result<web::Json<CreateTokenResponse>, ApiError> {
    let apporg = req
        .extensions()
        .get::<CleanAppOrgByClientId>()
        .ok_or_else(|| ApiError::new("Missing AppOrg data".to_string(), 500))
        .cloned()?;
    let res=    token_service
        .execute(
            apporg,
            post_data.refresh_token.clone(),
            post_data.public_key.clone(),
            post_data.private_key.clone(),
        )
        .await?;
    Ok(web::Json(res))
}
