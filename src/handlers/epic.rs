use worker::{Request, Response, RouteContext};
use crate::error::{NasaApiError, Result};
use crate::cache::CacheManager;
use crate::utils;

use super::HandlerContext;
async fn make_epic_request(url: &str, ctx: &RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    let api_key = utils::get_api_key(env)?;
    
    // EPIC API uses api_key parameter differently than other NASA APIs
    let full_url = if url.contains('?') {
        format!("{}&api_key={}", url, api_key)
    } else {
        format!("{}?api_key={}", url, api_key)
    };
    
    let response = reqwest::get(&full_url)
        .await
        .map_err(|e| NasaApiError::Request(e.to_string()))?;
    
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(NasaApiError::NasaApi(format!(
            "NASA EPIC API returned {} - {}",
            status, error_text
        )).into());
    }
    
    let body = response.text().await
        .map_err(|e| NasaApiError::Request(e.to_string()))?;
    
    Response::ok(body)
}

pub async fn get_natural_all(_req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    
    // Check cache
    let cache_key = "epic/natural/all";
    let cache_manager = CacheManager::new(env)?;
    
    if let Some(cached) = cache_manager.get(cache_key).await? {
        let mut response = Response::from_json(&cached.data)?;
        response.headers_mut().set("X-Cache-Status", "HIT")?;
        return Ok(response);
    }
    
    let url = "https://api.nasa.gov/EPIC/api/natural/all";
    
    let mut response = make_epic_request(url, &ctx).await?;
    let body = response.text().await?;
    let json_value: serde_json::Value = serde_json::from_str(&body)?;
    
    // Cache for 6 hours
    cache_manager.set(cache_key, json_value.clone(), 360).await?;
    
    let mut response = Response::from_json(&json_value)?;
    response.headers_mut().set("X-Cache-Status", "MISS")?;
    Ok(response)
}

pub async fn get_natural_date(_req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    
    let date = ctx.param("date")
        .ok_or_else(|| NasaApiError::BadRequest("Missing date parameter".to_string()))?;
    
    // Check cache
    let cache_key = format!("epic/natural/date:{}", date);
    let cache_manager = CacheManager::new(env)?;
    
    if let Some(cached) = cache_manager.get(&cache_key).await? {
        let mut response = Response::from_json(&cached.data)?;
        response.headers_mut().set("X-Cache-Status", "HIT")?;
        return Ok(response);
    }
    
    let url = format!("https://api.nasa.gov/EPIC/api/natural/date/{}", date);
    
    let mut response = make_epic_request(&url, &ctx).await?;
    let body = response.text().await?;
    let json_value: serde_json::Value = serde_json::from_str(&body)?;
    
    // Cache for 24 hours
    cache_manager.set(&cache_key, json_value.clone(), 1440).await?;
    
    let mut response = Response::from_json(&json_value)?;
    response.headers_mut().set("X-Cache-Status", "MISS")?;
    Ok(response)
}

pub async fn get_enhanced_all(_req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    
    // Check cache
    let cache_key = "epic/enhanced/all";
    let cache_manager = CacheManager::new(env)?;
    
    if let Some(cached) = cache_manager.get(cache_key).await? {
        let mut response = Response::from_json(&cached.data)?;
        response.headers_mut().set("X-Cache-Status", "HIT")?;
        return Ok(response);
    }
    
    let url = "https://api.nasa.gov/EPIC/api/enhanced/all";
    
    let mut response = make_epic_request(url, &ctx).await?;
    let body = response.text().await?;
    let json_value: serde_json::Value = serde_json::from_str(&body)?;
    
    // Cache for 6 hours
    cache_manager.set(cache_key, json_value.clone(), 360).await?;
    
    let mut response = Response::from_json(&json_value)?;
    response.headers_mut().set("X-Cache-Status", "MISS")?;
    Ok(response)
}

pub async fn get_enhanced_date(_req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    
    let date = ctx.param("date")
        .ok_or_else(|| NasaApiError::BadRequest("Missing date parameter".to_string()))?;
    
    // Check cache
    let cache_key = format!("epic/enhanced/date:{}", date);
    let cache_manager = CacheManager::new(env)?;
    
    if let Some(cached) = cache_manager.get(&cache_key).await? {
        let mut response = Response::from_json(&cached.data)?;
        response.headers_mut().set("X-Cache-Status", "HIT")?;
        return Ok(response);
    }
    
    let url = format!("https://api.nasa.gov/EPIC/api/enhanced/date/{}", date);
    
    let mut response = make_epic_request(&url, &ctx).await?;
    let body = response.text().await?;
    let json_value: serde_json::Value = serde_json::from_str(&body)?;
    
    // Cache for 24 hours
    cache_manager.set(&cache_key, json_value.clone(), 1440).await?;
    
    let mut response = Response::from_json(&json_value)?;
    response.headers_mut().set("X-Cache-Status", "MISS")?;
    Ok(response)
}