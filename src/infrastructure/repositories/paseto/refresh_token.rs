use crate::domain::entities::refresh_token::RefreshToken;
use crate::domain::errors::repositories_errors::RepositoryError;
use crate::domain::errors::repositories_errors::RepositoryResult;
use async_trait::async_trait;
use rusty_paseto::core::*;
use rusty_paseto::prelude::*;

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
        private_key: &Vec<u8>,
        rt: RefreshToken,
        secret: &str,
        secret_key: &str,
        issued_at: String,
        expire: String,
        notbefore: String,
    ) -> RepositoryResult<String>;
    async fn decrypt(&self, key: String, token: &str) -> RepositoryResult<String>;
}

#[async_trait]
impl PasetoRepository for PasetoRepositoryImpl {
    async fn encrypt(
        &self,
        private_key: &Vec<u8>,
        rt: RefreshToken,
        secret: &str,
        secret_key: &str,
        issued_at: String,
        expire: String,
        notbefore: String,
    ) -> RepositoryResult<String> {
        let private_key = Key::<64>::try_from(private_key.as_slice())
            .map_err(|e| RepositoryError::new(format!("invalid hex key: {e}"),500))?;
        let pk: &[u8] = private_key.as_slice();
        let private_key = PasetoAsymmetricPrivateKey::<V4, Public>::from(pk);
        let token_id = rt.token_id.clone().to_string();
        let token = PasetoBuilder::<V4, Public>::default()
            .set_claim(TokenIdentifierClaim::from(token_id.as_str()))
            .set_claim(CustomClaim::try_from(("secret", secret))?)
            .set_claim(CustomClaim::try_from(("secret_key", secret_key))?)
            .set_claim(CustomClaim::try_from(("version", rt.token_version))?)
            .set_claim(IssuedAtClaim::try_from(issued_at)?)
            .set_claim(ExpirationClaim::try_from(expire)?)
            .set_claim(NotBeforeClaim::try_from(notbefore)?)
            .build(&private_key)?;

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
