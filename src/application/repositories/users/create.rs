use crate::application::mappers::model::ModelMapper;
use crate::{
    application::{
        dto::{payload::user::CreateUserPayload, response::user::CreateUserResponse},
        mappers::{application::ApplicationMapper, user::found::FoundUserMapper},
    },
    domain::{
        entities::{
            apporg_client_id::CleanAppOrgByClientId, user::User,
            user_organization::UserOrganization,
        },
        errors::repositories_errors::RepositoryResult,
    },
    infrastructure::{
        models::{user::UserModel, user_organization::UserOrganizationModel},
        repositories::{
            database::user_repository::UserDatabaseRepository,
            security::argon2_repository::PasswordHasherRepository,
        },
    },
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
pub struct CreateUserRepositoryImpl {
    database_repo: Arc<dyn UserDatabaseRepository>,
    hasher_repo: Arc<dyn PasswordHasherRepository>,
}

impl CreateUserRepositoryImpl {
    pub fn new(
        database_repo: Arc<dyn UserDatabaseRepository>,
        hasher_repo: Arc<dyn PasswordHasherRepository>,
    ) -> Self {
        Self {
            database_repo,
            hasher_repo,
        }
    }
}
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait CreateUserRepository: Send + Sync {
    fn new_user_organization(
        &self,
        c_apporg: CleanAppOrgByClientId,
        user: User,
    ) -> UserOrganization;
    fn new_user(
        &self,
        apporg: CleanAppOrgByClientId,
        user: CreateUserPayload,
        hashed_password: String,
    ) -> User;
    fn hash_password(&self, password: String) -> RepositoryResult<String>;
    async fn find_user_by_username(
        &self,
        username: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<Option<CreateUserResponse>>;

    async fn find_user_by_email(
        &self,
        email: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<Option<CreateUserResponse>>;

    async fn create_user(&self, user: User, u_org: UserOrganization) -> RepositoryResult<()>;
}

#[async_trait]
impl CreateUserRepository for CreateUserRepositoryImpl {
    async fn create_user(&self, user: User, u_org: UserOrganization) -> RepositoryResult<()> {
        self.database_repo
            .create_user(
                UserModel::from_entity(user),
                UserOrganizationModel::from_entity(u_org),
            )
            .await
    }
    fn new_user(
        &self,
        c_apporg: CleanAppOrgByClientId,
        user: CreateUserPayload,
        hashed_password: String,
    ) -> User {
        User::new(
            c_apporg.application_id,
            c_apporg.organization_id,
            user.username,
            hashed_password,
            user.email,
        )
    }

    fn new_user_organization(
        &self,
        c_apporg: CleanAppOrgByClientId,
        user: User,
    ) -> UserOrganization {
        UserOrganization::new(c_apporg, user)
    }
    fn hash_password(&self, password: String) -> RepositoryResult<String> {
        self.hasher_repo.hash(password.as_str())
    }
    async fn find_user_by_username(
        &self,
        username: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<Option<CreateUserResponse>> {
        let Some(user) = self
            .database_repo
            .find_user_by_username(username, application_id, organization_id)
            .await?
        else {
            return Ok(None);
        };

        Ok(Some(FoundUserMapper::to_dto(user)))
    }

    async fn find_user_by_email(
        &self,
        email: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<Option<CreateUserResponse>> {
        let Some(user) = self
            .database_repo
            .find_user_by_email(email, application_id, organization_id)
            .await?
        else {
            return Ok(None);
        };
        Ok(Some(FoundUserMapper::to_dto(user)))
    }
}
