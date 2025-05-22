use crate::application::dto::payload::refresh_token::{
    CreateTokenPayload, GetTokenQuery, GetUserQuery, PaginationRefreshTokensByUserQuery, RotateTokenPayload
};
use crate::application::dto::response::refresh_token::{CreateTokenResponse, GetRefreshTokensResponse, SimpleResponse};
use crate::application::services::refresh_token::create::CreateRefreshTokenService;
use crate::application::services::refresh_token::get::GetRefreshTokenService;
use crate::application::services::refresh_token::revoke::RevokeRefreshTokenService;
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
    let res = token_service
        .execute(
            apporg,
            post_data.refresh_token.clone(),
            post_data.public_key.clone(),
            post_data.private_key.clone(),
        )
        .await?;
    Ok(web::Json(res))
}

pub async fn revoke_refresh_token_handle(
    req: actix_web::HttpRequest,
    path: web::Path<GetTokenQuery>,
    token_service: web::Data<dyn RevokeRefreshTokenService>,
) -> Result<web::Json<SimpleResponse>, ApiError> {
    let apporg = req
        .extensions()
        .get::<CleanAppOrgByClientId>()
        .ok_or_else(|| ApiError::new("Missing AppOrg data".to_string(), 500))
        .cloned()?;
    let res = token_service.execute(apporg, path.token_id).await?;
    Ok(web::Json(res))
}

pub async fn get_refresh_token_handle(
    req: actix_web::HttpRequest,

    path: web::Path<GetUserQuery>,
    query: web::Query<PaginationRefreshTokensByUserQuery>,
    token_service: web::Data<dyn GetRefreshTokenService>,
) -> Result<web::Json<GetRefreshTokensResponse>, ApiError> {
    let apporg = req
        .extensions()
        .get::<CleanAppOrgByClientId>()
        .ok_or_else(|| ApiError::new("Missing AppOrg data".to_string(), 500))
        .cloned()?;

    let page_size = query.page_size.unwrap_or(20);
    let paging_state = query.paging_state.clone();
    let user_id = path.user_id.clone();
    let res =token_service.execute(apporg, user_id, page_size, paging_state).await?;
    Ok(web::Json(res))
}
