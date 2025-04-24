use crate::{
    application::dto::user_dto::{CreateUserDTO, CreateUserResponse},
    domain::{
        errors::repositories_errors::{ApiError, CommonError},
        models::apporg_client_id_models::CleanAppOrgByClientId,
    },
    infrastructure::services::user_service::UserService,
};
use actix_web::HttpMessage;
use actix_web::{web, Result};
pub async fn create_user_handle(
    req: actix_web::HttpRequest,
    user_service: web::Data<dyn UserService>,
    post_data: web::Json<CreateUserDTO>,
) -> Result<web::Json<CreateUserResponse>, ApiError> {
    println!("esfkld;a");
    if post_data.password != post_data.repassword {
        return Err(ApiError::from(CommonError {
            message: "they are not same".to_string(),
            code: 400,
        }));
    }
    let apporg = req
        .extensions()
        .get::<CleanAppOrgByClientId>()
        .ok_or_else(|| {
            ApiError::from(CommonError {
                message: "Missing AppOrg data".to_string(),
                code: 500,
            })
        })
        .cloned()?;
    println!("{:?}", apporg);
    let created_user = user_service.create(post_data.into_inner().into()).await?;
    println!("test");
    Ok(web::Json(created_user.into()))
}
