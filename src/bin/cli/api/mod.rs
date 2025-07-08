use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;
use sha2::{Sha256, Digest};

pub struct ApiClient {
    client: Client,
    base_url: String,
    cache_dir: PathBuf,
    use_cache: bool,
    cache_ttl_minutes: u64,
}

impl ApiClient {
    pub fn new(base_url: String, cache_dir: PathBuf, use_cache: bool, cache_ttl_minutes: u64) -> Self {
        Self {
            client: Client::new(),
            base_url,
            cache_dir,
            use_cache,
            cache_ttl_minutes,
        }
    }
    
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
    
    
    pub async fn get(&self, path: &str, params: HashMap<String, String>) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}{path}", self.base_url);
        
        // Check cache first
        if self.use_cache {
            if let Some(cached) = self.get_from_cache(&url, &params).await? {
                return Ok(cached);
            }
        }
        
        // Make request
        let response = self.client
            .get(&url)
            .query(&params)
            .send()
            .await?;
            
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            return Err(format!("API error {status}: {error_text}").into());
        }
        
        let data: Value = response.json().await?;
        
        // Cache the response
        if self.use_cache {
            self.save_to_cache(&url, &params, &data).await?;
        }
        
        Ok(data)
    }
    
    pub async fn get_binary(&self, path: &str, params: HashMap<String, String>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let url = format!("{}{path}", self.base_url);
        
        let response = self.client
            .get(&url)
            .query(&params)
            .send()
            .await?;
            
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            return Err(format!("API error {status}: {error_text}").into());
        }
        
        Ok(response.bytes().await?.to_vec())
    }
    
    async fn get_from_cache(&self, url: &str, params: &HashMap<String, String>) -> Result<Option<Value>, Box<dyn std::error::Error>> {
        let cache_key = self.generate_cache_key(url, params);
        let cache_path = self.cache_dir.join(format!("{cache_key}.json"));
        
        if cache_path.exists() {
            let metadata = fs::metadata(&cache_path).await?;
            if let Ok(modified) = metadata.modified() {
                let age = std::time::SystemTime::now()
                    .duration_since(modified)
                    .unwrap_or(std::time::Duration::MAX);
                    
                if age.as_secs() < self.cache_ttl_minutes * 60 {
                    let content = fs::read_to_string(&cache_path).await?;
                    return Ok(Some(serde_json::from_str(&content)?));
                }
            }
        }
        
        Ok(None)
    }
    
    async fn save_to_cache(&self, url: &str, params: &HashMap<String, String>, data: &Value) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(&self.cache_dir).await?;
        
        let cache_key = self.generate_cache_key(url, params);
        let cache_path = self.cache_dir.join(format!("{cache_key}.json"));
        
        let content = serde_json::to_string_pretty(data)?;
        fs::write(&cache_path, content).await?;
        
        Ok(())
    }
    
    fn generate_cache_key(&self, url: &str, params: &HashMap<String, String>) -> String {
        let mut hasher = Sha256::new();
        hasher.update(url.as_bytes());
        
        let mut sorted_params: Vec<_> = params.iter().collect();
        sorted_params.sort_by_key(|(k, _)| k.as_str());
        
        for (key, value) in sorted_params {
            hasher.update(key.as_bytes());
            hasher.update(value.as_bytes());
        }
        
        let hash = hasher.finalize();
        format!("{hash:x}")
    }
    
    pub async fn clear_cache(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.cache_dir.exists() {
            fs::remove_dir_all(&self.cache_dir).await?;
            fs::create_dir_all(&self.cache_dir).await?;
        }
        Ok(())
    }
    
    pub async fn get_cache_stats(&self) -> Result<(usize, u64), Box<dyn std::error::Error>> {
        let mut count = 0;
        let mut total_size = 0;
        
        if self.cache_dir.exists() {
            let mut entries = fs::read_dir(&self.cache_dir).await?;
            while let Some(entry) = entries.next_entry().await? {
                if let Ok(metadata) = entry.metadata().await {
                    count += 1;
                    total_size += metadata.len();
                }
            }
        }
        
        Ok((count, total_size))
    }
}