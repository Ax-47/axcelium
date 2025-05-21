use crate::application::dto::payload::refresh_token::{CreateTokenPayload, GetUserIdQuery};
use crate::application::services::refresh_token::create::CreateRefreshTokenService;
use crate::domain::{
    entities::apporg_client_id::CleanAppOrgByClientId, errors::repositories_errors::ApiError,
};
use actix_web::HttpMessage;
use actix_web::{Result, web};
pub async fn create_refresh_token_handle(
    req: actix_web::HttpRequest,
    path: web::Path<GetUserIdQuery>,
    post_data: web::Json<CreateTokenPayload>,
    token_service: web::Data<dyn CreateRefreshTokenService>,
) -> Result<web::Json<()>, ApiError> {
    let apporg = req
        .extensions()
        .get::<CleanAppOrgByClientId>()
        .ok_or_else(|| ApiError::new("Missing AppOrg data".to_string(), 500))
        .cloned()?;
    token_service.execute(apporg,path.user_id,post_data.paseto_key.clone()).await?;
    Ok(web::Json(()))
}
