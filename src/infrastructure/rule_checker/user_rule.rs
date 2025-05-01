use crate::{
    domain::{
        errors::repositories_errors::{RepositoryError, RepositoryResult},
        models::{
            app_config::AppConfig, apporg_client_id_models::CleanAppOrgByClientId,
            user_models::CreateUser,
        },
    },
    infrastructure::database::user_repository::UserDatabaseRepository,
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct UserRuleCheckerImpl {
    repo: Arc<dyn UserDatabaseRepository>,
}

impl UserRuleCheckerImpl {
    pub fn new(repo: Arc<dyn UserDatabaseRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
pub trait UserRuleCheckerRepository: Send + Sync {
    fn check_rule_name(&self, name: &str) -> RepositoryResult<()>;

    async fn check_email_nullable(
        &self,
        config: &AppConfig,
        user: &CreateUser,
        c_apporg: &CleanAppOrgByClientId,
    ) -> RepositoryResult<()>;

    async fn check_username_unique(
        &self,
        config: &AppConfig,
        user: &CreateUser,
        c_apporg: &CleanAppOrgByClientId,
    ) -> RepositoryResult<()>;
}
#[async_trait]
impl UserRuleCheckerRepository for UserRuleCheckerImpl {
    fn check_rule_name(&self, name: &str) -> RepositoryResult<()> {
        if name.len() <= 2 || name.len() >= 50 {
            Err(RepositoryError::new("username is not valid".into(), 400))
        } else {
            Ok(())
        }
    }

    async fn check_email_nullable(
        &self,
        config: &AppConfig,
        user: &CreateUser,
        c_apporg: &CleanAppOrgByClientId,
    ) -> RepositoryResult<()> {
        if config.can_allow_email_nullable {
            return Ok(());
        }

        let Some(email) = user.email.as_ref() else {
            return Err(RepositoryError::new("email is required".to_string(), 400));
        };

        let found = self
            .repo
            .find_user_by_email(
                email.clone(),
                c_apporg.application_id,
                c_apporg.organization_id,
            )
            .await?;

        if found.is_some() {
            Err(RepositoryError::new("email already used".into(), 400))
        } else {
            Ok(())
        }
    }

    async fn check_username_unique(
        &self,
        config: &AppConfig,
        user: &CreateUser,
        c_apporg: &CleanAppOrgByClientId,
    ) -> RepositoryResult<()> {
        if !config.is_must_name_unique {
            return Ok(());
        }

        let found = self
            .repo
            .find_user_by_username(
                user.username.clone(),
                c_apporg.application_id,
                c_apporg.organization_id,
            )
            .await?;

        if found.is_some() {
            Err(RepositoryError::new("username already used".into(), 400))
        } else {
            Ok(())
        }
    }
}
