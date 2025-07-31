use serde::Deserialize;
use std::env;
use config::{Config, ConfigError, Environment, File};
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
        // Debug: Print current working directory and list files
        if let Ok(current_dir) = env::current_dir() {
            println!("Current working directory: {}", current_dir.display());
        }
        
        if let Ok(entries) = std::fs::read_dir(".") {
            println!("Files in current directory:");
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("  - {}", entry.path().display());
                }
            }
        }
        
        let environment = env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());
        
        let config = Config::builder()
            // Load default settings (required) - be explicit about the file extension
            .add_source(File::with_name("config/default.toml"))
            // Add environment-specific settings (optional)
            .add_source(File::with_name(&format!("config/{}.toml", environment)).required(false))
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