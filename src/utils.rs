use worker::{console_log, Date, Request};

#[allow(dead_code)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    console_error_panic_hook::set_once();
}

#[allow(dead_code)]
pub fn log_request(req: &Request) {
    console_log!(
        "{} - [{}]",
        Date::now().to_string(),
        req.path()
    );
}

pub fn get_api_key(env: &worker::Env) -> worker::Result<String> {
    // First try to get from secret
    if let Ok(secret) = env.secret("NASA_API_KEY") {
        return Ok(secret.to_string());
    }
    
    // Fall back to environment variable
    if let Ok(api_key) = env.var("NASA_API_KEY") {
        return Ok(api_key.to_string());
    }
    
    // Use demo key as last resort
    Ok("DEMO_KEY".to_string())
}

pub fn parse_query_params(req: &Request) -> worker::Result<Vec<(String, String)>> {
    let url = req.url()?;
    Ok(url
        .query_pairs()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect())
}

#[allow(dead_code)]
pub fn get_client_ip(req: &Request) -> String {
    req.headers()
        .get("CF-Connecting-IP")
        .ok()
        .flatten()
        .or_else(|| req.headers().get("X-Forwarded-For").ok().flatten())
        .unwrap_or_else(|| "unknown".to_string())
}