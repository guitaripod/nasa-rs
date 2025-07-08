use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use worker::{kv::KvStore, Env};
use crate::error::NasaApiError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedResponse {
    pub data: serde_json::Value,
    pub cached_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

pub struct CacheManager {
    kv: KvStore,
}

impl CacheManager {
    pub fn new(env: &Env) -> worker::Result<Self> {
        let kv = env.kv("NASA_CACHE")?;
        
        Ok(Self { kv })
    }
    
    pub async fn get(&self, key: &str) -> worker::Result<Option<CachedResponse>> {
        match self.kv.get(key).json::<CachedResponse>().await {
            Ok(Some(cached)) => {
                // Check if cache is expired
                if cached.expires_at > Utc::now() {
                    Ok(Some(cached))
                } else {
                    // Delete expired cache
                    let _ = self.kv.delete(key).await;
                    Ok(None)
                }
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
    
    pub async fn set(&self, key: &str, data: serde_json::Value, ttl_minutes: i64) -> worker::Result<()> {
        let now = Utc::now();
        let cached_response = CachedResponse {
            data,
            cached_at: now,
            expires_at: now + Duration::minutes(ttl_minutes),
        };
        
        self.kv
            .put(key, serde_json::to_string(&cached_response).map_err(|e| worker::Error::RustError(e.to_string()))?)
            ?
            .expiration_ttl(ttl_minutes as u64 * 60)
            .execute()
            .await
            ?;
        
        Ok(())
    }
    
    pub async fn delete(&self, key: &str) -> worker::Result<()> {
        self.kv
            .delete(key)
            .await
            ?;
        
        Ok(())
    }
}

pub fn get_cache_key(endpoint: &str, params: &[(String, String)]) -> String {
    let mut sorted_params = params.to_vec();
    sorted_params.sort_by(|a, b| a.0.cmp(&b.0));
    
    let param_string = sorted_params
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&");
    
    format!("{}:{}", endpoint, param_string)
}

pub fn get_ttl_for_endpoint(endpoint: &str) -> i64 {
    match endpoint {
        // APOD updates daily
        "apod" => 1440, // 24 hours
        
        // Near real-time data
        "neo/feed" => 60, // 1 hour
        "donki/cme" => 30, // 30 minutes
        "donki/flr" => 30, // 30 minutes
        "donki/notifications" => 15, // 15 minutes
        
        // Static or slowly changing data
        "neo/browse" => 360, // 6 hours
        "mars-photos/manifests" => 1440, // 24 hours
        "techtransfer" => 10080, // 1 week
        "media/search" => 360, // 6 hours
        "exoplanets/query" => 1440, // 24 hours
        
        // Default
        _ => 60, // 1 hour
    }
}
