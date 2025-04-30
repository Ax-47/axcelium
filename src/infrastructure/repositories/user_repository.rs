use crate::{
    domain::{
        errors::repositories_errors::{RepositoryError, RepositoryResult},
        models::{
            app_config::AppConfig,
            apporg_client_id_models::CleanAppOrgByClientId,
            user_models::{CreateUser, User, UserOrganization},
        },
    },
    infrastructure::database::user_repository::UserDatabaseRepository,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, PasswordHash,
};
use async_trait::async_trait;
use redis::Client as RedisClient;
use std::sync::Arc;
use std::time::Instant;
use uuid::Uuid;
pub struct UserRepositoryImpl {
    pub cache: Arc<RedisClient>,
    pub database_repo: Arc<dyn UserDatabaseRepository>,
}

impl UserRepositoryImpl {
    pub fn new(cache: Arc<RedisClient>, database_repo: Arc<dyn UserDatabaseRepository>) -> Self {
        Self {
            cache,
            database_repo,
        }
    }
}
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(
        &self,
        c_apporg: CleanAppOrgByClientId,
        user: CreateUser,
    ) -> RepositoryResult<Uuid>;

    async fn create_user(&self, user: User, u_org: UserOrganization) -> RepositoryResult<()>;
    fn check_rule_name(&self, rule_name: String) -> RepositoryResult<()>;
    async fn check_rule_email_can_be_nullable(
        &self,
        app_config: AppConfig,
        user: CreateUser,
        c_apporg: CleanAppOrgByClientId,
    ) -> RepositoryResult<()>;

    async fn check_rule_is_must_username_unique(
        &self,
        app_config: AppConfig,
        user: CreateUser,
        c_apporg: CleanAppOrgByClientId,
    ) -> RepositoryResult<()>;
    fn hash_password(&self, password: String) -> RepositoryResult<String>;
    fn verify_password(&self, stored_hash: String, password: String) -> RepositoryResult<bool>;
    async fn send_otp(&self);
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(
        &self,
        c_apporg: CleanAppOrgByClientId,
        user: CreateUser,
    ) -> RepositoryResult<Uuid> {
        let start_time = Instant::now();

        // ขั้นตอน 1: get_config
        let step1_start = Instant::now();
        let Ok(app_config) = c_apporg.get_config() else {
            return Err(RepositoryError::new(
                "failed to read config".to_string(),
                500,
            ));
        };
        let step1_duration = step1_start.elapsed();
        println!("get_config took: {:?}", step1_duration);

        // ขั้นตอน 2: check_rule_name
        let step2_start = Instant::now();
        self.check_rule_name(user.username.clone())?;
        let step2_duration = step2_start.elapsed();
        println!("check_rule_name took: {:?}", step2_duration);

        // ขั้นตอน 3: check_rule_email_can_be_nullable
        let step3_start = Instant::now();
        self.check_rule_email_can_be_nullable(app_config.clone(), user.clone(), c_apporg.clone())
            .await?;
        let step3_duration = step3_start.elapsed();
        println!(
            "check_rule_email_can_be_nullable took: {:?}",
            step3_duration
        );

        // ขั้นตอน 4: check_rule_is_must_username_unique
        let step4_start = Instant::now();
        self.check_rule_is_must_username_unique(app_config.clone(), user.clone(), c_apporg.clone())
            .await?;
        let step4_duration = step4_start.elapsed();
        println!(
            "check_rule_is_must_username_unique took: {:?}",
            step4_duration
        );

        // ขั้นตอน 5: hash_password
        let step5_start = Instant::now();
        let hashed_password = self.hash_password(user.password)?;
        let step5_duration = step5_start.elapsed();
        println!("hash_password took: {:?}", step5_duration);

        // ขั้นตอน 6: create User
        let step6_start = Instant::now();
        let new_user = User::new(c_apporg.clone(), user.username, hashed_password, user.email);
        let new_uorg = UserOrganization::new(c_apporg, new_user.clone());
        let user_id = new_user.user_id.clone();
        self.create_user(new_user, new_uorg).await?;
        let step6_duration = step6_start.elapsed();
        println!("create_user took: {:?}", step6_duration);

        // ขั้นตอนสุดท้าย: หยุดจับเวลาทั้งฟังก์ชัน
        let duration = start_time.elapsed();
        println!("create function took: {:?}", duration);

        Ok(user_id)
    }

    fn check_rule_name(&self, rule_name: String) -> RepositoryResult<()> {
        if rule_name.len() <= 2 || rule_name.len() >= 50 {
            return Err(RepositoryError::new(
                "username is not validate".to_string(),
                400,
            ));
        }
        Ok(())
    }
    async fn check_rule_is_must_username_unique(
        &self,
        app_config: AppConfig,
        user: CreateUser,
        c_apporg: CleanAppOrgByClientId,
    ) -> RepositoryResult<()> {
        if !app_config.is_must_name_unique {
            return Ok(());
        }
        let fetched_user = self
            .database_repo
            .find_user_by_username(
                user.username,
                c_apporg.application_id,
                c_apporg.organization_id,
            )
            .await?;
        if fetched_user.is_some() {
            return Err(RepositoryError::new(
                "this username has used".to_string(),
                400,
            ));
        }
        Ok(())
    }
    async fn check_rule_email_can_be_nullable(
        &self,
        app_config: AppConfig,
        user: CreateUser,
        c_apporg: CleanAppOrgByClientId,
    ) -> RepositoryResult<()> {
        if app_config.can_allow_email_nullable {
            return Ok(());
        }
        let Some(email) = user.email.as_ref() else {
            return Err(RepositoryError::new("email is required".to_string(), 399));
        };
        let fetched_user = self
            .database_repo
            .find_user_by_email(
                email.clone(),
                c_apporg.application_id,
                c_apporg.organization_id,
            )
            .await?;
        if fetched_user.is_some() {
            return Err(RepositoryError::new("this email has used".to_string(), 399));
        }
        Ok(())
    }

    async fn create_user(&self, user: User, u_org: UserOrganization) -> RepositoryResult<()> {
        let insert_tasks = vec![
            self.database_repo.insert_into_user(&user),
            self.database_repo.insert_into_user_by_email(&user),
            self.database_repo.insert_into_user_by_username(&user),
            self.database_repo.insert_into_user_organizations(&u_org),
            self.database_repo
                .insert_into_user_organizations_by_user(&u_org),
        ];
        futures::future::join_all(insert_tasks).await;
        Ok(())
    }
    fn hash_password(&self, password: String) -> RepositoryResult<String> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        Ok(password_hash)
    }

    fn verify_password(&self, stored_hash: String, password: String) -> RepositoryResult<bool> {
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&stored_hash)?;
        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
    async fn send_otp(&self) {}
}
