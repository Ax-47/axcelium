use crate::{
    application::{
        dto::{
            payload::role::{
                CreateRolePayload, GetRoleIdQuery, GetRolesByUserPayload, UpdateRolePayload,
            },
            response::{
                refresh_token::SimpleResponse,
                role::{
                    GetRoleResponse, GetRolesByAppResponse, GetRolesByUserResponse,
                    GetUsersByRoleResponse,
                },
            },
        },
        services::roles::{
            assign::AssignService, create_roles::CreateRoleService, delete_role::DeleteRoleService,
            get_role_by_app::GetRoleByAppService, get_roles_by_app::GetRolesByAppService,
            get_roles_by_user::GetRolesByUserService, get_users_by_role::GetUsersByRoleService,
            update_role::UpdateRoleService,
        },
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

pub async fn delete_role_handler(
    req: actix_web::HttpRequest,
    path: web::Path<GetRoleIdQuery>,
    role_service: web::Data<dyn DeleteRoleService>,
) -> Result<web::Json<SimpleResponse>> {
    let apporg = req
        .extensions()
        .get::<CleanAppOrgByClientId>()
        .ok_or_else(|| ApiError::new("Missing AppOrg data".to_string(), 500))
        .cloned()?;
    let res = role_service.execute(apporg, path.role_id).await?;
    Ok(web::Json(res))
}

pub async fn assign_handler(
    req: actix_web::HttpRequest,
    path: web::Path<GetRoleIdQuery>,
    post_data: web::Json<GetRolesByUserPayload>,
    role_service: web::Data<dyn AssignService>,
) -> Result<web::Json<SimpleResponse>> {
    let apporg = req
        .extensions()
        .get::<CleanAppOrgByClientId>()
        .ok_or_else(|| ApiError::new("Missing AppOrg data".to_string(), 500))
        .cloned()?;
    let res = role_service
        .execute(apporg, path.role_id, post_data.user_id)
        .await?;
    Ok(web::Json(res))
}
pub async fn get_role_by_app_handler(
    req: actix_web::HttpRequest,
    path: web::Path<GetRoleIdQuery>,
    role_service: web::Data<dyn GetRoleByAppService>,
) -> Result<web::Json<GetRoleResponse>> {
    let apporg = req
        .extensions()
        .get::<CleanAppOrgByClientId>()
        .ok_or_else(|| ApiError::new("Missing AppOrg data".to_string(), 500))
        .cloned()?;
    let res = role_service.execute(apporg, path.role_id).await?;
    Ok(web::Json(res))
}

pub async fn update_role_handler(
    req: actix_web::HttpRequest,
    path: web::Path<GetRoleIdQuery>,
    post_data: web::Json<UpdateRolePayload>,
    role_service: web::Data<dyn UpdateRoleService>,
) -> Result<web::Json<SimpleResponse>> {
    let apporg = req
        .extensions()
        .get::<CleanAppOrgByClientId>()
        .ok_or_else(|| ApiError::new("Missing AppOrg data".to_string(), 500))
        .cloned()?;
    let res = role_service
        .execute(apporg, path.role_id, post_data.into_inner())
        .await?;
    Ok(web::Json(res))
}
pub async fn get_roles_by_app_handler(
    req: actix_web::HttpRequest,
    role_service: web::Data<dyn GetRolesByAppService>,
) -> Result<web::Json<GetRolesByAppResponse>> {
    let apporg = req
        .extensions()
        .get::<CleanAppOrgByClientId>()
        .ok_or_else(|| ApiError::new("Missing AppOrg data".to_string(), 500))
        .cloned()?;
    let res = role_service.execute(apporg).await?;
    Ok(web::Json(res))
}
pub async fn get_roles_by_user_handler(
    req: actix_web::HttpRequest,
    post_data: web::Json<GetRolesByUserPayload>,
    role_service: web::Data<dyn GetRolesByUserService>,
) -> Result<web::Json<GetRolesByUserResponse>> {
    let apporg = req
        .extensions()
        .get::<CleanAppOrgByClientId>()
        .ok_or_else(|| ApiError::new("Missing AppOrg data".to_string(), 500))
        .cloned()?;
    let res = role_service.execute(apporg, post_data.user_id).await?;
    Ok(web::Json(res))
}

pub async fn get_users_by_role_handler(
    req: actix_web::HttpRequest,
    path: web::Path<GetRoleIdQuery>,
    role_service: web::Data<dyn GetUsersByRoleService>,
) -> Result<web::Json<GetUsersByRoleResponse>> {
    let apporg = req
        .extensions()
        .get::<CleanAppOrgByClientId>()
        .ok_or_else(|| ApiError::new("Missing AppOrg data".to_string(), 500))
        .cloned()?;
    let res = role_service.execute(apporg, path.role_id).await?;
    Ok(web::Json(res))
}
