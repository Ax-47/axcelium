use crate::{
    config, domain::models::apporg_client_id_models::AppOrgByClientId,
    infrastructure::repositories::initial_core::InitialCoreRepository,
};
use async_trait::async_trait;
use std::sync::Arc;
#[derive(Clone)]
pub struct InitialCoreServiceImpl {
    pub repository: Arc<dyn InitialCoreRepository>,
}
impl InitialCoreServiceImpl {
    pub fn new(repository: Arc<dyn InitialCoreRepository>) -> Self {
        Self { repository }
    }
    fn print_info(&self, apporg: AppOrgByClientId, key: String, client_secret: String) {
        println!("CORE_ORGANIZATION_ID= {}", apporg.organization_id);
        println!("CORE_APPLICATION_ID= {}", apporg.application_id);
        println!("CORE_CLIENT_ID= {}", apporg.client_id);
        println!("CORE_CLIENT_SECRET= {}", client_secret);
        println!("CORE_CLIENT_KEY= {}", key);
        println!(
            "CORE_CLIENT_TOKEN= axcelium-core: {}",
            self.repository
                .create_client_token(apporg.client_id, key, client_secret)
        );
    }
}
#[async_trait]
pub trait InitialCoreService: 'static + Sync + Send {
    async fn lunch(&self, cfg: config::Config);
}
#[async_trait]
impl InitialCoreService for InitialCoreServiceImpl {
    async fn lunch(&self, cfg: config::Config) {
        if !cfg.core.generate_core_org_app {
            return;
        }
        if self.repository.is_org_exist(cfg.organization.name.clone()).await {
            return;
        }
        let org = self.repository.new_org(cfg.organization);
        self.repository.create_org(org.clone()).await;
        let (app, client_key, client_secret) = self
            .repository
            .new_app(org.organization_id.clone(), cfg.application.clone())
            .await;
        self.repository.create_app(app.clone()).await;
        let apporg = self.repository.new_apporg_by_client_id(app, org);
        self.repository.create_apporg_by_client_id(apporg.clone()).await;

        self.print_info(apporg, client_key, client_secret);
    }
}
