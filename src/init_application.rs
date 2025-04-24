use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::Argon2;
use argon2::PasswordHasher;
use scylla::client::session::Session;
use scylla::errors::FirstRowError;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::models::application_models::Application;
use crate::domain::models::apporg_client_id_models::AppOrgByClientId;
use crate::domain::models::organization_models::Organization;
pub struct InitialCore {
    session: Arc<Session>,
}
impl InitialCore {
    pub fn new(session: Arc<Session>) -> Self {
        Self { session }
    }
    pub async fn init_core(&self) {
        let name = "Axcelium".to_string();
        let slug = "axcelium".to_string();
        let contact_email = "support@axcelium.io".to_string();
        let result = self.find_core(name.clone()).await;
        if result.is_ok() {
            return;
        }
        let org = Organization::new(name,slug,contact_email);
        self.create_organization(org.clone()).await;
        let client_secret = Uuid::new_v4();
        let hashed_client_secret = Self::hash_password(client_secret.to_string());
        let app_name = "Axcelium Core";
        let app_description = "The core SSO platform of Axcelium.";
        let app = Application::new(
            org.organization_id,
            app_name.to_string(),
            app_description.to_string(),
            hashed_client_secret,
        );
        self.create_application(app.clone()).await;
        let org_app = AppOrgByClientId::new(org, app);
        self.create_applications_organization_by_client_id(org_app.clone())
            .await;
        println!("ORGANIZATION_ID={}", org_app.organization_id);
        println!("APPLICATION_ID={}", org_app.application_id);
        println!("CLIENT_ID={}", org_app.client_id);
        println!("CLIENT_SECRET={}", client_secret);
    }
    fn hash_password(password: String) -> String {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();
        password_hash
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
