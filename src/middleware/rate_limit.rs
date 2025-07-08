use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use worker::{kv::KvStore, Env, Request};
use crate::error::{NasaApiError, Result};
use crate::utils::get_client_ip;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitInfo {
    pub count: u32,
    pub window_start: DateTime<Utc>,
}

#[allow(dead_code)]
pub struct RateLimiter {
    kv: KvStore,
    requests_per_hour: u32,
}

#[allow(dead_code)]
impl RateLimiter {
    pub fn new(env: &Env, requests_per_hour: u32) -> Result<Self> {
        let kv = env.kv("RATE_LIMIT")
            .map_err(|e| NasaApiError::Cache(format!("Failed to get rate limit KV: {e}")))?;
        
        Ok(Self {
            kv,
            requests_per_hour,
        })
    }
    
    pub async fn check_rate_limit(&self, req: &Request) -> Result<bool> {
        let client_ip = get_client_ip(req);
        let key = format!("rate_limit:{client_ip}");
        let now = Utc::now();
        
        match self.kv.get(&key).json::<RateLimitInfo>().await {
            Ok(Some(mut info)) => {
                // Check if we're still in the same window
                if now - info.window_start < Duration::hours(1) {
                    if info.count >= self.requests_per_hour {
                        return Ok(false); // Rate limit exceeded
                    }
                    info.count += 1;
                } else {
                    // New window
                    info = RateLimitInfo {
                        count: 1,
                        window_start: now,
                    };
                }
                
                // Update the count
                self.kv
                    .put(&key, serde_json::to_string(&info)?)
                    .map_err(|e| NasaApiError::Cache(format!("Failed to update rate limit: {e}")))?
                    .expiration_ttl(3600) // 1 hour
                    .execute()
                    .await
                    .map_err(|e| NasaApiError::Cache(format!("Failed to execute rate limit update: {e}")))?;
                
                Ok(true)
            }
            Ok(None) => {
                // First request
                let info = RateLimitInfo {
                    count: 1,
                    window_start: now,
                };
                
                self.kv
                    .put(&key, serde_json::to_string(&info)?)
                    .map_err(|e| NasaApiError::Cache(format!("Failed to set rate limit: {e}")))?
                    .expiration_ttl(3600) // 1 hour
                    .execute()
                    .await
                    .map_err(|e| NasaApiError::Cache(format!("Failed to execute rate limit set: {e}")))?;
                
                Ok(true)
            }
            Err(e) => Err(NasaApiError::Cache(format!("Failed to get rate limit: {e}"))),
        }
    }
}