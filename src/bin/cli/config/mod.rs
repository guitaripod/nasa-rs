use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use dirs::config_dir;
use tokio::fs;
use crate::cli::output::OutputFormat;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_endpoint: String,
    pub api_key: Option<String>,
    pub output_format: OutputFormat,
    pub use_cache: bool,
    pub cache_dir: PathBuf,
    pub cache_ttl_minutes: u64,
}

impl Default for Config {
    fn default() -> Self {
        let cache_dir = config_dir()
            .map(|dir| dir.join("nasa-cli").join("cache"))
            .unwrap_or_else(|| PathBuf::from(".nasa-cache"));
            
        Self {
            api_endpoint: "https://nasa-api-worker.guitaripod.workers.dev".to_string(),
            api_key: None,
            output_format: OutputFormat::Pretty,
            use_cache: true,
            cache_dir,
            cache_ttl_minutes: 60,
        }
    }
}

pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = config_dir()
            .map(|dir| dir.join("nasa-cli").join("config.toml"))
            .ok_or("Unable to determine config directory")?;
            
        Ok(Self { config_path })
    }
    
    pub async fn load(&self) -> Result<Config, Box<dyn std::error::Error>> {
        if self.config_path.exists() {
            let content = fs::read_to_string(&self.config_path).await?;
            Ok(toml::from_str(&content)?)
        } else {
            Ok(Config::default())
        }
    }
    
    pub async fn save(&self, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        let content = toml::to_string_pretty(config)?;
        fs::write(&self.config_path, content).await?;
        
        Ok(())
    }
    
    pub async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = Config::default();
        self.save(&config).await?;
        
        // Create cache directory
        fs::create_dir_all(&config.cache_dir).await?;
        
        Ok(())
    }
    
    pub async fn set(&self, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = self.load().await?;
        
        match key {
            "api_endpoint" => config.api_endpoint = value.to_string(),
            "api_key" => config.api_key = Some(value.to_string()),
            "output_format" => {
                use std::str::FromStr;
                config.output_format = OutputFormat::from_str(value)?;
            }
            "use_cache" => config.use_cache = value.parse()?,
            "cache_ttl_minutes" => config.cache_ttl_minutes = value.parse()?,
            _ => return Err(format!("Unknown config key: {key}").into()),
        }
        
        self.save(&config).await?;
        Ok(())
    }
}