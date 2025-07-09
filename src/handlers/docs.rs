use worker::*;
use serde_yaml;

const OPENAPI_SPEC: &str = include_str!("../../openapi.yaml");
const SWAGGER_UI_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>NASA API Documentation</title>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/swagger-ui-dist@5.10.3/swagger-ui.css">
    <style>
        body {
            margin: 0;
            padding: 0;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
        }
        #swagger-ui {
            max-width: 1460px;
            margin: 0 auto;
        }
        .swagger-ui .topbar {
            display: none;
        }
        .swagger-ui .info {
            margin: 50px 0;
        }
        .swagger-ui .info .title {
            font-size: 2.5em;
            font-weight: 700;
            color: #2c3e50;
        }
        .swagger-ui .info .description {
            font-size: 1.1em;
            line-height: 1.8;
            color: #34495e;
        }
        .swagger-ui .info .description h1 {
            color: #2c3e50;
            margin-top: 2em;
        }
        .swagger-ui .info .description h2 {
            color: #34495e;
            margin-top: 1.5em;
        }
        .swagger-ui .info .description code {
            background: #f4f4f4;
            padding: 2px 6px;
            border-radius: 3px;
            font-size: 0.9em;
        }
        .swagger-ui .scheme-container {
            background: #f8f9fa;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 1px 3px rgba(0,0,0,0.1);
        }
        .swagger-ui select {
            font-size: 1em;
            padding: 8px 12px;
            border-radius: 4px;
            border: 1px solid #ddd;
        }
        .swagger-ui .btn {
            font-size: 1em;
            font-weight: 600;
            padding: 8px 20px;
            border-radius: 4px;
            box-shadow: 0 1px 3px rgba(0,0,0,0.12);
        }
        .swagger-ui .btn.authorize {
            background: #4CAF50;
            color: white;
            border: none;
        }
        .swagger-ui .btn.authorize:hover {
            background: #45a049;
        }
        .swagger-ui .opblock.opblock-get .opblock-summary-method {
            background: #61affe;
        }
        .swagger-ui .opblock.opblock-get {
            border-color: #61affe;
        }
        .swagger-ui .opblock.opblock-get .opblock-summary {
            border-color: #61affe;
        }
        .swagger-ui .opblock-tag {
            font-size: 1.3em;
            font-weight: 600;
            margin-bottom: 1em;
            color: #2c3e50;
        }
        .swagger-ui .opblock-tag small {
            font-size: 0.7em;
            font-weight: 400;
            color: #7f8c8d;
            margin-left: 1em;
        }
        .header-banner {
            background: linear-gradient(135deg, #1e3c72 0%, #2a5298 100%);
            color: white;
            padding: 2rem 0;
            text-align: center;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        .header-banner h1 {
            margin: 0;
            font-size: 2.5em;
            font-weight: 700;
        }
        .header-banner p {
            margin: 0.5rem 0 0 0;
            font-size: 1.2em;
            opacity: 0.9;
        }
    </style>
</head>
<body>
    <div class="header-banner">
        <h1>🚀 NASA API Proxy Service</h1>
        <p>High-performance access to NASA's data universe</p>
    </div>
    <div id="swagger-ui"></div>
    <script src="https://cdn.jsdelivr.net/npm/swagger-ui-dist@5.10.3/swagger-ui-bundle.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/swagger-ui-dist@5.10.3/swagger-ui-standalone-preset.js"></script>
    <script>
        window.onload = function() {
            const ui = SwaggerUIBundle({
                url: "/api/docs/openapi.json",
                dom_id: '#swagger-ui',
                deepLinking: true,
                presets: [
                    SwaggerUIBundle.presets.apis,
                    SwaggerUIStandalonePreset
                ],
                plugins: [
                    SwaggerUIBundle.plugins.DownloadUrl
                ],
                layout: "StandaloneLayout",
                defaultModelsExpandDepth: -1,
                defaultModelExpandDepth: 1,
                docExpansion: "none",
                filter: true,
                showExtensions: true,
                showCommonExtensions: true,
                tagsSorter: "alpha",
                operationsSorter: "alpha",
                onComplete: function() {
                    // Add NASA logo if needed
                    console.log("NASA API Documentation loaded successfully");
                }
            });
            window.ui = ui;
        }
    </script>
</body>
</html>"#;

pub async fn get_openapi_json(_req: Request, _data: RouteContext<(Env, Context)>) -> worker::Result<Response> {
    // Parse YAML and convert to JSON
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(OPENAPI_SPEC)
        .map_err(|e| worker::Error::RustError(format!("Failed to parse OpenAPI spec: {}", e)))?;
    
    let json_string = serde_json::to_string_pretty(&yaml_value)
        .map_err(|e| worker::Error::RustError(format!("Failed to convert to JSON: {}", e)))?;
    
    Response::ok(json_string)
        .map(|r| {
            r.with_headers(Headers::from_iter(vec![
                ("Content-Type", "application/json"),
                ("Cache-Control", "public, max-age=3600"),
            ]))
        })
}

pub async fn get_openapi_yaml(_req: Request, _data: RouteContext<(Env, Context)>) -> worker::Result<Response> {
    Response::ok(OPENAPI_SPEC)
        .map(|r| {
            r.with_headers(Headers::from_iter(vec![
                ("Content-Type", "application/x-yaml"),
                ("Cache-Control", "public, max-age=3600"),
            ]))
        })
}

pub async fn get_swagger_ui(_req: Request, _data: RouteContext<(Env, Context)>) -> worker::Result<Response> {
    Response::ok(SWAGGER_UI_HTML)
        .map(|r| {
            r.with_headers(Headers::from_iter(vec![
                ("Content-Type", "text/html; charset=utf-8"),
                ("Cache-Control", "public, max-age=3600"),
            ]))
        })
}