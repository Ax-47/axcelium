use std::env;
use serde::Deserialize;
use std::{fs::File, io::BufReader, path::Path};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub core: CoreConfig,
    pub organization: OrganizationConfig,
    pub application: ApplicationConfig,
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
pub struct CoreConfig {
    pub secret: String,
    pub cache_ttl: u64,
    pub generate_core_org_app: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OrganizationConfig {
    pub name: String,
    pub slug: String,
    pub contact_email: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationConfig {
    pub name: String,
    pub description: String,
    pub is_must_name_unique: bool,
    pub can_allow_email_nullable: bool,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let config = serde_yaml::from_reader(reader)?;
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

/// Utility: Get required env
fn get_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("{key} must be set"))
}

/// Utility: Get with default fallback
fn get_env_with_default(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

/// Utility: Parse as u64
fn get_env_u64(key: &str) -> u64 {
    get_env(key)
        .parse::<u64>()
        .unwrap_or_else(|_| panic!("{key} must be a valid u64"))
}

/// Utility: Parse as bool
fn get_env_bool(key: &str) -> bool {
    get_env(key)
        .parse::<bool>()
        .unwrap_or_else(|_| panic!("{key} must be a valid boolean"))
}
