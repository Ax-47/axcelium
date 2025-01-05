use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CommonError {
    pub message: String,
    pub code: u32,
}

impl std::fmt::Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}, Code: {}", self.message, self.code)
    }
}

#[derive(Debug)]
pub struct RepositoryError {
    pub message: String,
}
#[derive(Debug)]
pub struct ApiError(CommonError);

impl From<CommonError> for ApiError {
    fn from(error: CommonError) -> ApiError {
        ApiError(error)
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl actix_web::ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::BadRequest().json(&self.0)
    }
}
pub type RepositoryResult<T> = Result<T, RepositoryError>;
impl From<RepositoryError> for CommonError {
    fn from(error: RepositoryError) -> Self {
        CommonError {
            message: error.message,
            code: 1,
        }
    }
}
impl From<sqlx::Error> for RepositoryError {
    fn from(error: sqlx::Error) -> Self {
        RepositoryError {
            message: format!("Database error: {}", error),
        }
    }
}
impl From<argon2::password_hash::Error> for RepositoryError {
    fn from(error: argon2::password_hash::Error) -> Self {
        RepositoryError {
            message: format!("Database error: {}", error),
        }
    }
}
