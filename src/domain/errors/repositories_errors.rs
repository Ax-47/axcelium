use scylla::errors::{ExecutionError, FirstRowError, IntoRowsResultError, MaybeFirstRowError};
use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct CommonError {
    pub message: String,
    pub code: i16,
}

impl std::fmt::Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}, Code: {}", self.message, self.code)
    }
}

#[derive(Debug)]
pub struct RepositoryError {
    pub message: String,
    pub code: i16,
}
impl RepositoryError {
    pub fn new(message: String, code: i16) -> Self {
        Self { message, code }
    }
}
#[derive(Debug)]
pub struct ApiError(CommonError);
impl ApiError {
    pub fn new(message: String, code: i16) -> Self {
        Self(CommonError { message, code })
    }
}

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
            code: error.code,
        }
    }
}

impl From<RepositoryError> for ApiError {
    fn from(error: RepositoryError) -> Self {
        ApiError(CommonError {
            message: error.message,
            code: error.code,
        })
    }
}
impl From<argon2::password_hash::Error> for RepositoryError {
    fn from(error: argon2::password_hash::Error) -> Self {
        RepositoryError {
            message: format!("failed to hash: {}", error),
            code: 500,
        }
    }
}

impl From<FirstRowError> for RepositoryError {
    fn from(error: FirstRowError) -> Self {
        RepositoryError {
            message: format!("failed to Query: {}", error),
            code: 500,
        }
    }
}

impl From<MaybeFirstRowError> for RepositoryError {
    fn from(error: MaybeFirstRowError) -> Self {
        RepositoryError {
            message: format!("failed to Query: {}", error),
            code: 500,
        }
    }
}
impl From<ExecutionError> for RepositoryError {
    fn from(error: ExecutionError) -> Self {
        RepositoryError {
            message: format!("DB query faile: {}", error),
            code: 500,
        }
    }
}

impl From<IntoRowsResultError> for RepositoryError {
    fn from(error: IntoRowsResultError) -> Self {
        RepositoryError {
            message: format!("DB query faile: {}", error),
            code: 500,
        }
    }
}
