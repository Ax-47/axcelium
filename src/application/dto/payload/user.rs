use email_address::EmailAddress;
use serde::Deserialize;
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Deserialize)]
pub struct CreateUserPayload {
    #[validate(custom(function = "validate_username_or_email"))]
    pub username: String,

    #[validate(email)]
    pub email: Option<String>,

    #[validate(length(min = 8))]
    pub password: String,
}

fn validate_username_or_email(value: &str) -> Result<(), ValidationError> {
    let is_valid_username = value.len() >= 3 && value.len() <= 50;
    let is_valid_email = EmailAddress::is_valid(value);
    if is_valid_username || is_valid_email {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_username_or_email"))
    }
}
#[derive(Deserialize)]
pub struct PaginationQuery {
    pub page_size: Option<i32>,
    pub paging_state: Option<String>,
}

#[derive(Deserialize)]
pub struct GetUserQuery {
    pub user_id: Uuid,
}