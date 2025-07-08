use worker::{Request, Response, RouteContext};
use crate::error::NasaApiError;
use crate::cache::{CacheManager, get_cache_key, get_ttl_for_endpoint};
use crate::utils;

use super::HandlerContext;
async fn make_media_request(url: &str) -> worker::Result<Response> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| NasaApiError::Request(e.to_string()))?;
    
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(NasaApiError::NasaApi(format!(
            "NASA Media API returned {status} - {error_text}"
        )).into());
    }
    
    let body = response.text().await
        .map_err(|e| NasaApiError::Request(e.to_string()))?;
    
    Response::ok(body)
}

pub async fn search_media(_req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    let params = utils::parse_query_params(&_req)?;
    
    // Check cache
    let cache_key = get_cache_key("media/search", &params);
    let cache_manager = CacheManager::new(env)?;
    
    if let Some(cached) = cache_manager.get(&cache_key).await? {
        let mut response = Response::from_json(&cached.data)?;
        response.headers_mut().set("X-Cache-Status", "HIT")?;
        return Ok(response);
    }
    
    // Build URL - NASA Image and Video Library doesn't require API key
    let mut url = "https://images-api.nasa.gov/search".to_string();
    let mut first_param = true;
    
    for (key, value) in &params {
        if key != "api_key" { // Skip api_key as it's not needed
            if first_param {
                url.push('?');
                first_param = false;
            } else {
                url.push('&');
            }
            url.push_str(&format!("{key}={}", urlencoding::encode(value)));
        }
    }
    
    let mut response = make_media_request(&url).await?;
    let body = response.text().await?;
    let json_value: serde_json::Value = serde_json::from_str(&body)?;
    
    // Cache the response
    let ttl = get_ttl_for_endpoint("media/search");
    cache_manager.set(&cache_key, json_value.clone(), ttl).await?;
    
    let mut response = Response::from_json(&json_value)?;
    response.headers_mut().set("X-Cache-Status", "MISS")?;
    Ok(response)
}

pub async fn get_asset(_req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    
    let nasa_id = ctx.param("nasa_id")
        .ok_or_else(|| NasaApiError::BadRequest("Missing nasa_id parameter".to_string()))?;
    
    // Check cache
    let cache_key = format!("media/asset:{nasa_id}");
    let cache_manager = CacheManager::new(env)?;
    
    if let Some(cached) = cache_manager.get(&cache_key).await? {
        let mut response = Response::from_json(&cached.data)?;
        response.headers_mut().set("X-Cache-Status", "HIT")?;
        return Ok(response);
    }
    
    let url = format!("https://images-api.nasa.gov/asset/{}", urlencoding::encode(nasa_id));
    
    let mut response = make_media_request(&url).await?;
    let body = response.text().await?;
    let json_value: serde_json::Value = serde_json::from_str(&body)?;
    
    // Cache for 24 hours
    cache_manager.set(&cache_key, json_value.clone(), 1440).await?;
    
    let mut response = Response::from_json(&json_value)?;
    response.headers_mut().set("X-Cache-Status", "MISS")?;
    Ok(response)
}

pub async fn get_metadata(_req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    
    let nasa_id = ctx.param("nasa_id")
        .ok_or_else(|| NasaApiError::BadRequest("Missing nasa_id parameter".to_string()))?;
    
    // Check cache
    let cache_key = format!("media/metadata:{nasa_id}");
    let cache_manager = CacheManager::new(env)?;
    
    if let Some(cached) = cache_manager.get(&cache_key).await? {
        let mut response = Response::from_json(&cached.data)?;
        response.headers_mut().set("X-Cache-Status", "HIT")?;
        return Ok(response);
    }
    
    let url = format!("https://images-api.nasa.gov/metadata/{}", urlencoding::encode(nasa_id));
    
    let mut response = make_media_request(&url).await?;
    let body = response.text().await?;
    let json_value: serde_json::Value = serde_json::from_str(&body)?;
    
    // Cache for 24 hours
    cache_manager.set(&cache_key, json_value.clone(), 1440).await?;
    
    let mut response = Response::from_json(&json_value)?;
    response.headers_mut().set("X-Cache-Status", "MISS")?;
    Ok(response)
}

pub async fn get_captions(_req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    
    let nasa_id = ctx.param("nasa_id")
        .ok_or_else(|| NasaApiError::BadRequest("Missing nasa_id parameter".to_string()))?;
    
    // Check cache
    let cache_key = format!("media/captions:{nasa_id}");
    let cache_manager = CacheManager::new(env)?;
    
    if let Some(cached) = cache_manager.get(&cache_key).await? {
        let mut response = Response::from_json(&cached.data)?;
        response.headers_mut().set("X-Cache-Status", "HIT")?;
        return Ok(response);
    }
    
    let url = format!("https://images-api.nasa.gov/captions/{}", urlencoding::encode(nasa_id));
    
    let mut response = make_media_request(&url).await?;
    let body = response.text().await?;
    let json_value: serde_json::Value = serde_json::from_str(&body)?;
    
    // Cache for 24 hours
    cache_manager.set(&cache_key, json_value.clone(), 1440).await?;
    
    let mut response = Response::from_json(&json_value)?;
    response.headers_mut().set("X-Cache-Status", "MISS")?;
    Ok(response)
}