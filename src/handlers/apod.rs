use chrono::{Duration, NaiveDate, Utc};
use futures::future::join_all;
use serde_json::{json, Value};
use worker::{console_error, Request, Response, RouteContext};

use super::{fetch_nasa_text, fetch_text_retrying, HandlerContext};
use crate::apod::entry::Entry;
use crate::apod::query::{self, Query};
use crate::apod::sanity;
use crate::cache::{get_cache_key, CacheManager};
use crate::utils;

/// Cache namespace for APOD answers. Bumped from `apod` when NASA's own API started returning the
/// placeholder, so nothing cached from that era can ever be served.
const CACHE_NAMESPACE: &str = "apod2";

/// The request parameters that change the answer; everything else is ignored when caching.
const KEYED_PARAMS: [&str; 5] = ["date", "start_date", "end_date", "count", "thumbs"];

/// Why an APOD request produced no answer.
enum Failure {
    /// The archive has nothing for what was asked; the caller should not retry.
    NotFound(String),
    /// Every source failed or returned unusable data; the caller may retry later.
    Upstream(String),
}

/// A finished answer and the source that produced it.
struct Resolved {
    body: Value,
    source: &'static str,
}

/// `GET /api/apod` — the classic APOD API (`date`, `start_date`/`end_date`, `count`, `thumbs`),
/// answered from NASA's science.nasa.gov publication, falling back to `api.nasa.gov` only for
/// entries that pass validation, and to the last good answer when both are down.
pub async fn get_apod(req: Request, ctx: RouteContext<HandlerContext>) -> worker::Result<Response> {
    let (env, _) = &ctx.data;
    let params = utils::parse_query_params(&req)?;
    let today = Utc::now().date_naive();

    let query = match Query::parse(&params, today) {
        Ok(query) => query,
        Err(message) => return failure_response(400, &message),
    };
    let thumbs = flag(&params, "thumbs");
    let ttl = query.ttl_minutes(today);
    let key = cache_key(&params);
    let cache = CacheManager::new(env)?;

    if ttl > 0 {
        if let Some(cached) = cache.get(&key).await? {
            return answer(&cached.data, "HIT", "cache");
        }
    }

    match resolve(&query, thumbs, &params, &ctx).await {
        Ok(Resolved { body, source }) => {
            if ttl > 0 {
                if let Err(e) = cache.set(&key, body.clone(), ttl).await {
                    console_error!("APOD cache write failed: {e}");
                }
            }
            answer(&body, "MISS", source)
        }
        Err(Failure::NotFound(message)) => failure_response(404, &message),
        Err(Failure::Upstream(message)) => {
            console_error!("APOD unavailable: {message}");
            if let Some(stale) = cache.get_stale(&key).await? {
                return answer(&stale.data, "STALE", "cache");
            }
            failure_response(502, "The Astronomy Picture of the Day is temporarily unavailable.")
        }
    }
}

/// Reads the answer from science.nasa.gov, then from NASA's API.
async fn resolve(
    query: &Query,
    thumbs: bool,
    params: &[(String, String)],
    ctx: &RouteContext<HandlerContext>,
) -> Result<Resolved, Failure> {
    match from_science_site(query, thumbs).await {
        Ok(items) => return shape(query, items, "science.nasa.gov"),
        Err(reason) => console_error!("APOD science.nasa.gov failed: {reason}"),
    }
    let items = from_nasa_api(params, ctx).await.map_err(Failure::Upstream)?;
    shape(query, items, "api.nasa.gov")
}

/// Turns the collected entries into the response body: one object for a single day, a list in
/// ascending date order otherwise, as the classic API did.
fn shape(query: &Query, mut items: Vec<Value>, source: &'static str) -> Result<Resolved, Failure> {
    items.sort_by(|a, b| a["date"].as_str().cmp(&b["date"].as_str()));
    if query.is_single() {
        let newest = items.pop().ok_or_else(|| match query {
            Query::Day(day) => Failure::NotFound(format!("No data available for date {day}")),
            _ => Failure::Upstream("no pictures published".into()),
        })?;
        return Ok(Resolved { body: newest, source });
    }
    Ok(Resolved { body: Value::Array(items), source })
}

/// The pictures for a query from the APOD articles on science.nasa.gov. An empty result is a real
/// answer; an error means the site could not be read or its markup no longer parses.
async fn from_science_site(query: &Query, thumbs: bool) -> Result<Vec<Value>, String> {
    let entries = match query {
        Query::Latest => fetch_page(&query::latest_url()).await?.entries,
        Query::Day(day) => range_entries(*day, *day).await?,
        Query::Range { start, end } => range_entries(*start, *end).await?,
        Query::Random(count) => {
            let days = random_days(*count, Utc::now().date_naive());
            let fetched = join_all(days.into_iter().map(|day| range_entries(day, day))).await;
            fetched.into_iter().collect::<Result<Vec<_>, _>>()?.into_iter().flatten().collect()
        }
    };
    Ok(entries.iter().map(|entry| entry.to_json(thumbs)).filter(sanity::is_usable).collect())
}

/// One page of posts, parsed.
struct Page {
    entries: Vec<Entry>,
}

/// Fetches and parses one WordPress page. Fails when posts came back but none could be read, which
/// means the markup changed and a fallback should take over.
async fn fetch_page(url: &str) -> Result<Page, String> {
    let body = fetch_text_retrying(url).await.map_err(|e| e.to_string())?;
    let posts: Vec<Value> = serde_json::from_str(&body).map_err(|e| format!("unreadable posts: {e}"))?;
    let mut entries = Vec::with_capacity(posts.len());
    for post in &posts {
        match Entry::from_post(post) {
            Ok(entry) => entries.push(entry),
            Err(reason) => console_error!("APOD post {} skipped: {reason:?}", post["slug"].as_str().unwrap_or("?")),
        }
    }
    if !posts.is_empty() && entries.is_empty() {
        return Err(format!("none of {} posts could be read", posts.len()));
    }
    Ok(Page { entries })
}

/// Every entry dated `start..=end`. Every page the range can fill is requested at once.
async fn range_entries(start: NaiveDate, end: NaiveDate) -> Result<Vec<Entry>, String> {
    let pages = (1..=query::pages_for(start, end)).map(|page| fetch_page_owned(query::range_url(start, end, page)));
    let mut found: Vec<Entry> = Vec::new();
    for (index, fetched) in join_all(pages).await.into_iter().enumerate() {
        match fetched {
            Ok(page) => found.extend(page.entries.into_iter().filter(|e| e.date >= start && e.date <= end)),
            Err(reason) if index > 0 && reason.contains("400") => {}
            Err(reason) => return Err(reason),
        }
    }
    found.sort_by_key(|entry| std::cmp::Reverse(entry.date));
    found.dedup_by_key(|entry| entry.date);
    Ok(found)
}

/// `fetch_page` for a URL the future must own, so several can be awaited together.
async fn fetch_page_owned(url: String) -> Result<Page, String> {
    fetch_page(&url).await
}

/// `count` distinct random days of the archive.
fn random_days(count: u32, today: NaiveDate) -> Vec<NaiveDate> {
    let span = (today - query::epoch()).num_days().max(1);
    let mut days: Vec<NaiveDate> = Vec::new();
    while days.len() < count as usize {
        let offset = (worker::js_sys::Math::random() * span as f64) as i64;
        let day = query::epoch() + Duration::days(offset);
        if !days.contains(&day) {
            days.push(day);
        }
    }
    days
}

/// The same request against `api.nasa.gov`, keeping only entries that are real pictures. Fails
/// when it answers with nothing but placeholders, which is how its scraper breaks.
async fn from_nasa_api(
    params: &[(String, String)],
    ctx: &RouteContext<HandlerContext>,
) -> Result<Vec<Value>, String> {
    let mut url = "https://api.nasa.gov/planetary/apod".to_string();
    let mut separator = '?';
    for (key, value) in params.iter().filter(|(key, _)| key != "api_key") {
        url.push(separator);
        separator = '&';
        url.push_str(&format!("{key}={}", urlencoding::encode(value)));
    }
    let body = fetch_nasa_text(&url, ctx).await.map_err(|e| e.to_string())?;
    let parsed: Value = serde_json::from_str(&body).map_err(|e| format!("unreadable NASA answer: {e}"))?;
    let received = match parsed {
        Value::Array(items) => items,
        other => vec![other],
    };
    let usable: Vec<Value> = received.iter().filter(|item| sanity::is_usable(item)).cloned().collect();
    if usable.is_empty() && !received.is_empty() {
        return Err("NASA API returned only placeholder entries".into());
    }
    Ok(usable)
}

/// Cache key from the parameters that shape the answer, in a stable order.
fn cache_key(params: &[(String, String)]) -> String {
    let keyed: Vec<(String, String)> =
        params.iter().filter(|(key, _)| KEYED_PARAMS.contains(&key.as_str())).cloned().collect();
    get_cache_key(CACHE_NAMESPACE, &keyed)
}

/// Whether a boolean query parameter is set to `true`.
fn flag(params: &[(String, String)], name: &str) -> bool {
    params.iter().any(|(key, value)| key == name && value.eq_ignore_ascii_case("true"))
}

/// A JSON response that reports where the answer came from.
fn answer(body: &Value, cache_status: &str, source: &str) -> worker::Result<Response> {
    let mut response = Response::from_json(body)?;
    response.headers_mut().set("X-Cache-Status", cache_status)?;
    response.headers_mut().set("X-Apod-Source", source)?;
    Ok(response)
}

/// An error in the classic APOD API's shape.
fn failure_response(status: u16, message: &str) -> worker::Result<Response> {
    let body = json!({ "code": status, "msg": message, "service_version": "v1" });
    Ok(Response::from_json(&body)?.with_status(status))
}
