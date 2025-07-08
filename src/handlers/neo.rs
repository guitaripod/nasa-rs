use worker::{Request, Response, RouteContext};
use crate::error::{NasaApiError, Result};
use crate::cache::{CacheManager, get_cache_key, get_ttl_for_endpoint};
use crate::utils;
use super::make_nasa_request;
use super::HandlerContext;

pub async fn get_neo_feed(_req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    let params = utils::parse_query_params(&_req)?;
    
    // Check cache
    let cache_key = get_cache_key("neo/feed", &params);
    let cache_manager = CacheManager::new(env)?;
    
    if let Some(cached) = cache_manager.get(&cache_key).await? {
        let mut response = Response::from_json(&cached.data)?;
        response.headers_mut().set("X-Cache-Status", "HIT")?;
        return Ok(response);
    }
    
    // Build NASA API URL
    let mut url = "https://api.nasa.gov/neo/rest/v1/feed".to_string();
    let mut first_param = true;
    
    for (key, value) in &params {
        if key != "api_key" {
            if first_param {
                url.push('?');
                first_param = false;
            } else {
                url.push('&');
            }
            url.push_str(&format!("{}={}", key, value));
        }
    }
    
    let mut response = make_nasa_request(&url, &ctx).await?;
    let body = response.text().await?;
    let json_value: serde_json::Value = serde_json::from_str(&body)?;
    
    // Cache the response
    let ttl = get_ttl_for_endpoint("neo/feed");
    cache_manager.set(&cache_key, json_value.clone(), ttl).await?;
    
    let mut response = Response::from_json(&json_value)?;
    response.headers_mut().set("X-Cache-Status", "MISS")?;
    Ok(response)
}

pub async fn get_neo_lookup(_req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    
    let asteroid_id = ctx.param("asteroid_id")
        .ok_or_else(|| NasaApiError::BadRequest("Missing asteroid_id parameter".to_string()))?;
    
    let _params = utils::parse_query_params(&_req)?;
    
    // Check cache
    let cache_key = format!("neo/lookup:{}", asteroid_id);
    let cache_manager = CacheManager::new(env)?;
    
    if let Some(cached) = cache_manager.get(&cache_key).await? {
        let mut response = Response::from_json(&cached.data)?;
        response.headers_mut().set("X-Cache-Status", "HIT")?;
        return Ok(response);
    }
    
    let url = format!("https://api.nasa.gov/neo/rest/v1/neo/{}", asteroid_id);
    
    let mut response = make_nasa_request(&url, &ctx).await?;
    let body = response.text().await?;
    let json_value: serde_json::Value = serde_json::from_str(&body)?;
    
    // Cache the response (6 hours for lookup data)
    cache_manager.set(&cache_key, json_value.clone(), 360).await?;
    
    let mut response = Response::from_json(&json_value)?;
    response.headers_mut().set("X-Cache-Status", "MISS")?;
    Ok(response)
}

pub async fn get_neo_browse(_req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    let params = utils::parse_query_params(&_req)?;
    
    // Check cache
    let cache_key = get_cache_key("neo/browse", &params);
    let cache_manager = CacheManager::new(env)?;
    
    if let Some(cached) = cache_manager.get(&cache_key).await? {
        let mut response = Response::from_json(&cached.data)?;
        response.headers_mut().set("X-Cache-Status", "HIT")?;
        return Ok(response);
    }
    
    // Build NASA API URL
    let mut url = "https://api.nasa.gov/neo/rest/v1/neo/browse".to_string();
    let mut first_param = true;
    
    for (key, value) in &params {
        if key != "api_key" {
            if first_param {
                url.push('?');
                first_param = false;
            } else {
                url.push('&');
            }
            url.push_str(&format!("{}={}", key, value));
        }
    }
    
    let mut response = make_nasa_request(&url, &ctx).await?;
    let body = response.text().await?;
    let json_value: serde_json::Value = serde_json::from_str(&body)?;
    
    // Cache the response
    let ttl = get_ttl_for_endpoint("neo/browse");
    cache_manager.set(&cache_key, json_value.clone(), ttl).await?;
    
    let mut response = Response::from_json(&json_value)?;
    response.headers_mut().set("X-Cache-Status", "MISS")?;
    Ok(response)
}