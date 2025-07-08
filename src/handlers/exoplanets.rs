use worker::{Request, Response, RouteContext};
use crate::error::NasaApiError;
use crate::cache::{CacheManager, get_cache_key, get_ttl_for_endpoint};
use crate::utils;

use super::HandlerContext;
pub async fn query_exoplanets(req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    let params = utils::parse_query_params(&req)?;
    
    // Extract and validate query parameter
    let query = params.iter()
        .find(|(k, _)| k == "query")
        .ok_or_else(|| NasaApiError::BadRequest("Missing required parameter: query".to_string()))?
        .1.clone();
    
    let format = params.iter()
        .find(|(k, _)| k == "format")
        .map(|(_, v)| v.clone())
        .unwrap_or_else(|| "json".to_string());
    
    // Check cache
    let cache_key = get_cache_key("exoplanets/query", &params);
    let cache_manager = CacheManager::new(env)?;
    
    if let Some(cached) = cache_manager.get(&cache_key).await? {
        let mut response = Response::from_json(&cached.data)?;
        response.headers_mut().set("X-Cache-Status", "HIT")?;
        return Ok(response);
    }
    
    // Build URL for Exoplanet Archive TAP service
    let url = format!(
        "https://exoplanetarchive.ipac.caltech.edu/TAP/sync?query={}&format={}",
        urlencoding::encode(&query),
        urlencoding::encode(&format)
    );
    
    let response = reqwest::get(&url)
        .await
        .map_err(|e| NasaApiError::Request(e.to_string()))?;
    
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(NasaApiError::NasaApi(format!(
            "Exoplanet Archive returned {status} - {error_text}"
        )).into());
    }
    
    let body = response.text().await
        .map_err(|e| NasaApiError::Request(e.to_string()))?;
    
    // Handle different response formats
    let json_value = if format == "json" {
        serde_json::from_str(&body)?
    } else {
        // For non-JSON formats, wrap in a simple object
        serde_json::json!({
            "format": format,
            "data": body
        })
    };
    
    // Cache the response
    let ttl = get_ttl_for_endpoint("exoplanets/query");
    cache_manager.set(&cache_key, json_value.clone(), ttl).await?;
    
    let mut response = Response::from_json(&json_value)?;
    response.headers_mut().set("X-Cache-Status", "MISS")?;
    Ok(response)
}