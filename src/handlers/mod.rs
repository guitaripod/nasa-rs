//! Request handlers for various NASA API endpoints.
//! 
//! This module contains all the handler functions that process incoming requests
//! and interact with NASA's various APIs.

/// Astronomy Picture of the Day (APOD) handler.
pub mod apod;
/// API documentation handlers.
pub mod docs;
/// Space Weather Database (DONKI) handlers.
pub mod donki;
/// Earth imagery and assets handlers.
pub mod earth;
/// Earth Polychromatic Imaging Camera (EPIC) handlers.
pub mod epic;
/// Exoplanet archive query handlers.
pub mod exoplanets;
/// JWST imagery (enriched + mosaic-prioritised).
pub mod jwst;
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
/// Webb telescope live status + curated gallery.
pub mod webb;

// Common handler utilities
use worker::{Response, RouteContext, Env, Context, Delay};
use std::time::Duration;
use crate::utils;

/// Type alias for the context passed to all handler functions.
/// Contains the Worker environment and context needed for processing requests.
pub type HandlerContext = (Env, Context);

/// How many times to attempt a NASA request before giving up. NASA's APIs intermittently return
/// 5xx / time out on cold paths, then succeed on a retry — so a couple of retries turns most of
/// those transient failures into successes instead of propagating a 500 to the client.
const MAX_ATTEMPTS: u32 = 3;

/// Fetches a NASA endpoint (adding the API key), retrying transient failures — network errors,
/// 5xx, and 429 — with a short backoff. Returns the response body on success, or an error after
/// exhausting retries so the caller can fall back to a stale cache. A non-429 4xx is a client
/// error and returns immediately without retrying.
pub async fn fetch_nasa_text(
    url: &str,
    ctx: &RouteContext<HandlerContext>,
) -> worker::Result<String> {
    let (env, _) = &ctx.data;
    let api_key = utils::get_api_key(env)?;

    let full_url = if url.contains('?') {
        format!("{url}&api_key={api_key}")
    } else {
        format!("{url}?api_key={api_key}")
    };

    fetch_text_retrying(&full_url).await
}

/// Retrying GET for a fully-formed URL (no API key added) — used for public upstreams like the
/// JWST CDN proxy. Same transient-failure handling as [`fetch_nasa_text`].
pub async fn fetch_text_retrying(full_url: &str) -> worker::Result<String> {
    let mut last_err = String::from("unknown error");
    for attempt in 1..=MAX_ATTEMPTS {
        match reqwest::get(full_url).await {
            Ok(response) => {
                let status = response.status();
                if status.is_success() {
                    return response
                        .text()
                        .await
                        .map_err(|e| worker::Error::RustError(format!("Failed to read NASA response: {e}")));
                }
                if status.is_client_error() && status.as_u16() != 429 {
                    let body = response.text().await.unwrap_or_default();
                    return Err(worker::Error::RustError(format!(
                        "NASA API {status}: {}",
                        truncate(&body, 300)
                    )));
                }
                last_err = format!("NASA API {status}");
            }
            Err(e) => last_err = format!("request failed: {e}"),
        }

        if attempt < MAX_ATTEMPTS {
            Delay::from(Duration::from_millis(250 * attempt as u64)).await;
        }
    }

    Err(worker::Error::RustError(format!(
        "NASA API unavailable after {MAX_ATTEMPTS} attempts: {last_err}"
    )))
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", s.chars().take(max).collect::<String>())
    }
}

/// Makes an authenticated NASA request and returns a ready `Response`. Retries transient upstream
/// failures internally (see [`fetch_nasa_text`]). Handlers that want stale-on-error fallback should
/// call [`fetch_nasa_text`] directly and consult the cache on `Err`.
pub async fn make_nasa_request(
    url: &str,
    ctx: &RouteContext<HandlerContext>,
) -> worker::Result<Response> {
    let body = fetch_nasa_text(url, ctx).await?;
    Response::ok(body)
}