//! Request handlers for various NASA API endpoints.
//! 
//! This module contains all the handler functions that process incoming requests
//! and interact with NASA's various APIs.

/// Astronomy Picture of the Day (APOD) handler.
pub mod apod;
/// Space Weather Database (DONKI) handlers.
pub mod donki;
/// Earth imagery and assets handlers.
pub mod earth;
/// Earth Polychromatic Imaging Camera (EPIC) handlers.
pub mod epic;
/// Exoplanet archive query handlers.
pub mod exoplanets;
/// Mars rover photos handlers.
pub mod mars;
/// NASA Image and Video Library handlers.
pub mod media;
/// Near Earth Objects (NEO) handlers.
pub mod neo;
/// Solar System Dynamics (SSD/CNEOS) handlers.
pub mod ssd;
/// Technology Transfer handlers.
pub mod tech;

// Common handler utilities
use worker::{Response, RouteContext, Env, Context};
use crate::utils;

/// Type alias for the context passed to all handler functions.
/// Contains the Worker environment and context needed for processing requests.
pub type HandlerContext = (Env, Context);

/// Makes an authenticated request to a NASA API endpoint.
/// 
/// This function handles adding the API key to the request URL and processing
/// the response, including error handling for non-successful status codes.
/// 
/// # Arguments
/// 
/// * `url` - The NASA API endpoint URL (without API key)
/// * `ctx` - The route context containing environment variables
/// 
/// # Returns
/// 
/// A Worker Response containing the API response body or an error
pub async fn make_nasa_request(
    url: &str,
    ctx: &RouteContext<HandlerContext>,
) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    let api_key = utils::get_api_key(env)?;
    
    let full_url = if url.contains('?') {
        format!("{url}&api_key={api_key}")
    } else {
        format!("{url}?api_key={api_key}")
    };
    
    let response = reqwest::get(&full_url)
        .await
        .map_err(|e| worker::Error::RustError(format!("Request failed: {e}")))?;
    
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return worker::Response::error(
            format!("NASA API returned {status} - {error_text}"),
            status.as_u16()
        );
    }
    
    let body = response.text().await
        .map_err(|e| worker::Error::RustError(format!("Failed to read response: {e}")))?;
    
    Response::ok(body)
}