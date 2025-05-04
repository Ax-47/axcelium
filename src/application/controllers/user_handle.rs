use crate::{
    application::dto::{payload::user::CreateUserPayload, response::user::CreateUserResponse},
    domain::{
        errors::repositories_errors::ApiError,
        models::apporg_client_id_models::CleanAppOrgByClientId,
    },
    infrastructure::services::user_service::UserService,
};
use actix_web::HttpMessage;
use actix_web::{web, Result};
pub async fn create_user_handle(
    req: actix_web::HttpRequest,
    user_service: web::Data<dyn UserService>,
    post_data: web::Json<CreateUserPayload>,
) -> Result<web::Json<CreateUserResponse>, ApiError> {
    let apporg = req
        .extensions()
        .get::<CleanAppOrgByClientId>()
        .ok_or_else(|| ApiError::new("Missing AppOrg data".to_string(), 500))
        .cloned()?;
    let created_user = user_service
        .create(apporg, post_data.into_inner().into())
        .await?;
    Ok(web::Json(created_user.into()))
}
