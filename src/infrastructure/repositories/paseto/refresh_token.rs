use crate::domain::errors::repositories_errors::RepositoryError;
use crate::domain::errors::repositories_errors::RepositoryResult;
use async_trait::async_trait;
use rusty_paseto::core::*;
use rusty_paseto::prelude::*;
use time;
use time::format_description::well_known::Rfc3339;

pub struct PasetoRepositoryImpl {}

impl PasetoRepositoryImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
pub trait PasetoRepository: Send + Sync {
    async fn encrypt(
        &self,
        key: String,
        token_secret: &str,
        version: &str,
    ) -> RepositoryResult<String>;
    async fn decrypt(&self, key: String, token: &str) -> RepositoryResult<String>;
}

#[async_trait]
impl PasetoRepository for PasetoRepositoryImpl {
    async fn encrypt(
        &self,
        key: String,
        token_secret: &str,
        version: &str,
    ) -> RepositoryResult<String> {
        if key.len() != 32 {
            return Err(RepositoryError::new(
                "Paseto key must be 32 bytes".into(),
                500,
            ));
        }

        let paseto_key = PasetoSymmetricKey::<V4, Local>::from(Key::from(key.as_bytes()));
        let in_30_days =
            (time::OffsetDateTime::now_utc() + time::Duration::days(30)).format(&Rfc3339)?;

        let in_40_mins =
            (time::OffsetDateTime::now_utc() + time::Duration::minutes(40)).format(&Rfc3339)?;
        let token = PasetoBuilder::<V4, Local>::default()
            .set_claim(TokenIdentifierClaim::from(token_secret))
            .set_claim(CustomClaim::try_from(("version", version))?)
            .set_claim(ExpirationClaim::try_from(in_30_days)?)
            .set_claim(NotBeforeClaim::try_from(in_40_mins)?)
            .build(&paseto_key)?;

        Ok(token)
    }

    async fn decrypt(&self, key: String, token: &str) -> RepositoryResult<String> {
        if key.len() != 32 {
            return Err(RepositoryError::new(
                "Paseto key must be 32 bytes".into(),
                500,
            ));
        }
        let paseto_key = PasetoSymmetricKey::<V4, Local>::from(Key::from(key.as_bytes()));

        let payload = PasetoParser::<V4, Local>::default().parse(token, &paseto_key)?;
        Ok(payload.to_string())
    }
}
