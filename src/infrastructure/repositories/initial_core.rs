use crate::domain::models::app_config::AppConfig;
use crate::domain::models::application_models::Application;
use crate::domain::models::apporg_client_id_models::AppOrgByClientId;
use crate::domain::models::organization_models::Organization;
use crate::infrastructure::cache_layer::applications_organization_by_client_id_repository::ApplicationsOrganizationByClientIdCacheLayerRepository;
use crate::infrastructure::cipher::aes_gcm_repository::AesGcmCipherRepository;
use crate::infrastructure::cipher::base64_repository::Base64Repository;
use crate::infrastructure::database::application_repository::ApplicationDatabaseRepository;
use crate::infrastructure::database::organization_repository::OrganizationDatabaseRepository;
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

#[async_trait]
pub trait InitialCoreRepository: Send + Sync {
    fn create_org_obj(&self, name: String, slug: String, email: String) -> Organization;
    async fn is_org_exist(&self, org_name:String) -> bool;
    async fn create_org(&self,org: Organization);
    async fn create_app_obj(&self,organization_id: Uuid, name:String, description:String,config:AppConfig)-> (Application, String, String);
    async fn create_app(&self,app:Application);
    async fn create_apporg_by_client_id_obj(&self,app:Application,org:Organization)->AppOrgByClientId;
    async fn create_apporg_by_client_id(&self,apporg:AppOrgByClientId);
}

#[async_trait]
impl InitialCoreRepository for InitialCoreImpl {


        // println!("CORE_ORGANIZATION_ID= {}", org_app.organization_id);
        // println!("CORE_APPLICATION_ID= {}", org_app.application_id);
        // println!("CORE_CLIENT_ID= {}", org_app.client_id);
        // println!("CORE_CLIENT_SECRET= {}", client_secret);
        // println!("CORE_CLIENT_KEY= {}", key);
        // println!(
        //     "CORE_CLIENT_TOKEN= axcelium-core: {}",
        //     self.create_client_token(org_app.client_id, key, client_secret)
        // );

    // let name = Self::get_env("CORE_ORGANIZATION_NAME", "Axcelium");
    // let slug = Self::get_env("CORE_ORGANIZATION_SLUG", "axcelium");
    // let email = Self::get_env("CORE_ORGANIZATION_CONTACT_EMAIL", "support@axcelium.io");
    fn create_org_obj(&self, name: String, slug: String, email: String) -> Organization {
        Organization::new(name, slug, email)
    }
    async fn is_org_exist(&self, org_name:String) -> bool {
        self.org_db_repo
            .find_organization(org_name)
            .await
            .unwrap()
            .is_some()
    }
    async fn create_org(&self,org: Organization){

        self.org_db_repo
            .create_organization(org)
            .await
            .unwrap()
    }

        // let name = Self::get_env("CORE_APPLICATION_NAME", "Axcelium Core");
        // let description = Self::get_env(
        //     "CORE_APPLICATION_DESCRIPTION",
        //     "The core SSO platform of Axcelium.",
        // );
        // let is_must_name_unique = Self::get_env_bool("CORE_APPLICATION_CONFIG_IS_MUST_NAME_UNIQUE");
        // let can_allow_email_nullable =
        //     Self::get_env_bool("CORE_APPLICATION_CONFIG_CAN_ALLOW_EMAIL_NULLABLE");
    async fn create_app_obj(&self,organization_id: Uuid, name:String, description:String,config:AppConfig)-> (Application, String, String){

        let client_secret = Application::gen_client_secret().unwrap();
        let (nonce, encrypted_client_secret) = self.aes_repo.encrypt(&client_secret).await.unwrap();
        let base64_secret = self.base64_repo.encode(&client_secret);
        (
            Application::new(
                organization_id,
                name,
                description,
                encrypted_client_secret,
                &config,
            ),
            base64_secret,
            nonce,
        )
    }
    async fn create_app(&self,app:Application){
        self.app_db_repo
            .create_application(app)
            .await
            .unwrap()
    }
    async fn create_apporg_by_client_id_obj(&self,app:Application,org:Organization)->AppOrgByClientId{
         AppOrgByClientId::new(org, app)
    }
    async fn create_apporg_by_client_id(&self,apporg:AppOrgByClientId){
        self.apporg_by_client_id_cachelayer_repo
            .create_apporg_by_client_id(apporg)
            .await
            .unwrap();
    }
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
    // vvv config
    // fn get_env(key: &str, default: &str) -> String {
    //     env::var(key).unwrap_or_else(|_| default.to_string())
    // }

    // fn get_env_bool(key: &str) -> bool {
    //     env::var(key).map(|v| v == "true").unwrap_or(false)
    // }

    pub fn create_client_token(
        &self,
        client_id: Uuid,
        client_key: String,
        client_secret: String,
    ) -> String {
        let encoded_id = self.base64_repo.encode(client_id.to_string().as_bytes());
        format!("{}.{}.{}", encoded_id, client_key, client_secret)
    } //bl
}
