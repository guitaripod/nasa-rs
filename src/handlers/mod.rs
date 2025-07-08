pub mod apod;
pub mod donki;
pub mod earth;
pub mod epic;
pub mod exoplanets;
pub mod mars;
pub mod media;
pub mod neo;
pub mod ssd;
pub mod tech;

// Common handler utilities
use worker::{Response, RouteContext, Env, Context};
use crate::error::{NasaApiError, Result};
use crate::utils;

pub type HandlerContext = (Env, Context);

pub async fn make_nasa_request(
    url: &str,
    ctx: &RouteContext<HandlerContext>,
) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    let api_key = utils::get_api_key(env)?;
    
    let full_url = if url.contains('?') {
        format!("{}&api_key={}", url, api_key)
    } else {
        format!("{}?api_key={}", url, api_key)
    };
    
    let response = reqwest::get(&full_url)
        .await
        .map_err(|e| worker::Error::RustError(format!("Request failed: {}", e)))?;
    
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return worker::Response::error(
            format!("NASA API returned {} - {}", status, error_text),
            status.as_u16()
        );
    }
    
    let body = response.text().await
        .map_err(|e| worker::Error::RustError(format!("Failed to read response: {}", e)))?;
    
    Response::ok(body)
}