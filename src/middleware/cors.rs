use worker::{Headers, Method, Request, Response};

pub async fn handle_cors(req: Request) -> worker::Result<Response> {
    let origin = req.headers().get("Origin")?.unwrap_or_default();
    
    let headers = Headers::new();
    headers.set("Access-Control-Allow-Origin", &origin)?;
    headers.set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")?;
    headers.set("Access-Control-Allow-Headers", "Content-Type, Authorization")?;
    headers.set("Access-Control-Max-Age", "86400")?; // 24 hours
    
    if req.method() == Method::Options {
        return Ok(Response::empty()?.with_headers(headers));
    }
    
    // Return 404 for non-OPTIONS requests to catch-all
    Response::error("Not Found", 404)
}

#[allow(dead_code)]
pub fn add_cors_headers(response: &mut Response, origin: &str) -> worker::Result<()> {
    let headers = response.headers_mut();
    headers.set("Access-Control-Allow-Origin", origin)?;
    headers.set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")?;
    headers.set("Access-Control-Allow-Headers", "Content-Type, Authorization")?;
    headers.set("Access-Control-Max-Age", "86400")?;
    Ok(())
}