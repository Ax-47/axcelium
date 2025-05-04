use crate::domain::models::app_config::AppConfig;
use serde::Deserialize;
use std::{fs, path::Path};
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub core: CoreConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,

    #[serde(default)]
    pub organization: OrganizationConfig,

    #[serde(default)]
    pub application: ApplicationConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CoreConfig {
    pub secret: String,
    pub generate_core_org_app: bool,
    pub cache_ttl: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub urls: Vec<String>,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub urls: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OrganizationConfig {
    pub name: String,
    pub slug: String,
    pub contact_email: String,
}

impl Default for OrganizationConfig {
    fn default() -> Self {
        Self {
            name: "Axcelium".to_string(),
            slug: "axcelium".to_string(),
            contact_email: "support@axcelium.io".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationConfig {
    pub name: String,
    pub description: String,

    #[serde(default)]
    pub config: AppConfig,
}
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            is_must_name_unique: false,
            can_allow_email_nullable: false,
        }
    }
}
impl Default for ApplicationConfig {
    fn default() -> Self {
        Self {
            name: "Axcelium Core".to_string(),
            description: "The core SSO platform of Axcelium.".to_string(),
            config: AppConfig::default(),
        }
    }
}

fn interpolate_env_vars(yaml: &str) -> String {
    let re = regex::Regex::new(r"\$\{([A-Z0-9_]+)\}").unwrap();
    re.replace_all(yaml, |caps: &regex::Captures| {
        std::env::var(&caps[1]).unwrap_or_else(|_| "".into())
    })
    .to_string()
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();

        let raw = fs::read_to_string(path)?;
        let interpolated = interpolate_env_vars(&raw);

        let settings: config::Config = config::Config::builder()
            .add_source(config::File::from_str(&interpolated, config::FileFormat::Yaml))
            .build()?;

        let config: Config = settings.try_deserialize()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.database.urls.is_empty() {
            return Err("database.urls is required".into());
        }
        if self.database.username.trim().is_empty() {
            return Err("database.username is required".into());
        }
        if self.database.password.trim().is_empty() {
            return Err("database.password is required".into());
        }
        if self.redis.urls.is_empty() {
            return Err("redis.urls is required".into());
        }
        if self.core.secret.trim().is_empty() {
            return Err("core.secret is required".into());
        }
        Ok(())
    }
}