use crate::{
    application::dto::user_dto::{CreateUserDTO, CreateUserResponse},
    domain::errors::repositories_errors::{ApiError, CommonError},
    infrastructure::services::user_service::UserService,
};
use actix_web::{web, Result};

pub async fn create_user_handle(
    user_service: web::Data<dyn UserService>,
    post_data: web::Json<CreateUserDTO>,
) -> Result<web::Json<CreateUserResponse>, ApiError> {
    print!("esfkld;a");
    if post_data.password != post_data.repassword{
        return Err(ApiError::from(CommonError{message:"they are not same".to_string(),code:400}))
    }
    let created_user = user_service.create(post_data.into_inner().into()).await?;
    println!("test");
    Ok(web::Json(created_user.into()))
}
