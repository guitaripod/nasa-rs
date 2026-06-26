use worker::{Request, Response, RouteContext};
use crate::cache::{CacheManager, get_cache_key, get_ttl_for_endpoint};
use crate::utils;
use super::{fetch_nasa_text, HandlerContext};

pub async fn get_apod(req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    let params = utils::parse_query_params(&req)?;

    let cache_key = get_cache_key("apod", &params);
    let cache_manager = CacheManager::new(env)?;

    // Fresh cache hit — return immediately.
    if let Some(cached) = cache_manager.get(&cache_key).await? {
        let mut response = Response::from_json(&cached.data)?;
        response.headers_mut().set("X-Cache-Status", "HIT")?;
        return Ok(response);
    }

    // Build the NASA URL (api_key is added by fetch_nasa_text); encode values defensively.
    let mut url = "https://api.nasa.gov/planetary/apod".to_string();
    let mut first_param = true;
    for (key, value) in &params {
        if key == "api_key" {
            continue;
        }
        url.push(if first_param { '?' } else { '&' });
        first_param = false;
        url.push_str(&format!("{key}={}", urlencoding::encode(value)));
    }

    match fetch_nasa_text(&url, &ctx).await {
        Ok(body) => {
            let json_value: serde_json::Value = serde_json::from_str(&body)?;
            let ttl = get_ttl_for_endpoint("apod");
            cache_manager.set(&cache_key, json_value.clone(), ttl).await?;
            let mut response = Response::from_json(&json_value)?;
            response.headers_mut().set("X-Cache-Status", "MISS")?;
            Ok(response)
        }
        Err(e) => {
            // Upstream failed after retries — serve stale content instead of a 500 if we have any.
            if let Some(stale) = cache_manager.get_stale(&cache_key).await? {
                let mut response = Response::from_json(&stale.data)?;
                response.headers_mut().set("X-Cache-Status", "STALE")?;
                return Ok(response);
            }
            Err(e)
        }
    }
}
