use worker::{Request, Response, RouteContext};
use crate::error::{NasaApiError, Result};
use crate::cache::{CacheManager, get_cache_key};
use crate::utils;
use super::make_nasa_request;
use super::HandlerContext;

fn validate_rover(rover: &str) -> Result<()> {
    match rover {
        "curiosity" | "opportunity" | "spirit" => Ok(()),
        _ => Err(NasaApiError::BadRequest(format!("Invalid rover: {}. Must be one of: curiosity, opportunity, spirit", rover)))
    }
}

pub async fn get_rover_photos(req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    
    let rover = ctx.param("rover")
        .ok_or_else(|| NasaApiError::BadRequest("Missing rover parameter".to_string()))?;
    
    validate_rover(rover)?;
    
    let params = utils::parse_query_params(&req)?;
    
    // Validate sol or earth_date is provided
    let has_sol = params.iter().any(|(k, _)| k == "sol");
    let has_earth_date = params.iter().any(|(k, _)| k == "earth_date");
    
    if !has_sol && !has_earth_date {
        return Err(NasaApiError::BadRequest("Either 'sol' or 'earth_date' parameter is required".to_string()).into());
    }
    
    if has_sol && has_earth_date {
        return Err(NasaApiError::BadRequest("Cannot use both 'sol' and 'earth_date' parameters".to_string()).into());
    }
    
    // Check cache
    let cache_key = get_cache_key(&format!("mars-photos/{}/photos", rover), &params);
    let cache_manager = CacheManager::new(env)?;
    
    if let Some(cached) = cache_manager.get(&cache_key).await? {
        let mut response = Response::from_json(&cached.data)?;
        response.headers_mut().set("X-Cache-Status", "HIT")?;
        return Ok(response);
    }
    
    // Build NASA API URL
    let mut url = format!("https://api.nasa.gov/mars-photos/api/v1/rovers/{}/photos", rover);
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
    
    // Cache for 6 hours
    cache_manager.set(&cache_key, json_value.clone(), 360).await?;
    
    let mut response = Response::from_json(&json_value)?;
    response.headers_mut().set("X-Cache-Status", "MISS")?;
    Ok(response)
}

pub async fn get_latest_photos(_req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    
    let rover = ctx.param("rover")
        .ok_or_else(|| NasaApiError::BadRequest("Missing rover parameter".to_string()))?;
    
    validate_rover(rover)?;
    
    // Check cache
    let cache_key = format!("mars-photos/{}/latest", rover);
    let cache_manager = CacheManager::new(env)?;
    
    if let Some(cached) = cache_manager.get(&cache_key).await? {
        let mut response = Response::from_json(&cached.data)?;
        response.headers_mut().set("X-Cache-Status", "HIT")?;
        return Ok(response);
    }
    
    let url = format!("https://api.nasa.gov/mars-photos/api/v1/rovers/{}/latest_photos", rover);
    
    let mut response = make_nasa_request(&url, &ctx).await?;
    let body = response.text().await?;
    let json_value: serde_json::Value = serde_json::from_str(&body)?;
    
    // Cache for 1 hour (latest photos change more frequently)
    cache_manager.set(&cache_key, json_value.clone(), 60).await?;
    
    let mut response = Response::from_json(&json_value)?;
    response.headers_mut().set("X-Cache-Status", "MISS")?;
    Ok(response)
}

pub async fn get_manifest(_req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    
    let rover = ctx.param("rover")
        .ok_or_else(|| NasaApiError::BadRequest("Missing rover parameter".to_string()))?;
    
    validate_rover(rover)?;
    
    // Check cache
    let cache_key = format!("mars-photos/manifests/{}", rover);
    let cache_manager = CacheManager::new(env)?;
    
    if let Some(cached) = cache_manager.get(&cache_key).await? {
        let mut response = Response::from_json(&cached.data)?;
        response.headers_mut().set("X-Cache-Status", "HIT")?;
        return Ok(response);
    }
    
    let url = format!("https://api.nasa.gov/mars-photos/api/v1/manifests/{}", rover);
    
    let mut response = make_nasa_request(&url, &ctx).await?;
    let body = response.text().await?;
    let json_value: serde_json::Value = serde_json::from_str(&body)?;
    
    // Cache for 24 hours (manifest data is relatively stable)
    cache_manager.set(&cache_key, json_value.clone(), 1440).await?;
    
    let mut response = Response::from_json(&json_value)?;
    response.headers_mut().set("X-Cache-Status", "MISS")?;
    Ok(response)
}