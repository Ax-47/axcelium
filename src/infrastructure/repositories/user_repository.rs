use crate::{
    domain::{
        entities::{user::User, user_organization::UserOrganization},
        errors::repositories_errors::RepositoryResult,
        models::{
            apporg_client_id_models::CleanAppOrgByClientId,
            user_models::{CreateUser, CreatedUser},
        },
    },
    infrastructure::repositories::{
        database::user_repository::UserDatabaseRepository,
        security::argon2_repository::PasswordHasherRepository,
    },
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
pub struct UserRepositoryImpl {
    database_repo: Arc<dyn UserDatabaseRepository>,
    hasher_repo: Arc<dyn PasswordHasherRepository>,
}

impl UserRepositoryImpl {
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
#[async_trait]
pub trait UserRepository: Send + Sync {
    fn new_user_organization(
        &self,
        c_apporg: CleanAppOrgByClientId,
        user: User,
    ) -> UserOrganization;
    fn new_user(
        &self,
        apporg: CleanAppOrgByClientId,
        user: CreateUser,
        hashed_password: String,
    ) -> User;
    fn hash_password(&self, password: String) -> RepositoryResult<String>;
    async fn find_user_by_username(
        &self,
        username: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<Option<CreatedUser>>;

    async fn find_user_by_email(
        &self,
        email: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<Option<CreatedUser>>;

    async fn create_user(&self, user: User, u_org: UserOrganization) -> RepositoryResult<()>;
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create_user(&self, user: User, u_org: UserOrganization) -> RepositoryResult<()> {
        self.database_repo.create_user(user, u_org).await
    }
    fn new_user(
        &self,
        c_apporg: CleanAppOrgByClientId,
        user: CreateUser,
        hashed_password: String,
    ) -> User {
        User::new(c_apporg.application_id,c_apporg.organization_id, user.username, hashed_password, user.email)
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
    ) -> RepositoryResult<Option<CreatedUser>> {
        self.database_repo
            .find_user_by_username(username, application_id, organization_id)
            .await
    }

    async fn find_user_by_email(
        &self,
        email: String,
        application_id: Uuid,
        organization_id: Uuid,
    ) -> RepositoryResult<Option<CreatedUser>> {
        self.database_repo
            .find_user_by_email(email, application_id, organization_id)
            .await
    }
}
