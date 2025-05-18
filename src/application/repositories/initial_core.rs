use crate::application::mappers::model::ModelMapper;
use crate::config;
use crate::domain::entities::application::Application;
use crate::domain::entities::apporg_client_id::AppOrgByClientId;
use crate::domain::entities::organization::Organization;
use crate::infrastructure::models::application::AppcalitionModel;
use crate::infrastructure::models::apporg_client_id::AppOrgModel;
use crate::infrastructure::models::organization::OrganizationModel;
use crate::infrastructure::repositories::{
    cache_layer::applications_organization_by_client_id_repository::ApplicationsOrganizationByClientIdCacheLayerRepository,
    cipher::aes_gcm_repository::AesGcmCipherRepository,
    cipher::base64_repository::Base64Repository,
    database::application_repository::ApplicationDatabaseRepository,
    database::organization_repository::OrganizationDatabaseRepository,
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
pub struct InitialCoreImpl {
    aes_repo: Arc<dyn AesGcmCipherRepository>,
    base64_repo: Arc<dyn Base64Repository>,
    org_db_repo: Arc<dyn OrganizationDatabaseRepository>,
    app_db_repo: Arc<dyn ApplicationDatabaseRepository>,
    apporg_by_client_id_cachelayer_repo:
        Arc<dyn ApplicationsOrganizationByClientIdCacheLayerRepository>,
}

impl InitialCoreImpl {
    pub fn new(
        aes_repo: Arc<dyn AesGcmCipherRepository>,
        base64_repo: Arc<dyn Base64Repository>,
        org_db_repo: Arc<dyn OrganizationDatabaseRepository>,
        app_db_repo: Arc<dyn ApplicationDatabaseRepository>,
        apporg_by_client_id_cachelayer_repo: Arc<
            dyn ApplicationsOrganizationByClientIdCacheLayerRepository,
        >,
    ) -> Self {
        Self {
            aes_repo,
            base64_repo,
            org_db_repo,
            app_db_repo,
            apporg_by_client_id_cachelayer_repo,
        }
    }
}
#[async_trait]
pub trait InitialCoreRepository: Send + Sync {
    fn new_org(&self, cfg: config::OrganizationConfig) -> Organization;
    async fn is_org_exist(&self, org_name: String) -> bool;
    async fn create_org(&self, org: Organization);
    async fn new_app(
        &self,
        organization_id: Uuid,
        app: config::ApplicationConfig,
    ) -> (Application, String, String);
    async fn create_app(&self, app: Application);
    fn new_apporg_by_client_id(&self, app: Application, org: Organization) -> AppOrgByClientId;
    async fn create_apporg_by_client_id(&self, apporg: AppOrgByClientId);

    fn create_client_token(
        &self,
        client_id: Uuid,
        client_key: String,
        client_secret: String,
    ) -> String;
}

#[async_trait]
impl InitialCoreRepository for InitialCoreImpl {
    fn create_client_token(
        &self,
        client_id: Uuid,
        client_key: String,
        client_secret: String,
    ) -> String {
        let encoded_id = self.base64_repo.encode(client_id.to_string().as_bytes());
        format!("{}.{}.{}", encoded_id, client_key, client_secret)
    }
    fn new_org(&self, cfg: config::OrganizationConfig) -> Organization {
        Organization::new(cfg.name, cfg.slug, cfg.contact_email)
    }
    async fn is_org_exist(&self, org_name: String) -> bool {
        self.org_db_repo
            .find_organization(org_name)
            .await
            .unwrap()
            .is_some()
    }
    async fn create_org(&self, org: Organization) {
        self.org_db_repo
            .create_organization(OrganizationModel::from_entity(org))
            .await
            .unwrap()
    }

    async fn new_app(
        &self,
        organization_id: Uuid,
        app: config::ApplicationConfig,
    ) -> (Application, String, String) {
        let client_secret = Application::gen_client_secret().unwrap();
        let (nonce, encrypted_client_secret) = self.aes_repo.encrypt(&client_secret).await.unwrap();
        let base64_secret = self.base64_repo.encode(&client_secret);
        (
            Application::new(
                organization_id,
                app.name,
                app.description,
                encrypted_client_secret,
                &app.config,
            ),
            nonce,
            base64_secret,
        )
    }
    async fn create_app(&self, app: Application) {
        self.app_db_repo
            .create_application(AppcalitionModel::from_entity(app))
            .await
            .unwrap()
    }
    fn new_apporg_by_client_id(&self, app: Application, org: Organization) -> AppOrgByClientId {
        AppOrgByClientId::new(org, app)
    }
    async fn create_apporg_by_client_id(&self, apporg: AppOrgByClientId) {
        self.apporg_by_client_id_cachelayer_repo
            .create_apporg_by_client_id(AppOrgModel::from_entity(apporg))
            .await
            .unwrap();
    }
}
