use scylla::errors::{ExecutionError, FirstRowError, IntoRowsResultError, MaybeFirstRowError};
use serde::Serialize;
use std::string::FromUtf8Error;
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
        actix_web::HttpResponse::build(self.status_code()).json(self.0.message.clone())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self.0.code {
            401 => actix_web::http::StatusCode::UNAUTHORIZED,
            403 => actix_web::http::StatusCode::FORBIDDEN,
            404 => actix_web::http::StatusCode::NOT_FOUND,
            500 => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            _ => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
impl actix_web::ResponseError for RepositoryError {
    fn error_response(&self) -> actix_web::HttpResponse {
        let body = CommonError {
            message: self.message.clone(),
            code: self.code,
        };
        actix_web::HttpResponse::build(self.status_code()).json(body)
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self.code {
            401 => actix_web::http::StatusCode::UNAUTHORIZED,
            403 => actix_web::http::StatusCode::FORBIDDEN,
            404 => actix_web::http::StatusCode::NOT_FOUND,
            500 => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            _ => actix_web::http::StatusCode::BAD_REQUEST,
        }
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
impl From<uuid::Error> for RepositoryError {
    fn from(_: uuid::Error) -> Self {
        RepositoryError {
            message: "invalid UUID for client_id".to_string(),
            code: 400,
        }
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

impl From<FromUtf8Error> for RepositoryError {
    fn from(_error: FromUtf8Error) -> Self {
        RepositoryError {
            message: format!("failed to convert"),
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

impl From<aes_gcm::Error> for RepositoryError {
    fn from(_: aes_gcm::Error) -> Self {
        RepositoryError {
            message: format!("Cipher failed."),
            code: 500,
        }
    }
}

impl From<base64::DecodeError> for RepositoryError {
    fn from(_: base64::DecodeError) -> Self {
        RepositoryError {
            message: format!("Cipher failed."),
            code: 500,
        }
    }
}
