use super::super::services::users::create::CreateUserService;
use crate::{
    application::{
        dto::{
            payload::user::{CreateUserPayload, GetUserQuery, PaginationQuery, UpdateUserPayload},
            response::user::{
                CreateUserResponse, GetUserResponse, GetUsersResponse, UpdateUsersResponse,
            },
        },
        services::users::{
            delete::DeleteUserService, get_user::GetUserService, get_users::GetUsersService,
            update_user::UpdateUserService,
        },
    },
    domain::{
        entities::apporg_client_id::CleanAppOrgByClientId, errors::repositories_errors::ApiError,
    },
};
use actix_web::HttpMessage;
use actix_web::{web, Result};
use validator::Validate;
pub async fn create_user_handle(
    req: actix_web::HttpRequest,
    user_service: web::Data<dyn CreateUserService>,
    post_data: web::Json<CreateUserPayload>,
) -> Result<web::Json<CreateUserResponse>, ApiError> {
    post_data
        .validate()
        .map_err(|e| ApiError::new(e.to_string(), 400))?;
    let apporg = req
        .extensions()
        .get::<CleanAppOrgByClientId>()
        .ok_or_else(|| ApiError::new("Missing AppOrg data".to_string(), 500))
        .cloned()?;
    let created_user = user_service
        .execute(apporg, post_data.into_inner().into())
        .await?;
    Ok(web::Json(created_user.into()))
}

pub async fn get_users_handle(
    req: actix_web::HttpRequest,
    query: web::Query<PaginationQuery>,
    user_service: web::Data<dyn GetUsersService>,
) -> Result<web::Json<GetUsersResponse>, ApiError> {
    let apporg = req
        .extensions()
        .get::<CleanAppOrgByClientId>()
        .ok_or_else(|| ApiError::new("Missing AppOrg data".to_string(), 500))
        .cloned()?;
    let page_size = query.page_size.unwrap_or(20);
    let paging_state = query.paging_state.clone();
    let created_user = user_service
        .execute(
            apporg.organization_id,
            apporg.application_id,
            page_size,
            paging_state,
        )
        .await?;
    Ok(web::Json(created_user))
}

pub async fn get_user_handle(
    req: actix_web::HttpRequest,
    path: web::Path<GetUserQuery>,
    user_service: web::Data<dyn GetUserService>,
) -> Result<web::Json<GetUserResponse>, ApiError> {
    let apporg = req
        .extensions()
        .get::<CleanAppOrgByClientId>()
        .ok_or_else(|| ApiError::new("Missing AppOrg data".to_string(), 500))
        .cloned()?;
    let created_user = user_service
        .execute(apporg.organization_id, apporg.application_id, path.user_id)
        .await?;
    Ok(web::Json(created_user))
}

pub async fn update_user_handle(
    req: actix_web::HttpRequest,
    path: web::Path<GetUserQuery>,
    post_data: web::Json<UpdateUserPayload>,
    user_service: web::Data<dyn UpdateUserService>,
) -> Result<web::Json<UpdateUsersResponse>, ApiError> {
    let user: UpdateUserPayload = post_data.into_inner().into();
    if user.email.is_some() && user.password.is_some() && user.username.is_some() {
        return Err(ApiError::new("empty input".to_string(), 400));
    }
    let apporg = req
        .extensions()
        .get::<CleanAppOrgByClientId>()
        .ok_or_else(|| ApiError::new("Missing AppOrg data".to_string(), 500))
        .cloned()?;
    let created_user = user_service
        .execute(
            apporg.organization_id,
            apporg.application_id,
            path.user_id,
            user,
        )
        .await?;
    Ok(web::Json(created_user))
}

pub async fn delate_user_handle(
    req: actix_web::HttpRequest,
    path: web::Path<GetUserQuery>,
    user_service: web::Data<dyn DeleteUserService>,
) -> Result<web::Json<UpdateUsersResponse>, ApiError> {
    let apporg = req
        .extensions()
        .get::<CleanAppOrgByClientId>()
        .ok_or_else(|| ApiError::new("Missing AppOrg data".to_string(), 500))
        .cloned()?;
    let created_user = user_service
        .execute(apporg.organization_id, apporg.application_id, path.user_id)
        .await?;
    Ok(web::Json(created_user))
}
