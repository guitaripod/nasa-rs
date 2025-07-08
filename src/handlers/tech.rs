use worker::{Request, Response, RouteContext};
use crate::error::Result;
use crate::cache::{CacheManager, get_cache_key, get_ttl_for_endpoint};
use crate::utils;
use super::make_nasa_request;
use super::HandlerContext;

macro_rules! tech_handler {
    ($fn_name:ident, $endpoint:expr) => {
        pub async fn $fn_name(req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
            let (env, _) = &ctx.data;
            let params = utils::parse_query_params(&req)?;
            
            // Check cache
            let cache_key = get_cache_key(&format!("techtransfer/{}", $endpoint), &params);
            let cache_manager = CacheManager::new(env)?;
            
            if let Some(cached) = cache_manager.get(&cache_key).await? {
                let mut response = Response::from_json(&cached.data)?;
                response.headers_mut().set("X-Cache-Status", "HIT")?;
                return Ok(response);
            }
            
            // Build NASA API URL
            let mut url = format!("https://api.nasa.gov/techtransfer/{}/", $endpoint);
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
            
            // Cache for 1 week (tech transfer data doesn't change frequently)
            let ttl = get_ttl_for_endpoint("techtransfer");
            cache_manager.set(&cache_key, json_value.clone(), ttl).await?;
            
            let mut response = Response::from_json(&json_value)?;
            response.headers_mut().set("X-Cache-Status", "MISS")?;
            Ok(response)
        }
    };
}

// Generate handlers for each TechTransfer endpoint
tech_handler!(get_patents, "patent");
tech_handler!(get_patents_issued, "patent_issued");
tech_handler!(get_software, "software");
tech_handler!(get_spinoffs, "spinoff");