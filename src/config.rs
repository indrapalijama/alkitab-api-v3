use serde::Deserialize;
use std::env;
use config::{Config, ConfigError, File};
use lazy_static::lazy_static;

#[derive(Debug, Deserialize, Clone)]
pub struct BibleConfig {
    pub base_url: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub bible: BibleConfig,
    #[serde(skip)]
    pub environment: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let environment = env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());
        
        let config = Config::builder()
            .add_source(File::with_name("config/default.toml"))
            .add_source(File::with_name(&format!("config/{}.toml", environment)).required(false))
            .build()?;
        let mut app_config: AppConfig = config.try_deserialize()?;
        app_config.environment = environment;        
        Ok(app_config)
    }
}

lazy_static! {
    pub static ref CONFIG: AppConfig = AppConfig::load().expect("Failed to load configuration");
}