//! Webb telescope live status + curated gallery.
//!
//! Two consumer-facing features the raw pipeline catalog ([`super::jwst`]) can't provide:
//!
//! * **Status** — the official "Where Is Webb" flight state (instrument + sunshield temperatures,
//!   mission age, deployment phase), reshaped into a clean payload the apps render as a live
//!   telescope dashboard.
//! * **Gallery** — the curated, captioned press imagery from the ESA/Webb archive (Djangoplicity,
//!   AVM-standard), grouped into themed collections with real multi-resolution asset URLs, credit
//!   lines (CC BY 4.0), sky coordinates, distances and instruments — a magazine-quality gallery
//!   instead of machine-named FITS products.
//!
//! Both are KV-cached (and served stale on upstream failure); the gallery is rebuilt off the
//! request path by the daily cron so no user ever pays its fan-out cost.

use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use worker::{Request, Response, RouteContext};
use chrono::{TimeZone, Utc};
use crate::cache::CacheManager;
use crate::utils;
use super::{fetch_text_retrying, HandlerContext};

// ---------------------------------------------------------------------------
// Status — "Where Is Webb"
// ---------------------------------------------------------------------------

const STATUS_UPSTREAM: &str = "https://jwst.nasa.gov/content/webbLaunch/flightCurrentState2.0.json";
const STATUS_KEY: &str = "jwst:status:v1";
const STATUS_TTL_MIN: i64 = 180; // temps drift slowly; refresh a few times a day

#[derive(Deserialize)]
struct FlightState {
    #[serde(rename = "currentState")]
    current_state: CurrentState,
}

#[derive(Deserialize)]
struct CurrentState {
    #[serde(rename = "launchDateTimeString")]
    launch: Option<String>,
    #[serde(rename = "currentDeployTableIndex")]
    deploy_index: Option<i64>,
    #[serde(rename = "tempWarmSide1C")]
    warm1: Option<f64>,
    #[serde(rename = "tempWarmSide2C")]
    warm2: Option<f64>,
    #[serde(rename = "tempCoolSide1C")]
    cool1: Option<f64>,
    #[serde(rename = "tempCoolSide2C")]
    cool2: Option<f64>,
    #[serde(rename = "tempInstNirCamK")]
    nircam: Option<f64>,
    #[serde(rename = "tempInstNirSpecK")]
    nirspec: Option<f64>,
    #[serde(rename = "tempInstFgsNirissK")]
    fgs_niriss: Option<f64>,
    #[serde(rename = "tempInstMiriK")]
    miri: Option<f64>,
    #[serde(rename = "tempInstFsmK")]
    fsm: Option<f64>,
    #[serde(rename = "tempsShow")]
    temps_show: Option<bool>,
}

pub async fn get_status(_req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    let cache = CacheManager::new(env)?;

    if let Some(cached) = cache.get(STATUS_KEY).await? {
        return with_status(Response::from_json(&cached.data)?, "HIT");
    }

    match fetch_text_retrying(STATUS_UPSTREAM).await {
        Ok(body) => match serde_json::from_str::<FlightState>(&body) {
            Ok(state) => {
                let payload = build_status(&state.current_state);
                cache.set(STATUS_KEY, payload.clone(), STATUS_TTL_MIN).await?;
                with_status(Response::from_json(&payload)?, "MISS")
            }
            Err(e) => stale_or_err(&cache, STATUS_KEY, format!("status decode failed: {e}")).await,
        },
        Err(e) => stale_or_err(&cache, STATUS_KEY, format!("status upstream failed: {e}")).await,
    }
}

/// JWST lifted off 2021-12-25 12:20 UTC — a fixed historical instant, so mission age is computed
/// from it directly rather than parsing the upstream's seconds-less timestamp.
fn build_status(state: &CurrentState) -> Value {
    let launch = Utc.with_ymd_and_hms(2021, 12, 25, 12, 20, 0).single();
    let days_since_launch = launch.map(|l| (Utc::now() - l).num_days().max(0)).unwrap_or(0);
    let years_since_launch = (days_since_launch as f64 / 365.25 * 10.0).round() / 10.0;

    let instruments: Vec<Value> = [
        ("NIRCam", state.nircam),
        ("NIRSpec", state.nirspec),
        ("NIRISS / FGS", state.fgs_niriss),
        ("MIRI", state.miri),
        ("Fine Steering Mirror", state.fsm),
    ]
    .into_iter()
    .filter_map(|(name, k)| k.map(|kelvin| json!({ "name": name, "kelvin": kelvin })))
    .collect();

    let warm: Vec<f64> = [state.warm1, state.warm2].into_iter().flatten().collect();
    let cool: Vec<f64> = [state.cool1, state.cool2].into_iter().flatten().collect();

    json!({
        "launch_iso": state.launch.clone().unwrap_or_else(|| "2021-12-25T12:20Z".to_string()),
        "days_since_launch": days_since_launch,
        "years_since_launch": years_since_launch,
        "phase": phase_for(days_since_launch),
        "deploy_index": state.deploy_index,
        "fully_deployed": state.deploy_index.map(|i| i >= 40).unwrap_or(true),
        "temps_show": state.temps_show.unwrap_or(true),
        "warm_side_c": warm,
        "cool_side_c": cool,
        "instruments_k": instruments,
        "orbit": "Sun–Earth Lagrange Point 2 (L2)",
        "distance_km_approx": 1_500_000,
        "mirror_segments": 18,
        "mirror_diameter_m": 6.5,
        "source": "NASA / STScI — Where Is Webb"
    })
}

/// Webb finished commissioning and began routine science on 2022-07-12 (~199 days after launch);
/// every later date is steady-state science operations.
fn phase_for(days_since_launch: i64) -> &'static str {
    if days_since_launch >= 199 {
        "Science operations"
    } else if days_since_launch >= 30 {
        "Commissioning"
    } else {
        "Deployment & cooldown"
    }
}

// ---------------------------------------------------------------------------
// Gallery — curated ESA/Webb imagery, grouped by collection
// ---------------------------------------------------------------------------

const GALLERY_KEY: &str = "jwst:gallery:v2";
const GALLERY_TTL_MIN: i64 = 4320; // 3 days; cron-warmed
const ESA_CDN: &str = "https://cdn.esawebb.org/archives/images";
/// Per-collection cap keeps the catalog curated and the cron fan-out small while giving the free
/// tier plenty to browse and Premium "hundreds more".
const PER_COLLECTION: usize = 60;

/// A themed shelf, backed by an ESA/Webb archive category (the category endpoint returns ~100 AVM
/// records scoped to that subject). Hardware/graphics categories are deliberately excluded.
struct Collection {
    slug: &'static str,
    title: &'static str,
    subtitle: &'static str,
}

/// Curated shelves in display order — the first images lead, then the crowd-pleasers.
const COLLECTIONS: &[Collection] = &[
    Collection { slug: "firstimages", title: "First Images", subtitle: "The images that introduced Webb to the world" },
    Collection { slug: "nebulae", title: "Nebulae", subtitle: "Where stars are born and die" },
    Collection { slug: "galaxies", title: "Galaxies", subtitle: "Island universes across cosmic time" },
    Collection { slug: "stars", title: "Stars & Star Birth", subtitle: "Stellar nurseries, jets, and dying stars" },
    Collection { slug: "solarsystem", title: "Our Solar System", subtitle: "Webb turns its mirror on the neighbours" },
    Collection { slug: "quasarsblackholes", title: "Deep & Distant", subtitle: "Quasars, black holes, and the early universe" },
    Collection { slug: "pictureofthemonth", title: "Picture of the Month", subtitle: "Webb's curated monthly showcase" },
];

#[derive(Serialize, Deserialize, Clone, Default)]
struct GalleryItem {
    id: String,
    esa_id: String,
    title: String,
    caption: String,
    credit: String,
    date: String,
    collection: String,
    collection_title: String,
    objects: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    distance_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ra: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dec: Option<f64>,
    instruments: Vec<String>,
    reference_url: String,
    thumb_url: String,
    image_url: String,
    full_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    wallpaper_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wallpaper_portrait_url: Option<String>,
}

pub async fn get_gallery(req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    let params = utils::parse_query_params(&req)?;
    let want = param(&params, "collection");
    let page: usize = param(&params, "page").and_then(|v| v.parse().ok()).unwrap_or(1).max(1);
    let per_page: usize = param(&params, "perPage").and_then(|v| v.parse().ok()).unwrap_or(30).clamp(1, 100);

    let cache = CacheManager::new(env)?;
    let (catalog, status) = load_catalog(&cache).await?;
    let Some(catalog) = catalog else {
        return Response::error("Webb gallery unavailable", 502);
    };

    let filtered: Vec<&GalleryItem> = catalog
        .iter()
        .filter(|it| want.is_none_or(|w| it.collection == w))
        .collect();
    let start = (page - 1) * per_page;
    let slice: Vec<&&GalleryItem> = filtered.iter().skip(start).take(per_page).collect();
    with_status(Response::from_json(&slice)?, status)
}

/// Full-text search across the curated catalog (title / caption / objects / collection). Keeps the
/// gallery self-contained — no dependency on a separate, lower-quality search index.
pub async fn get_search(req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    let params = utils::parse_query_params(&req)?;
    let q = param(&params, "q").unwrap_or("").trim().to_lowercase();
    if q.is_empty() {
        return Response::error("missing q", 400);
    }
    let page: usize = param(&params, "page").and_then(|v| v.parse().ok()).unwrap_or(1).max(1);
    let per_page: usize = param(&params, "perPage").and_then(|v| v.parse().ok()).unwrap_or(30).clamp(1, 100);

    let cache = CacheManager::new(env)?;
    let (catalog, status) = load_catalog(&cache).await?;
    let Some(catalog) = catalog else {
        return Response::error("Webb gallery unavailable", 502);
    };

    let terms: Vec<&str> = q.split_whitespace().collect();
    let hits: Vec<&GalleryItem> = catalog
        .iter()
        .filter(|it| {
            let hay = format!(
                "{} {} {} {}",
                it.title.to_lowercase(),
                it.caption.to_lowercase(),
                it.objects.join(" ").to_lowercase(),
                it.collection_title.to_lowercase()
            );
            terms.iter().all(|t| hay.contains(t))
        })
        .collect();
    let start = (page - 1) * per_page;
    let slice: Vec<&&GalleryItem> = hits.iter().skip(start).take(per_page).collect();
    with_status(Response::from_json(&slice)?, status)
}

/// Returns the collection manifest (slugs + titles + counts) so clients can render shelves and a
/// section picker without downloading the whole catalog first.
pub async fn get_collections(_req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    let cache = CacheManager::new(env)?;
    let (catalog, status) = load_catalog(&cache).await?;
    let catalog = catalog.unwrap_or_default();

    let manifest: Vec<Value> = COLLECTIONS
        .iter()
        .map(|c| {
            let count = catalog.iter().filter(|it| it.collection == c.slug).count();
            json!({ "slug": c.slug, "title": c.title, "subtitle": c.subtitle, "count": count })
        })
        .filter(|v| v["count"].as_u64().unwrap_or(0) > 0)
        .collect();
    with_status(Response::from_json(&manifest)?, status)
}

/// Cron entry point: rebuild + cache the gallery so the per-category fan-out never lands on a user.
pub async fn warm_gallery(env: &worker::Env) -> worker::Result<()> {
    let catalog = build_gallery().await?;
    if !catalog.is_empty() {
        CacheManager::new(env)?
            .set(GALLERY_KEY, serde_json::to_value(&catalog)?, GALLERY_TTL_MIN)
            .await?;
        worker::console_log!("Webb gallery warmed: {} items", catalog.len());
    }
    Ok(())
}

/// Loads the catalog: fresh cache → live rebuild → stale cache. Returns `(catalog, cache_status)`;
/// `None` catalog means even the stale fallback was empty.
async fn load_catalog(cache: &CacheManager) -> worker::Result<(Option<Vec<GalleryItem>>, &'static str)> {
    if let Some(c) = cache.get(GALLERY_KEY).await? {
        return Ok((Some(serde_json::from_value(c.data).unwrap_or_default()), "HIT"));
    }
    match build_gallery().await {
        Ok(cat) if !cat.is_empty() => {
            cache.set(GALLERY_KEY, serde_json::to_value(&cat)?, GALLERY_TTL_MIN).await?;
            Ok((Some(cat), "MISS"))
        }
        _ => {
            if let Some(stale) = cache.get_stale(GALLERY_KEY).await? {
                Ok((Some(serde_json::from_value(stale.data).unwrap_or_default()), "STALE"))
            } else {
                Ok((None, "MISS"))
            }
        }
    }
}

async fn build_gallery() -> worker::Result<Vec<GalleryItem>> {
    let mut all: Vec<GalleryItem> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    for collection in COLLECTIONS {
        let url = format!("https://esawebb.org/images/archive/category/{}/json/", collection.slug);
        let body = match fetch_text_retrying(&url).await {
            Ok(b) => b,
            Err(e) => {
                worker::console_error!("Webb gallery fetch failed ({}): {e}", collection.slug);
                continue;
            }
        };
        let records: Vec<Value> = match serde_json::from_str(&body) {
            Ok(r) => r,
            Err(e) => {
                worker::console_error!("Webb gallery decode failed ({}): {e}", collection.slug);
                continue;
            }
        };

        let mut added = 0usize;
        for rec in &records {
            if added >= PER_COLLECTION {
                break;
            }
            let Some(item) = map_record(rec, collection) else { continue };
            if !seen.insert(item.esa_id.clone()) {
                continue;
            }
            all.push(item);
            added += 1;
        }
    }

    Ok(all)
}

/// Maps one AVM record to a [`GalleryItem`], decoding the feed's Python-`repr` text fields and
/// resolving multi-resolution asset URLs. Returns `None` when the record lacks an id or any usable
/// image — both load-bearing.
fn map_record(rec: &Value, collection: &Collection) -> Option<GalleryItem> {
    let esa_id = rec.get("ID").and_then(Value::as_str)?.to_string();
    if esa_id.is_empty() {
        return None;
    }

    let assets = resolve_assets(rec, &esa_id);
    if assets.image.is_empty() {
        return None;
    }

    let title = decode_text(rec.get("Title"));
    let title = if title.is_empty() { "Webb Image".to_string() } else { title };

    Some(GalleryItem {
        id: esa_id.clone(),
        title,
        caption: clean_caption(&decode_text(rec.get("Description"))),
        credit: credit_for(&decode_text(rec.get("Credit"))),
        date: rec.get("Date").and_then(Value::as_str).unwrap_or("").chars().take(10).collect(),
        collection: collection.slug.to_string(),
        collection_title: collection.title.to_string(),
        objects: string_list(rec.get("Subject.Name")),
        distance_text: distance_text(rec),
        ra: coord(rec.get("Spatial.ReferenceValue"), 0),
        dec: coord(rec.get("Spatial.ReferenceValue"), 1),
        instruments: unique_instruments(rec.get("Instrument")),
        reference_url: rec.get("ReferenceURL").and_then(Value::as_str).unwrap_or("").to_string(),
        thumb_url: assets.thumb,
        image_url: assets.image,
        full_url: assets.full,
        wallpaper_url: assets.wallpaper,
        wallpaper_portrait_url: assets.wallpaper_portrait,
        esa_id,
    })
}

struct Assets {
    thumb: String,
    image: String,
    full: String,
    wallpaper: Option<String>,
    wallpaper_portrait: Option<String>,
}

/// Picks grid / display / full / wallpaper renditions from the record's `formats_url` map, falling
/// back to the deterministic CDN path when a rendition is missing. ESA exposes true UHD/QHD desktop
/// and mobile wallpapers — the load-bearing assets for the Premium download/wallpaper feature.
fn resolve_assets(rec: &Value, esa_id: &str) -> Assets {
    let formats = rec.get("formats_url").and_then(Value::as_object);
    let from_formats = |keys: &[&str]| -> Option<String> {
        let map = formats?;
        keys.iter().find_map(|k| map.get(*k).and_then(Value::as_str).map(https))
    };
    let derived = |fmt: &str| format!("{ESA_CDN}/{fmt}/{esa_id}.jpg");

    Assets {
        thumb: from_formats(&["newsfeature", "medium", "screen640", "screen"]).unwrap_or_else(|| derived("newsfeature")),
        image: from_formats(&["screen", "large", "medium"]).unwrap_or_else(|| derived("screen")),
        full: from_formats(&["publicationjpg", "large", "screen"]).unwrap_or_else(|| derived("large")),
        wallpaper: from_formats(&["wallpaper_uhd", "wallpaper_qhd", "wallpaper4", "wallpaper_fhd", "wallpaper3", "wallpaper2", "wallpaper1"]),
        wallpaper_portrait: from_formats(&["wallpaper_mobile_uhd", "wallpaper_mobile_qhd_plus", "wallpaper_mobile_fhd_plus", "wallpaper_mobile_fhd", "portrait1080"]),
    }
}

// ---------------------------------------------------------------------------
// AVM field decoding
// ---------------------------------------------------------------------------

/// The ESA feed serialises scalar text via Python `repr`, so values arrive as byte-string literals
/// like `b'Webb\xe2\x80\x99s view'`. Strip the wrapper and decode the `\xNN` / `\n` / `\'` escapes
/// back into UTF-8.
fn decode_text(v: Option<&Value>) -> String {
    let Some(s) = v.and_then(Value::as_str) else { return String::new() };
    decode_py_bytes(s)
}

fn decode_py_bytes(s: &str) -> String {
    let t = s.trim();
    let wrapped = t.len() >= 3
        && ((t.starts_with("b'") && t.ends_with('\'')) || (t.starts_with("b\"") && t.ends_with('"')));
    let inner = if wrapped { &t[2..t.len() - 1] } else { t };
    let bytes = inner.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\' && i + 1 < bytes.len() {
            match bytes[i + 1] {
                b'x' if i + 3 < bytes.len() => {
                    let hi = (bytes[i + 2] as char).to_digit(16);
                    let lo = (bytes[i + 3] as char).to_digit(16);
                    if let (Some(h), Some(l)) = (hi, lo) {
                        out.push((h * 16 + l) as u8);
                        i += 4;
                        continue;
                    }
                    out.push(bytes[i]);
                    i += 1;
                }
                b'n' => { out.push(b'\n'); i += 2; }
                b't' => { out.push(b' '); i += 2; }
                b'r' => { i += 2; }
                b'\'' => { out.push(b'\''); i += 2; }
                b'"' => { out.push(b'"'); i += 2; }
                b'\\' => { out.push(b'\\'); i += 2; }
                _ => { out.push(bytes[i]); i += 1; }
            }
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).trim().to_string()
}

/// AVM list fields (`Subject.Name`, `Instrument`, …) arrive as clean JSON arrays of strings.
fn string_list(v: Option<&Value>) -> Vec<String> {
    v.and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(Value::as_str)
                .map(decode_py_bytes)
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

fn unique_instruments(v: Option<&Value>) -> Vec<String> {
    let mut seen = HashSet::new();
    string_list(v)
        .into_iter()
        .filter(|s| {
            let known = ["NIRCam", "MIRI", "NIRSpec", "NIRISS", "FGS"];
            known.iter().any(|k| s.eq_ignore_ascii_case(k))
        })
        .filter(|s| seen.insert(s.to_uppercase()))
        .collect()
}

fn coord(v: Option<&Value>, index: usize) -> Option<f64> {
    v.and_then(Value::as_array)?
        .get(index)
        .and_then(Value::as_str)
        .and_then(|s| s.parse::<f64>().ok())
}

/// Prefers a clean "6,500 light-years" from the numeric `Distance[0]`, falling back to the free-text
/// `Distance.Notes` (e.g. "2,000 parsecs").
fn distance_text(rec: &Value) -> Option<String> {
    if let Some(ly) = rec
        .get("Distance")
        .and_then(Value::as_array)
        .and_then(|a| a.first())
        .and_then(Value::as_str)
        .and_then(|s| s.parse::<f64>().ok())
    {
        if ly > 0.0 {
            return Some(format!("{} light-years", thousands(ly)));
        }
    }
    let notes = decode_text(rec.get("Distance.Notes"));
    if notes.is_empty() {
        None
    } else {
        Some(notes)
    }
}

fn thousands(value: f64) -> String {
    let n = value.round() as i64;
    let s = n.abs().to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    if n < 0 {
        format!("-{out}")
    } else {
        out
    }
}

fn https(url: &str) -> String {
    if let Some(rest) = url.strip_prefix("http://") {
        format!("https://{rest}")
    } else {
        url.to_string()
    }
}

/// ESA captions are clean prose; just trim trailing whitespace and cap the length so cells/sheets
/// stay sane. Cuts on a sentence boundary when one is near the limit.
fn clean_caption(caption: &str) -> String {
    let trimmed = caption.trim();
    if trimmed.len() <= 1000 {
        return trimmed.to_string();
    }
    let mut cut = 1000;
    while cut > 0 && !trimmed.is_char_boundary(cut) {
        cut -= 1;
    }
    let head = &trimmed[..cut];
    let end = head.rfind(". ").map(|i| i + 1).unwrap_or(cut);
    format!("{}…", trimmed[..end].trim_end())
}

fn credit_for(credit: &str) -> String {
    let c = credit.trim();
    if c.is_empty() {
        "NASA, ESA, CSA, STScI".to_string()
    } else {
        c.to_string()
    }
}

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

fn param<'a>(params: &'a [(String, String)], name: &str) -> Option<&'a str> {
    params.iter().find(|(k, _)| k == name).map(|(_, v)| v.as_str())
}

fn with_status(mut response: Response, status: &str) -> worker::Result<Response> {
    response.headers_mut().set("X-Cache-Status", status)?;
    Ok(response)
}

async fn stale_or_err(cache: &CacheManager, key: &str, err: String) -> worker::Result<Response> {
    if let Some(stale) = cache.get_stale(key).await? {
        return with_status(Response::from_json(&stale.data)?, "STALE");
    }
    Response::error(err, 502)
}
