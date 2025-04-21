use serde::Deserialize;
use std::env;
use config::{Config, ConfigError, Environment, File};
use lazy_static::lazy_static;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16
}

#[derive(Debug, Deserialize, Clone)]
pub struct BibleConfig {
    pub base_url: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub bible: BibleConfig,
    #[serde(skip)]
    pub environment: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let environment = env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());
        
        let config = Config::builder()
            // Start with default settings
            .add_source(File::with_name("config/default"))
            // Add environment-specific settings
            .add_source(File::with_name(&format!("config/{}", environment)).required(false))
            // Add local settings (gitignored)
            .add_source(File::with_name("config/local").required(false))
            // Add environment variables with prefix "APP_"
            .add_source(Environment::with_prefix("APP"))
            .build()?;
        
        let mut app_config: AppConfig = config.try_deserialize()?;
        app_config.environment = environment;
        
        Ok(app_config)
    }
}

lazy_static! {
    pub static ref CONFIG: AppConfig = AppConfig::load().expect("Failed to load configuration");
} 