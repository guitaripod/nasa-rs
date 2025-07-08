use worker::{Request, Response, RouteContext};
use crate::error::{NasaApiError, Result};
use crate::cache::{CacheManager, get_cache_key};
use crate::utils;
use super::make_nasa_request;
use super::HandlerContext;

pub async fn get_imagery(req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let params = utils::parse_query_params(&req)?;
    
    // Validate required parameters
    let lat = params.iter()
        .find(|(k, _)| k == "lat")
        .ok_or_else(|| NasaApiError::BadRequest("Missing required parameter: lat".to_string()))?
        .1.clone();
    
    let lon = params.iter()
        .find(|(k, _)| k == "lon")
        .ok_or_else(|| NasaApiError::BadRequest("Missing required parameter: lon".to_string()))?
        .1.clone();
    
    // Validate lat/lon ranges
    let lat_f: f64 = lat.parse()
        .map_err(|_| NasaApiError::BadRequest("Invalid latitude format".to_string()))?;
    let lon_f: f64 = lon.parse()
        .map_err(|_| NasaApiError::BadRequest("Invalid longitude format".to_string()))?;
    
    if lat_f < -90.0 || lat_f > 90.0 {
        return Err(NasaApiError::BadRequest("Latitude must be between -90 and 90".to_string()).into());
    }
    
    if lon_f < -180.0 || lon_f > 180.0 {
        return Err(NasaApiError::BadRequest("Longitude must be between -180 and 180".to_string()).into());
    }
    
    // For imagery endpoint, we don't cache as it returns binary data
    let mut url = "https://api.nasa.gov/planetary/earth/imagery".to_string();
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
    
    // Make request and return image directly
    let response = make_nasa_request(&url, &ctx).await?;
    Ok(response)
}

pub async fn get_assets(req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    let params = utils::parse_query_params(&req)?;
    
    // Validate required parameters
    let lat = params.iter()
        .find(|(k, _)| k == "lat")
        .ok_or_else(|| NasaApiError::BadRequest("Missing required parameter: lat".to_string()))?
        .1.clone();
    
    let lon = params.iter()
        .find(|(k, _)| k == "lon")
        .ok_or_else(|| NasaApiError::BadRequest("Missing required parameter: lon".to_string()))?
        .1.clone();
    
    let _date = params.iter()
        .find(|(k, _)| k == "date")
        .ok_or_else(|| NasaApiError::BadRequest("Missing required parameter: date".to_string()))?
        .1.clone();
    
    // Validate lat/lon ranges
    let lat_f: f64 = lat.parse()
        .map_err(|_| NasaApiError::BadRequest("Invalid latitude format".to_string()))?;
    let lon_f: f64 = lon.parse()
        .map_err(|_| NasaApiError::BadRequest("Invalid longitude format".to_string()))?;
    
    if lat_f < -90.0 || lat_f > 90.0 {
        return Err(NasaApiError::BadRequest("Latitude must be between -90 and 90".to_string()).into());
    }
    
    if lon_f < -180.0 || lon_f > 180.0 {
        return Err(NasaApiError::BadRequest("Longitude must be between -180 and 180".to_string()).into());
    }
    
    // Check cache
    let cache_key = get_cache_key("earth/assets", &params);
    let cache_manager = CacheManager::new(env)?;
    
    if let Some(cached) = cache_manager.get(&cache_key).await? {
        let mut response = Response::from_json(&cached.data)?;
        response.headers_mut().set("X-Cache-Status", "HIT")?;
        return Ok(response);
    }
    
    // Build NASA API URL
    let mut url = "https://api.nasa.gov/planetary/earth/assets".to_string();
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
    
    // Cache the response for 1 day
    cache_manager.set(&cache_key, json_value.clone(), 1440).await?;
    
    let mut response = Response::from_json(&json_value)?;
    response.headers_mut().set("X-Cache-Status", "MISS")?;
    Ok(response)
}