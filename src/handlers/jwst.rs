//! JWST imagery, enriched.
//!
//! The upstream feed (jwst-provider → jwstapi.com CDN) returns raw pipeline products keyed by
//! machine ids. This handler builds the full catalog, decodes human-readable metadata
//! (target / instrument / wavelength / product type) server-side, and surfaces the combined
//! mosaics ahead of the raw single-detector frames — so clients get nicer images first and rich
//! captions without their own decoder. The whole catalog is cached (and served stale on upstream
//! failure); requests are paged from it.

use std::collections::HashSet;
use std::time::Duration;
use worker::{Delay, Env, Fetcher, Request, Response, RouteContext};
use serde::{Deserialize, Serialize};
use crate::cache::CacheManager;
use crate::utils;
use super::HandlerContext;

const JWST_UPSTREAM: &str = "https://jwst-provider.guitaripod.workers.dev/large-images";
const CATALOG_KEY: &str = "jwst:catalog:v1";
// JWST imagery is essentially static, and rebuilding the catalog is a ~50s fan-out — so keep it
// fresh for days and rely on the cron to refresh it, so no user ever pays the rebuild cost.
const CATALOG_TTL_MIN: i64 = 4320; // 3 days
const MAX_PAGES: u32 = 40;

#[derive(Deserialize)]
struct RawItem {
    url: String,
    observation_id: String,
    program: i64,
    file_type: String,
    suffix: String,
    size: i64,
}

#[derive(Serialize, Deserialize, Clone)]
struct EnrichedItem {
    url: String,
    observation_id: String,
    program: i64,
    file_type: String,
    suffix: String,
    size: i64,
    target: String,
    target_subtitle: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    instrument: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wavelength_microns: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    band: Option<String>,
    product: String,
    is_mosaic: bool,
}

pub async fn get_jwst(req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    let params = utils::parse_query_params(&req)?;
    let page: usize = param(&params, "page").and_then(|v| v.parse().ok()).unwrap_or(1).max(1);
    let per_page: usize = param(&params, "perPage").and_then(|v| v.parse().ok()).unwrap_or(20).clamp(1, 100);

    let cache = CacheManager::new(env)?;

    let (catalog, cache_status) = if let Some(c) = cache.get(CATALOG_KEY).await? {
        (serde_json::from_value(c.data).unwrap_or_default(), "HIT")
    } else {
        match build_catalog(env).await {
            Ok(cat) if !cat.is_empty() => {
                cache.set(CATALOG_KEY, serde_json::to_value(&cat)?, CATALOG_TTL_MIN).await?;
                (cat, "MISS")
            }
            built => {
                // Upstream failed or returned nothing — serve a stale catalog if we have one,
                // otherwise surface the error rather than a misleading empty 200.
                if let Some(stale) = cache.get_stale(CATALOG_KEY).await? {
                    (serde_json::from_value(stale.data).unwrap_or_default(), "STALE")
                } else {
                    match built {
                        Ok(_) => return Response::error("JWST catalog is empty", 502),
                        Err(e) => return Response::error(format!("JWST upstream failed: {e}"), 502),
                    }
                }
            }
        }
    };

    let start = (page - 1) * per_page;
    let slice: Vec<&EnrichedItem> = catalog.iter().skip(start).take(per_page).collect();
    let mut response = Response::from_json(&slice)?;
    response.headers_mut().set("X-Cache-Status", cache_status)?;
    Ok(response)
}

fn param<'a>(params: &'a [(String, String)], name: &str) -> Option<&'a str> {
    params.iter().find(|(k, _)| k == name).map(|(_, v)| v.as_str())
}

/// Rebuilds and caches the full enriched catalog. Called from the cron so the expensive fan-out
/// happens off the request path and the catalog is always warm.
pub async fn warm_catalog(env: &Env) -> worker::Result<()> {
    let catalog = build_catalog(env).await?;
    if !catalog.is_empty() {
        CacheManager::new(env)?
            .set(CATALOG_KEY, serde_json::to_value(&catalog)?, CATALOG_TTL_MIN)
            .await?;
        worker::console_log!("JWST catalog warmed: {} items", catalog.len());
    }
    Ok(())
}

async fn build_catalog(env: &Env) -> worker::Result<Vec<EnrichedItem>> {
    let fetcher = env.service("JWST_PROVIDER")?;
    let mut all: Vec<EnrichedItem> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    for page in 1..=MAX_PAGES {
        let url = format!("{JWST_UPSTREAM}?page={page}&perPage=50");
        let body = fetch_via_service(&fetcher, &url).await?;
        let raw: Vec<RawItem> = serde_json::from_str(&body)
            .map_err(|e| worker::Error::RustError(format!("JWST decode failed (page {page}, body starts: {}): {e}", body.chars().take(120).collect::<String>())))?;
        if raw.is_empty() {
            break;
        }
        for r in raw {
            if seen.insert(r.url.clone()) {
                all.push(enrich(r));
            }
        }
    }
    // Combined mosaics first, then by observation id for stable ordering.
    all.sort_by(|a, b| product_rank(&a.product).cmp(&product_rank(&b.product)).then_with(|| a.observation_id.cmp(&b.observation_id)));
    Ok(all)
}

/// Fetches a page through the jwst-provider service binding, retrying transient failures.
async fn fetch_via_service(fetcher: &Fetcher, url: &str) -> worker::Result<String> {
    let mut last_err = String::from("unknown error");
    for attempt in 1..=3u64 {
        match fetcher.fetch(url, None).await {
            Ok(mut resp) => {
                let status = resp.status_code();
                if (200..300).contains(&status) {
                    return resp.text().await;
                }
                last_err = format!("status {status}");
            }
            Err(e) => last_err = format!("{e}"),
        }
        if attempt < 3 {
            Delay::from(Duration::from_millis(200 * attempt)).await;
        }
    }
    worker::console_error!("JWST service fetch failed for {url}: {last_err}");
    Err(worker::Error::RustError(format!("JWST upstream failed: {last_err}")))
}

fn product_rank(product: &str) -> u8 {
    match product {
        "Combined mosaic" => 0,
        "Single-detector frame" => 1,
        "Spectrum" => 2,
        _ => 3,
    }
}

fn enrich(raw: RawItem) -> EnrichedItem {
    let tokens: Vec<String> = raw
        .observation_id
        .to_lowercase()
        .split(|c| c == '_' || c == '-')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    let (target, target_subtitle) = target_for_program(raw.program);
    let filter = filter_token(&tokens);
    let wavelength_microns = filter.as_deref().and_then(filter_wavelength);
    let band = band_for(filter.as_deref(), wavelength_microns);
    let instrument = instrument_for(&tokens, filter.as_deref());
    let product = product_for_suffix(&raw.suffix);
    let is_mosaic = product == "Combined mosaic";

    EnrichedItem {
        url: raw.url,
        observation_id: raw.observation_id,
        program: raw.program,
        file_type: raw.file_type,
        suffix: raw.suffix,
        size: raw.size,
        target,
        target_subtitle,
        instrument,
        filter: filter.map(|f| f.to_uppercase()),
        wavelength_microns,
        band: band.map(|b| b.to_string()),
        product: product.to_string(),
        is_mosaic,
    }
}

/// Curated JWST program → (headline, subtitle). The dataset is dominated by 2731/2732/2736; the
/// rest are the other Early Release Observations so adjacent programs decode gracefully too.
fn target_for_program(program: i64) -> (String, String) {
    let known = match program {
        2731 => Some(("Cosmic Cliffs", "NGC 3324 · Carina complex")),
        2732 => Some(("Stephan's Quintet", "HCG 92 · compact galaxy group")),
        2733 => Some(("Southern Ring Nebula", "NGC 3132 · planetary nebula")),
        2734 => Some(("WASP-96 b", "Exoplanet atmosphere")),
        2736 => Some(("Webb's First Deep Field", "SMACS 0723 · galaxy cluster")),
        _ => None,
    };
    match known {
        Some((t, s)) => (t.to_string(), s.to_string()),
        None => ("Webb Observation".to_string(), format!("Program {program}")),
    }
}

fn instrument_for(tokens: &[String], filter: Option<&str>) -> Option<String> {
    for t in tokens {
        if t.starts_with("nrc") || t == "nircam" {
            return Some("NIRCam".to_string());
        }
        if t.starts_with("nrs") || t == "nirspec" {
            return Some("NIRSpec".to_string());
        }
        if t.starts_with("miri") || t == "mirimage" {
            return Some("MIRI".to_string());
        }
        if t == "nis" || t == "niriss" {
            return Some("NIRISS".to_string());
        }
        if t.starts_with("guider") || t == "fgs" {
            return Some("FGS".to_string());
        }
    }
    // Stage-3 filter-only names: 4-digit token => mid-IR (MIRI), 3-digit => near-IR (NIRCam).
    filter.and_then(filter_digits).map(|d| {
        if d.len() >= 4 { "MIRI".to_string() } else { "NIRCam".to_string() }
    })
}

fn filter_token(tokens: &[String]) -> Option<String> {
    tokens.iter().find(|t| {
        let bytes = t.as_bytes();
        if bytes.first() != Some(&b'f') || t.len() < 4 {
            return false;
        }
        let rest = &t[1..];
        match rest.chars().last() {
            Some(last) if "wmnc".contains(last) => rest[..rest.len() - 1].chars().all(|c| c.is_ascii_digit()),
            _ => false,
        }
    }).cloned()
}

fn filter_digits(filter: &str) -> Option<String> {
    let lower = filter.to_lowercase();
    if !lower.starts_with('f') {
        return None;
    }
    let digits: String = lower[1..].chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() { None } else { Some(digits) }
}

fn filter_wavelength(filter: &str) -> Option<f64> {
    filter_digits(filter).and_then(|d| d.parse::<f64>().ok()).map(|v| v / 100.0)
}

fn band_for(filter: Option<&str>, wavelength: Option<f64>) -> Option<&'static str> {
    if let Some(d) = filter.and_then(filter_digits) {
        return Some(if d.len() >= 4 { "Mid-infrared" } else { "Near-infrared" });
    }
    wavelength.map(|w| if w >= 5.0 { "Mid-infrared" } else { "Near-infrared" })
}

fn product_for_suffix(suffix: &str) -> &'static str {
    match suffix.trim_matches('_').to_lowercase().as_str() {
        "i2d" | "s2d" => "Combined mosaic",
        "cal" | "crf" | "crfints" => "Single-detector frame",
        "x1d" | "x1dints" => "Spectrum",
        "rate" | "rateints" | "uncal" => "Raw frame",
        _ => "Single-detector frame",
    }
}
