use worker::{Request, Response, RouteContext};
use crate::error::Result;
use crate::cache::{CacheManager, get_cache_key, get_ttl_for_endpoint};
use crate::utils;
use super::{make_nasa_request, HandlerContext};

pub async fn get_apod(req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    let params = utils::parse_query_params(&req)?;
    
    // Check cache
    let cache_key = get_cache_key("apod", &params);
    let cache_manager = CacheManager::new(env)?;
    
    if let Some(cached) = cache_manager.get(&cache_key).await? {
        let mut response = Response::from_json(&cached.data)?;
        response.headers_mut().set("X-Cache-Status", "HIT")?;
        return Ok(response);
    }
    
    // Build NASA API URL
    let mut url = "https://api.nasa.gov/planetary/apod".to_string();
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
    
    // Make request to NASA API
    let mut response = make_nasa_request(&url, &ctx).await?;
    let body = response.text().await?;
    let json_value: serde_json::Value = serde_json::from_str(&body)?;
    
    // Cache the response
    let ttl = get_ttl_for_endpoint("apod");
    cache_manager.set(&cache_key, json_value.clone(), ttl).await?;
    
    let mut response = Response::from_json(&json_value)?;
    response.headers_mut().set("X-Cache-Status", "MISS")?;
    Ok(response)
}