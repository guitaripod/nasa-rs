//! # NASA API Proxy Worker
//! 
//! A Cloudflare Worker that provides a unified interface to various NASA APIs with caching,
//! rate limiting, and enhanced functionality.
//! 
//! ## Features
//! 
//! - **Unified API Access**: Single endpoint for multiple NASA data sources
//! - **Intelligent Caching**: Reduces API calls and improves response times
//! - **Rate Limiting**: Protects against abuse and ensures fair usage
//! - **CORS Support**: Enables browser-based applications to access the API
//! - **Enhanced Error Handling**: Consistent error responses across all endpoints
//! 
//! ## Available APIs
//! 
//! - APOD (Astronomy Picture of the Day)
//! - NeoWs (Near Earth Objects)
//! - DONKI (Space Weather Database)
//! - Earth Imagery
//! - EPIC (Earth Polychromatic Imaging Camera)
//! - Mars Rover Photos
//! - NASA Image and Video Library
//! - Exoplanet Archive
//! - SSD/CNEOS (Solar System Dynamics)
//! - Tech Transfer

use worker::*;

mod cache;
mod error;
mod handlers;
mod middleware;
mod models;
mod router;
mod utils;
#[macro_use]
mod macros;

pub use error::{NasaApiError, Result};

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> worker::Result<Response> {
    // Enable panic logging for better debugging
    console_error_panic_hook::set_once();
    
    // Use the router from the router module with proper context
    let router = Router::with_data((env.clone(), ctx));
    
    // Add all routes
    router
        // Landing page
        .get("/", |_, _| {
            let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>NASA API Proxy Service</title>
    <style>
        body {
            margin: 0;
            padding: 0;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            background: #0a0e27;
            color: #ffffff;
            min-height: 100vh;
            display: flex;
            flex-direction: column;
        }
        .header {
            background: linear-gradient(135deg, #1e3c72 0%, #2a5298 100%);
            padding: 3rem 0;
            text-align: center;
            box-shadow: 0 2px 10px rgba(0,0,0,0.3);
        }
        .header h1 {
            margin: 0;
            font-size: 3em;
            font-weight: 700;
            text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
        }
        .header p {
            margin: 1rem 0 0 0;
            font-size: 1.3em;
            opacity: 0.9;
        }
        .container {
            flex: 1;
            max-width: 1200px;
            margin: 3rem auto;
            padding: 0 2rem;
        }
        .cards {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 2rem;
            margin-top: 3rem;
        }
        .card {
            background: rgba(255, 255, 255, 0.05);
            border: 1px solid rgba(255, 255, 255, 0.1);
            border-radius: 12px;
            padding: 2rem;
            transition: all 0.3s ease;
        }
        .card:hover {
            background: rgba(255, 255, 255, 0.08);
            transform: translateY(-5px);
            box-shadow: 0 10px 30px rgba(0,0,0,0.3);
        }
        .card h3 {
            margin: 0 0 1rem 0;
            color: #61affe;
            font-size: 1.5em;
        }
        .card p {
            line-height: 1.6;
            opacity: 0.9;
        }
        .btn {
            display: inline-block;
            margin-top: 1rem;
            padding: 0.8rem 2rem;
            background: #61affe;
            color: white;
            text-decoration: none;
            border-radius: 6px;
            font-weight: 600;
            transition: all 0.3s ease;
        }
        .btn:hover {
            background: #4a9eff;
            transform: translateY(-2px);
            box-shadow: 0 5px 15px rgba(97, 175, 254, 0.4);
        }
        .features {
            margin-top: 3rem;
            padding: 2rem;
            background: rgba(255, 255, 255, 0.03);
            border-radius: 12px;
        }
        .features h2 {
            color: #61affe;
            margin-bottom: 1.5rem;
        }
        .features ul {
            list-style: none;
            padding: 0;
        }
        .features li {
            padding: 0.5rem 0;
            position: relative;
            padding-left: 2rem;
        }
        .features li:before {
            content: "✓";
            position: absolute;
            left: 0;
            color: #4CAF50;
            font-weight: bold;
        }
        .footer {
            text-align: center;
            padding: 2rem;
            opacity: 0.7;
            border-top: 1px solid rgba(255, 255, 255, 0.1);
        }
        .endpoints {
            background: rgba(0, 0, 0, 0.3);
            padding: 1rem;
            border-radius: 8px;
            margin-top: 1rem;
            font-family: monospace;
            font-size: 0.9em;
            overflow-x: auto;
        }
        code {
            background: rgba(255, 255, 255, 0.1);
            padding: 2px 6px;
            border-radius: 3px;
            font-family: monospace;
        }
    </style>
</head>
<body>
    <div class="header">
        <h1>🚀 NASA API Proxy Service</h1>
        <p>High-performance access to NASA's data universe</p>
    </div>
    
    <div class="container">
        <div class="cards">
            <div class="card">
                <h3>📚 API Documentation</h3>
                <p>Explore our comprehensive API documentation with interactive examples and detailed endpoint descriptions.</p>
                <a href="/api/docs" class="btn">View Documentation</a>
            </div>
            
            <div class="card">
                <h3>🌍 Available APIs</h3>
                <p>Access 10+ NASA data sources including APOD, Mars Rover Photos, Near Earth Objects, Space Weather, and more.</p>
                <div class="endpoints">
                    <div>GET /api/apod</div>
                    <div>GET /api/neo/feed</div>
                    <div>GET /api/mars-photos/{rover}/photos</div>
                    <div>GET /api/donki/flr</div>
                    <div>... and many more!</div>
                </div>
            </div>
            
            <div class="card">
                <h3>⚡ Performance</h3>
                <p>Intelligent caching and edge computing ensure fast response times for all API requests worldwide.</p>
                <p style="margin-top: 1rem;"><code>X-Cache-Status</code> header indicates cache hits</p>
            </div>
        </div>
        
        <div class="features">
            <h2>Features</h2>
            <ul>
                <li>No API key required - authentication handled automatically</li>
                <li>Intelligent caching reduces latency and improves reliability</li>
                <li>Rate limiting protection (100 requests/minute per IP)</li>
                <li>CORS enabled for browser applications</li>
                <li>Consistent error handling across all endpoints</li>
                <li>Real-time space weather notifications</li>
                <li>Historical data access for research</li>
                <li>RESTful API design following best practices</li>
            </ul>
        </div>
        
        <div class="features">
            <h2>Quick Start</h2>
            <p>Make your first API request:</p>
            <div class="endpoints">
                curl https://nasa-api-worker.guitaripod.workers.dev/api/apod
            </div>
            <p style="margin-top: 1rem;">For more examples and detailed usage, check out the <a href="/api/docs" style="color: #61affe;">API documentation</a>.</p>
        </div>
    </div>
    
    <div class="footer">
        <p>NASA API Proxy Service | Powered by Cloudflare Workers</p>
        <p>Data provided by NASA's Open APIs</p>
    </div>
</body>
</html>"#;
            Response::ok(html).map(|r| {
                r.with_headers(worker::Headers::from_iter(vec![
                    ("Content-Type", "text/html; charset=utf-8"),
                ]))
            })
        })
        
        // Health check
        .get("/health", |_, _| Response::ok("OK"))
        
        // APOD (Astronomy Picture of the Day)
        .get_async("/api/apod", handlers::apod::get_apod)
        
        // NeoWs (Near Earth Objects)
        .get_async("/api/neo/feed", handlers::neo::get_neo_feed)
        .get_async("/api/neo/:asteroid_id", handlers::neo::get_neo_lookup)
        .get_async("/api/neo/browse", handlers::neo::get_neo_browse)
        
        // DONKI (Space Weather)
        .get_async("/api/donki/cme", handlers::donki::get_cme)
        .get_async("/api/donki/cme-analysis", handlers::donki::get_cme_analysis)
        .get_async("/api/donki/gst", handlers::donki::get_gst)
        .get_async("/api/donki/ips", handlers::donki::get_ips)
        .get_async("/api/donki/flr", handlers::donki::get_flr)
        .get_async("/api/donki/sep", handlers::donki::get_sep)
        .get_async("/api/donki/mpc", handlers::donki::get_mpc)
        .get_async("/api/donki/rbe", handlers::donki::get_rbe)
        .get_async("/api/donki/hss", handlers::donki::get_hss)
        .get_async("/api/donki/wsa-enlil", handlers::donki::get_wsa_enlil)
        .get_async("/api/donki/notifications", handlers::donki::get_notifications)
        
        // Earth Imagery
        .get_async("/api/earth/imagery", handlers::earth::get_imagery)
        .get_async("/api/earth/assets", handlers::earth::get_assets)
        
        // EPIC
        .get_async("/api/epic/natural/all", handlers::epic::get_natural_all)
        .get_async("/api/epic/natural/date/:date", handlers::epic::get_natural_date)
        .get_async("/api/epic/enhanced/all", handlers::epic::get_enhanced_all)
        .get_async("/api/epic/enhanced/date/:date", handlers::epic::get_enhanced_date)
        
        // Mars Rover Photos
        .get_async("/api/mars-photos/:rover/photos", handlers::mars::get_rover_photos)
        .get_async("/api/mars-photos/:rover/latest", handlers::mars::get_latest_photos)
        .get_async("/api/mars-photos/manifests/:rover", handlers::mars::get_manifest)
        
        // Tech Transfer
        .get_async("/api/techtransfer/patents", handlers::tech::get_patents)
        .get_async("/api/techtransfer/patents-issued", handlers::tech::get_patents_issued)
        .get_async("/api/techtransfer/software", handlers::tech::get_software)
        .get_async("/api/techtransfer/spinoffs", handlers::tech::get_spinoffs)
        
        // NASA Image and Video Library
        .get_async("/api/media/search", handlers::media::search_media)
        .get_async("/api/media/asset/:nasa_id", handlers::media::get_asset)
        .get_async("/api/media/metadata/:nasa_id", handlers::media::get_metadata)
        .get_async("/api/media/captions/:nasa_id", handlers::media::get_captions)
        
        // Exoplanet Archive
        .get_async("/api/exoplanets/query", handlers::exoplanets::query_exoplanets)
        
        // SSD/CNEOS (Solar System Dynamics)
        .get_async("/api/ssd/cad", handlers::ssd::get_close_approach)
        .get_async("/api/ssd/sbdb", handlers::ssd::get_small_body)
        .get_async("/api/ssd/sentry", handlers::ssd::get_sentry)
        .get_async("/api/ssd/scout", handlers::ssd::get_scout)
        .get_async("/api/ssd/nhats", handlers::ssd::get_nhats)
        .get_async("/api/ssd/fireballs", handlers::ssd::get_fireballs)
        
        // API Documentation
        .get_async("/api/docs", handlers::docs::get_swagger_ui)
        .get_async("/api/docs/", handlers::docs::get_swagger_ui)
        .get_async("/api/docs/openapi.json", handlers::docs::get_openapi_json)
        .get_async("/api/docs/openapi.yaml", handlers::docs::get_openapi_yaml)
        
        // Apply CORS middleware to all routes
        .or_else_any_method_async("/*catchall", |req, _| async move {
            middleware::cors::handle_cors(req).await
        })
        .run(req, env)
        .await
        .or_else(|err| {
            console_error!("Router error: {}", err);
            Response::error("Internal Server Error", 500)
        })
}