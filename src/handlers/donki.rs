use worker::{Request, Response, RouteContext};
use crate::cache::{CacheManager, get_cache_key};
use crate::utils;
use super::make_nasa_request;
use super::HandlerContext;

macro_rules! donki_handler {
    ($fn_name:ident, $endpoint:expr, $cache_ttl:expr) => {
        pub async fn $fn_name(req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
            let (env, _) = &ctx.data;
            let params = utils::parse_query_params(&req)?;
            
            // Check cache
            let cache_key = get_cache_key($endpoint, &params);
            let cache_manager = CacheManager::new(env)?;
            
            if let Some(cached) = cache_manager.get(&cache_key).await? {
                let mut response = Response::from_json(&cached.data)?;
                response.headers_mut().set("X-Cache-Status", "HIT")?;
                return Ok(response);
            }
            
            // Build NASA API URL
            let mut url = format!("https://api.nasa.gov/DONKI/{}", $endpoint);
            let mut first_param = true;
            
            for (key, value) in &params {
                if key != "api_key" {
                    if first_param {
                        url.push('?');
                        first_param = false;
                    } else {
                        url.push('&');
                    }
                    url.push_str(&format!("{key}={value}"));
                }
            }
            
            let mut response = make_nasa_request(&url, &ctx).await?;
            let body = response.text().await?;
            let json_value: serde_json::Value = serde_json::from_str(&body)?;
            
            // Cache the response
            cache_manager.set(&cache_key, json_value.clone(), $cache_ttl).await?;
            
            let mut response = Response::from_json(&json_value)?;
            response.headers_mut().set("X-Cache-Status", "MISS")?;
            Ok(response)
        }
    };
}

// Generate handlers for each DONKI endpoint
donki_handler!(get_cme, "CME", 30);
donki_handler!(get_cme_analysis, "CMEAnalysis", 30);
donki_handler!(get_gst, "GST", 30);
donki_handler!(get_ips, "IPS", 30);
donki_handler!(get_flr, "FLR", 30);
donki_handler!(get_sep, "SEP", 30);
donki_handler!(get_mpc, "MPC", 30);
donki_handler!(get_rbe, "RBE", 30);
donki_handler!(get_hss, "HSS", 30);
donki_handler!(get_wsa_enlil, "WSAEnlilSimulations", 30);
donki_handler!(get_notifications, "notifications", 15);