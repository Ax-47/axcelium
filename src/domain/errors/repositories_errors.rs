use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use redis::RedisError;
use rusty_paseto::prelude::{GenericBuilderError, GenericParserError, PasetoClaimError};
use scylla::errors::{
    DeserializationError, ExecutionError, FirstRowError, IntoRowsResultError, MaybeFirstRowError,
    PrepareError, RowsError,
};
use serde::Serialize;
use std::{fmt, string::FromUtf8Error};
#[derive(Debug, Serialize)]
pub struct CommonError {
    pub message: String,
    pub code: i16,
}

impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}, Code: {}", self.message, self.code)
    }
}

impl CommonError {
    fn to_status(&self) -> StatusCode {
        match self.code {
            401 => StatusCode::UNAUTHORIZED,
            403 => StatusCode::FORBIDDEN,
            404 => StatusCode::NOT_FOUND,
            500 => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        }
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

    fn to_common(&self) -> CommonError {
        CommonError {
            message: self.message.clone(),
            code: self.code,
        }
    }
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}, Code: {}", self.message, self.code)
    }
}

impl ResponseError for RepositoryError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.to_common())
    }

    fn status_code(&self) -> StatusCode {
        self.to_common().to_status()
    }
}

#[derive(Debug, Serialize)]
pub struct ApiError(CommonError);

impl ApiError {
    pub fn new(message: String, code: i16) -> Self {
        Self(CommonError { message, code })
    }
}

impl From<CommonError> for ApiError {
    fn from(error: CommonError) -> Self {
        Self(error)
    }
}

impl From<RepositoryError> for ApiError {
    fn from(err: RepositoryError) -> Self {
        Self(err.to_common())
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self)
    }

    fn status_code(&self) -> StatusCode {
        self.0.to_status()
    }
}

pub type RepositoryResult<T> = Result<T, RepositoryError>;

// === Error Mappings ===

impl From<uuid::Error> for RepositoryError {
    fn from(e: uuid::Error) -> Self {
        println!("{}", e);
        RepositoryError::new("invalid UUID for client_id".to_string(), 400)
    }
}


impl From<std::str::Utf8Error> for RepositoryError {
    fn from(e: std::str::Utf8Error) -> Self {
        RepositoryError::new(format!("key error: {e:#?}"), 400)
    }
}
impl From<std::convert::Infallible> for RepositoryError {
    fn from(e: std::convert::Infallible) -> Self {
        RepositoryError::new(format!("key error: {e:#?}"), 400)
    }
}
impl From<rusty_paseto::core::Key<32>> for RepositoryError {
    fn from(e: rusty_paseto::core::Key<32>) -> Self {
        RepositoryError::new(format!("key error: {e:#?}"), 400)
    }
}

impl From<rusty_paseto::core::Key<64>> for RepositoryError {
    fn from(e: rusty_paseto::core::Key<64>) -> Self {
        RepositoryError::new(format!("key error: {e:#?}"), 400)
    }
}
impl From<GenericParserError> for RepositoryError {
    fn from(e: GenericParserError) -> Self {
        RepositoryError::new(format!("parser error: {}", e), 400)
    }
}
impl From<GenericBuilderError> for RepositoryError {
    fn from(e: GenericBuilderError) -> Self {
        RepositoryError::new(format!("builder error: {}", e), 400)
    }
}
impl From<PasetoClaimError> for RepositoryError {
    fn from(e: PasetoClaimError) -> Self {
        RepositoryError::new(format!("time error: {}", e), 400)
    }
}

impl From<time::error::Format> for RepositoryError {
    fn from(e: time::error::Format) -> Self {
        RepositoryError::new(format!("time format error: {}", e), 500)
    }
}

impl From<rand_core::OsError> for RepositoryError {
    fn from(e: rand_core::OsError) -> Self {
        println!("{}", e);
        RepositoryError::new("random error".to_string(), 400)
    }
}
impl From<argon2::password_hash::Error> for RepositoryError {
    fn from(err: argon2::password_hash::Error) -> Self {
        RepositoryError::new(format!("failed to hash: {}", err), 500)
    }
}

impl From<FromUtf8Error> for RepositoryError {
    fn from(e: FromUtf8Error) -> Self {
        println!("{}", e);
        RepositoryError::new("failed to convert".to_string(), 500)
    }
}
impl From<FirstRowError> for RepositoryError {
    fn from(err: FirstRowError) -> Self {
        RepositoryError::new(format!("failed to Query: {}", err), 500)
    }
}
impl From<RowsError> for RepositoryError {
    fn from(err: RowsError) -> Self {
        RepositoryError::new(format!("failed to Query: {}", err), 500)
    }
}
impl From<DeserializationError> for RepositoryError {
    fn from(err: DeserializationError) -> Self {
        RepositoryError::new(format!("failed to Query: {}", err), 500)
    }
}
impl From<PrepareError> for RepositoryError {
    fn from(err: PrepareError) -> Self {
        RepositoryError::new(format!("failed to Query: {}", err), 500)
    }
}

impl From<MaybeFirstRowError> for RepositoryError {
    fn from(err: MaybeFirstRowError) -> Self {
        RepositoryError::new(format!("failed to Query: {}", err), 500)
    }
}

impl From<ExecutionError> for RepositoryError {
    fn from(err: ExecutionError) -> Self {
        RepositoryError::new(format!("DB query failed: {}", err), 500)
    }
}

impl From<IntoRowsResultError> for RepositoryError {
    fn from(err: IntoRowsResultError) -> Self {
        RepositoryError::new(format!("DB query failed: {}", err), 500)
    }
}

impl From<aes_gcm::Error> for RepositoryError {
    fn from(_: aes_gcm::Error) -> Self {
        RepositoryError::new("Cipher failed.".to_string(), 500)
    }
}

impl From<base64::DecodeError> for RepositoryError {
    fn from(_: base64::DecodeError) -> Self {
        RepositoryError::new("Cipher failed.".to_string(), 500)
    }
}
impl From<RedisError> for RepositoryError {
    fn from(e: RedisError) -> Self {
        println!("{e}");
        RepositoryError::new("caching error failed.".to_string(), 500)
    }
}
impl From<serde_json::Error> for RepositoryError {
    fn from(_: serde_json::Error) -> Self {
        RepositoryError::new("convert error".to_string(), 500)
    }
}
