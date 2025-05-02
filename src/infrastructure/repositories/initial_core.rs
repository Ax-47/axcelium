use crate::domain::models::app_config::AppConfig;
use crate::domain::models::application_models::Application;
use crate::domain::models::apporg_client_id_models::AppOrgByClientId;
use crate::domain::models::organization_models::Organization;
use crate::infrastructure::cipher::aes_gcm_repository::AesGcmCipherRepository;
use crate::infrastructure::cipher::base64_repository::Base64Repository;
use scylla::client::session::Session;
use scylla::errors::FirstRowError;
use std::env;
use std::sync::Arc;
use uuid::Uuid;
pub struct InitialCoreImpl {
    session: Arc<Session>,
    cipher_repo: Arc<dyn AesGcmCipherRepository>,
    base64_repo: Arc<dyn Base64Repository>,
}
impl InitialCoreImpl {
    pub fn new(
        session: Arc<Session>,
        cipher_repo: Arc<dyn AesGcmCipherRepository>,
        base64_repo: Arc<dyn Base64Repository>,
    ) -> Self {
        Self {
            session,
            cipher_repo,
            base64_repo,
        }
    }
    fn create_org(&self) -> Organization {
        let org_name = env::var("CORE_ORGANIZATION_NAME").unwrap_or("Axcelium".to_string());
        let org_slug = env::var("CORE_ORGANIZATION_SLUG").unwrap_or("axcelium".to_string());
        let org_contact_email = env::var("CORE_ORGANIZATION_CONTACT_EMAIL")
            .unwrap_or("support@axcelium.io".to_string());
        Organization::new(org_name, org_slug, org_contact_email)
    }
    async fn create_app(&self, core_org: &Organization) -> (Application, String, String) {
        let app_name = env::var("CORE_APPLICATION_NAME").unwrap_or("Axcelium Core".to_string());
        let app_description = env::var("CORE_APPLICATION_DESCRIPTION")
            .unwrap_or("The core SSO platform of Axcelium.".to_string());
        let is_name_unique = env::var("CORE_APPLICATION_CONFIG_IS_MUST_NAME_UNIQUE")
            .map(|v| v == "true")
            .unwrap_or(false);
        let can_allow_email_nullable = env::var("CORE_APPLICATION_CONFIG_CAN_ALLOW_EMAIL_NULLABLE")
            .map(|v| v == "true")
            .unwrap_or(false);
        let app_config = AppConfig::new(is_name_unique, can_allow_email_nullable);
        let client_secret = Application::gen_client_secret().unwrap();
        let sliced_client_secret = client_secret.as_slice();
        let (nonce, encrypted_client_secret) = self
            .cipher_repo
            .encrypt(sliced_client_secret)
            .await
            .unwrap();
        let base64tified_client_secret = self.base64_repo.encode(sliced_client_secret);
        (
            Application::new(
                core_org.organization_id,
                app_name,
                app_description,
                encrypted_client_secret,
                &app_config,
            ),
            base64tified_client_secret,
            nonce,
        )
    }
    pub async fn init_core(&self) {
        let org = self.create_org();
        let result = self.find_core(org.name.clone()).await;
        if result.is_ok() {
            return;
        }
        self.create_organization(org.clone()).await;
        let (app, client_secret, key) = self.create_app(&org).await;
        self.create_application(app.clone()).await;
        let org_app = AppOrgByClientId::new(org, app);
        self.create_applications_organization_by_client_id(org_app.clone())
            .await;
        println!("ORGANIZATION_ID= {}", org_app.organization_id);
        println!("APPLICATION_ID= {}", org_app.application_id);
        println!("CLIENT_ID= {}", org_app.client_id);
        println!("CLIENT_SECRET= {}", client_secret);
        println!("CLIENT_KEY= {}", key);
        println!(
            "CLIENT_TOKEN= axcelium-core: {}",
            self.create_client_token(org_app.client_id, key, client_secret)
        );
    }
    pub fn create_client_token(
        &self,
        client_id: Uuid,
        client_key: String,
        client_secret: String,
    ) -> String {
        let base64tified_client_id = self.base64_repo.encode(client_id.as_bytes());
        return format!(
            "{}.{}.{}",
            base64tified_client_id, client_key, client_secret
        );
    }
    async fn find_core(&self, name: String) -> Result<(Uuid,), FirstRowError> {
        let query = "
    SELECT organization_id FROM axcelium.organizations
    WHERE name = ? ALLOW FILTERING;";
        let res = self
            .session
            .query_unpaged(query, (name,))
            .await
            .unwrap()
            .into_rows_result()
            .unwrap()
            .first_row::<(Uuid,)>();
        res
    }
    // TODO: THE NEXT DAY; split it into repo 
    async fn create_organization(&self, org: Organization) {
        let query = "
        INSERT INTO axcelium.organizations (
            organization_id,
            name,
            slug,
            contact_email,
            is_active,
            created_at,
            updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?);
    ";
        self.session.query_unpaged(query, &org).await.unwrap();
    }

    async fn create_application(&self, app: Application) {
        let query = "
        INSERT INTO axcelium.applications (
            organization_id,
            application_id,
            name,
            description,
            client_id,
            client_secret,
            created_at,
            updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?);
    ";
        self.session.query_unpaged(query, &app).await.unwrap();
    }
    async fn create_applications_organization_by_client_id(&self, org_app: AppOrgByClientId) {
        let query = "
            INSERT INTO axcelium.applications_organization_by_client_id (
                client_id,
                application_id,
                organization_id,
                client_secret,
                organization_name,
                organization_slug,
                application_name,
                application_description,
                is_active,
                created_at,
                updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);
        ";

        self.session.query_unpaged(query, &org_app).await.unwrap();
    }
}
