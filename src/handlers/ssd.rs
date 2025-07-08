use worker::{Request, Response, RouteContext};
use crate::error::{NasaApiError, Result};
use crate::cache::{CacheManager, get_cache_key};
use crate::utils;

use super::HandlerContext;
async fn make_ssd_request(url: &str) -> worker::Result<Response> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| NasaApiError::Request(e.to_string()))?;
    
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(NasaApiError::NasaApi(format!(
            "JPL SSD API returned {} - {}",
            status, error_text
        )).into());
    }
    
    let body = response.text().await
        .map_err(|e| NasaApiError::Request(e.to_string()))?;
    
    Response::ok(body)
}

pub async fn get_close_approach(_req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    let params = utils::parse_query_params(&_req)?;
    
    // Check cache
    let cache_key = get_cache_key("ssd/cad", &params);
    let cache_manager = CacheManager::new(env)?;
    
    if let Some(cached) = cache_manager.get(&cache_key).await? {
        let mut response = Response::from_json(&cached.data)?;
        response.headers_mut().set("X-Cache-Status", "HIT")?;
        return Ok(response);
    }
    
    // Build URL
    let mut url = "https://ssd-api.jpl.nasa.gov/cad.api".to_string();
    let mut first_param = true;
    
    for (key, value) in &params {
        if first_param {
            url.push('?');
            first_param = false;
        } else {
            url.push('&');
        }
        url.push_str(&format!("{}={}", key, urlencoding::encode(&value)));
    }
    
    let mut response = make_ssd_request(&url).await?;
    let body = response.text().await?;
    let json_value: serde_json::Value = serde_json::from_str(&body)?;
    
    // Cache for 1 hour
    cache_manager.set(&cache_key, json_value.clone(), 60).await?;
    
    let mut response = Response::from_json(&json_value)?;
    response.headers_mut().set("X-Cache-Status", "MISS")?;
    Ok(response)
}

pub async fn get_small_body(_req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    let params = utils::parse_query_params(&_req)?;
    
    // Extract search string
    let _sstr = params.iter()
        .find(|(k, _)| k == "sstr")
        .ok_or_else(|| NasaApiError::BadRequest("Missing required parameter: sstr".to_string()))?
        .1.clone();
    
    // Check cache
    let cache_key = get_cache_key("ssd/sbdb", &params);
    let cache_manager = CacheManager::new(env)?;
    
    if let Some(cached) = cache_manager.get(&cache_key).await? {
        let mut response = Response::from_json(&cached.data)?;
        response.headers_mut().set("X-Cache-Status", "HIT")?;
        return Ok(response);
    }
    
    // Build URL
    let mut url = "https://ssd-api.jpl.nasa.gov/sbdb.api".to_string();
    let mut first_param = true;
    
    for (key, value) in &params {
        if first_param {
            url.push('?');
            first_param = false;
        } else {
            url.push('&');
        }
        url.push_str(&format!("{}={}", key, urlencoding::encode(&value)));
    }
    
    let mut response = make_ssd_request(&url).await?;
    let body = response.text().await?;
    let json_value: serde_json::Value = serde_json::from_str(&body)?;
    
    // Cache for 6 hours
    cache_manager.set(&cache_key, json_value.clone(), 360).await?;
    
    let mut response = Response::from_json(&json_value)?;
    response.headers_mut().set("X-Cache-Status", "MISS")?;
    Ok(response)
}

pub async fn get_sentry(_req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    let params = utils::parse_query_params(&_req)?;
    
    // Check cache
    let cache_key = get_cache_key("ssd/sentry", &params);
    let cache_manager = CacheManager::new(env)?;
    
    if let Some(cached) = cache_manager.get(&cache_key).await? {
        let mut response = Response::from_json(&cached.data)?;
        response.headers_mut().set("X-Cache-Status", "HIT")?;
        return Ok(response);
    }
    
    // Build URL
    let mut url = "https://ssd-api.jpl.nasa.gov/sentry.api".to_string();
    let mut first_param = true;
    
    for (key, value) in &params {
        if first_param {
            url.push('?');
            first_param = false;
        } else {
            url.push('&');
        }
        url.push_str(&format!("{}={}", key, urlencoding::encode(&value)));
    }
    
    let mut response = make_ssd_request(&url).await?;
    let body = response.text().await?;
    let json_value: serde_json::Value = serde_json::from_str(&body)?;
    
    // Cache for 1 hour (impact risk data is important)
    cache_manager.set(&cache_key, json_value.clone(), 60).await?;
    
    let mut response = Response::from_json(&json_value)?;
    response.headers_mut().set("X-Cache-Status", "MISS")?;
    Ok(response)
}

pub async fn get_scout(_req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    let params = utils::parse_query_params(&_req)?;
    
    // Extract temporary designation
    let tdes = params.iter()
        .find(|(k, _)| k == "tdes")
        .ok_or_else(|| NasaApiError::BadRequest("Missing required parameter: tdes".to_string()))?
        .1.clone();
    
    // Check cache
    let cache_key = get_cache_key("ssd/scout", &params);
    let cache_manager = CacheManager::new(env)?;
    
    if let Some(cached) = cache_manager.get(&cache_key).await? {
        let mut response = Response::from_json(&cached.data)?;
        response.headers_mut().set("X-Cache-Status", "HIT")?;
        return Ok(response);
    }
    
    // Build URL
    let url = format!("https://ssd-api.jpl.nasa.gov/scout.api?tdes={}", urlencoding::encode(&tdes));
    
    let mut response = make_ssd_request(&url).await?;
    let body = response.text().await?;
    let json_value: serde_json::Value = serde_json::from_str(&body)?;
    
    // Cache for 30 minutes (Scout data for new objects changes frequently)
    cache_manager.set(&cache_key, json_value.clone(), 30).await?;
    
    let mut response = Response::from_json(&json_value)?;
    response.headers_mut().set("X-Cache-Status", "MISS")?;
    Ok(response)
}

pub async fn get_nhats(_req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    let params = utils::parse_query_params(&_req)?;
    
    // Check cache
    let cache_key = get_cache_key("ssd/nhats", &params);
    let cache_manager = CacheManager::new(env)?;
    
    if let Some(cached) = cache_manager.get(&cache_key).await? {
        let mut response = Response::from_json(&cached.data)?;
        response.headers_mut().set("X-Cache-Status", "HIT")?;
        return Ok(response);
    }
    
    // Build URL
    let mut url = "https://ssd-api.jpl.nasa.gov/nhats.api".to_string();
    let mut first_param = true;
    
    for (key, value) in &params {
        if first_param {
            url.push('?');
            first_param = false;
        } else {
            url.push('&');
        }
        url.push_str(&format!("{}={}", key, urlencoding::encode(&value)));
    }
    
    let mut response = make_ssd_request(&url).await?;
    let body = response.text().await?;
    let json_value: serde_json::Value = serde_json::from_str(&body)?;
    
    // Cache for 24 hours (NHATS data is relatively stable)
    cache_manager.set(&cache_key, json_value.clone(), 1440).await?;
    
    let mut response = Response::from_json(&json_value)?;
    response.headers_mut().set("X-Cache-Status", "MISS")?;
    Ok(response)
}

pub async fn get_fireballs(_req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    let params = utils::parse_query_params(&_req)?;
    
    // Check cache
    let cache_key = get_cache_key("ssd/fireballs", &params);
    let cache_manager = CacheManager::new(env)?;
    
    if let Some(cached) = cache_manager.get(&cache_key).await? {
        let mut response = Response::from_json(&cached.data)?;
        response.headers_mut().set("X-Cache-Status", "HIT")?;
        return Ok(response);
    }
    
    // Build URL
    let mut url = "https://ssd-api.jpl.nasa.gov/fireball.api".to_string();
    let mut first_param = true;
    
    for (key, value) in &params {
        if first_param {
            url.push('?');
            first_param = false;
        } else {
            url.push('&');
        }
        url.push_str(&format!("{}={}", key, urlencoding::encode(&value)));
    }
    
    let mut response = make_ssd_request(&url).await?;
    let body = response.text().await?;
    let json_value: serde_json::Value = serde_json::from_str(&body)?;
    
    // Cache for 1 hour
    cache_manager.set(&cache_key, json_value.clone(), 60).await?;
    
    let mut response = Response::from_json(&json_value)?;
    response.headers_mut().set("X-Cache-Status", "MISS")?;
    Ok(response)
}