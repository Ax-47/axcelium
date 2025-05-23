use crate::{
    application::{
        dto::{payload::role::{CreateRolePayload, GetRoleIdQuery}, response::{refresh_token::SimpleResponse, role::GetRoleResponse}},
        services::roles::{create_roles::CreateRoleService, get::GetRoleService},
    },
    domain::{
        entities::apporg_client_id::CleanAppOrgByClientId, errors::repositories_errors::ApiError,
    },
};
use actix_web::HttpMessage;
use actix_web::{Result, web};
pub async fn create_role_handler(
    req: actix_web::HttpRequest,
    role_service: web::Data<dyn CreateRoleService>,
    post_data: web::Json<CreateRolePayload>,
) -> Result<web::Json<SimpleResponse>> {
    let apporg = req
        .extensions()
        .get::<CleanAppOrgByClientId>()
        .ok_or_else(|| ApiError::new("Missing AppOrg data".to_string(), 500))
        .cloned()?;
    let res = role_service.execute(apporg, post_data.into_inner()).await?;
    Ok(web::Json(res))
}

pub async fn get_role_handler(
    req: actix_web::HttpRequest,
    path: web::Path<GetRoleIdQuery>,
    role_service: web::Data<dyn GetRoleService>,
) -> Result<web::Json<GetRoleResponse>> {
    let apporg = req
        .extensions()
        .get::<CleanAppOrgByClientId>()
        .ok_or_else(|| ApiError::new("Missing AppOrg data".to_string(), 500))
        .cloned()?;
    let res = role_service.execute(apporg, path.role_id).await?;
    Ok(web::Json(res))
}