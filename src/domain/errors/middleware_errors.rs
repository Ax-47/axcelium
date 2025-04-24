use core::fmt;
use std::string::FromUtf8Error;
use base64::DecodeError;
use std::error::Error;
use serde::Serialize;
use scylla::errors::FirstRowError;


#[derive(Debug, Serialize)]
pub struct MiddelwareError {
    pub message: String,
    pub code: i16,
}
impl fmt::Display for MiddelwareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (code: {})", self.message, self.code)
    }
}

impl Error for MiddelwareError {}
pub type MiddelwareResult<T> = Result<T, MiddelwareError>;
impl From<DecodeError> for MiddelwareError {
     fn from(error: DecodeError) -> Self {
        MiddelwareError {
            message: format!("failed to Decode: {}", error),
            code: 400,
        }
    }
}

impl From<FromUtf8Error> for MiddelwareError {
     fn from(error: FromUtf8Error) -> Self {
        MiddelwareError {
            message: format!("failed to Decode: {}", error),
            code: 400,
        }
    }
}

impl From<FirstRowError> for MiddelwareError {
     fn from(error: FirstRowError) -> Self {
        MiddelwareError {
            message: format!("failed to Query: {}", error),
            code: 400,
        }
    }
}
impl From<argon2::password_hash::Error> for MiddelwareError {
    fn from(error: argon2::password_hash::Error) -> Self {
        MiddelwareError {
            message: format!("failed to hash: {}", error),
            code: 500,
        }
    }
}
impl actix_web::ResponseError for MiddelwareError {
    fn error_response(&self) -> actix_web::HttpResponse {
        let body = MiddelwareError {
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