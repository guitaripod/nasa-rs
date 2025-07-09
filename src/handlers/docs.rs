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
        :root {
            --nasa-blue: #0B3D91;
            --nasa-red: #FC3D21;
            --space-dark: #0a0e27;
            --space-blue: #1e3c72;
            --space-light-blue: #2a5298;
            --star-white: #ffffff;
        }

        body {
            margin: 0;
            padding: 0;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            background: #f5f5f5;
            background-image: 
                radial-gradient(circle at 20% 80%, rgba(30, 60, 114, 0.1) 0%, transparent 50%),
                radial-gradient(circle at 80% 20%, rgba(42, 82, 152, 0.1) 0%, transparent 50%),
                radial-gradient(circle at 40% 40%, rgba(11, 61, 145, 0.05) 0%, transparent 50%);
        }

        /* Header with stars animation */
        .header-banner {
            background: linear-gradient(135deg, var(--space-dark) 0%, var(--space-blue) 50%, var(--space-light-blue) 100%);
            color: white;
            padding: 3rem 0;
            text-align: center;
            box-shadow: 0 4px 20px rgba(0,0,0,0.3);
            position: relative;
            overflow: hidden;
        }

        .header-banner::before {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background-image: 
                radial-gradient(2px 2px at 20% 30%, white, transparent),
                radial-gradient(2px 2px at 60% 70%, white, transparent),
                radial-gradient(1px 1px at 50% 50%, white, transparent),
                radial-gradient(1px 1px at 80% 10%, white, transparent),
                radial-gradient(2px 2px at 90% 60%, white, transparent);
            background-size: 200px 200px;
            background-position: 0 0, 40px 60px, 130px 270px, 70px 100px, 150px 50px;
            animation: stars 60s linear infinite;
            opacity: 0.4;
        }

        @keyframes stars {
            from { transform: translateY(0); }
            to { transform: translateY(-200px); }
        }

        .header-banner h1 {
            margin: 0;
            font-size: 3em;
            font-weight: 700;
            text-shadow: 3px 3px 6px rgba(0,0,0,0.5);
            position: relative;
            z-index: 1;
        }

        .header-banner p {
            margin: 1rem 0 0 0;
            font-size: 1.3em;
            opacity: 0.95;
            text-shadow: 2px 2px 4px rgba(0,0,0,0.4);
            position: relative;
            z-index: 1;
        }

        .nasa-badge {
            position: absolute;
            top: 20px;
            right: 20px;
            width: 80px;
            height: 80px;
            background: white;
            border-radius: 50%;
            display: flex;
            align-items: center;
            justify-content: center;
            box-shadow: 0 4px 15px rgba(0,0,0,0.3);
            font-size: 2.5em;
            z-index: 2;
        }

        #swagger-ui {
            max-width: 1460px;
            margin: 0 auto;
            background: white;
            box-shadow: 0 0 40px rgba(0,0,0,0.1);
            border-radius: 8px;
            margin-top: -30px;
            position: relative;
            z-index: 10;
        }

        .swagger-ui .topbar {
            display: none;
        }

        .swagger-ui .info {
            margin: 0;
            padding: 50px;
            background: linear-gradient(to bottom, #ffffff 0%, #f8f9fa 100%);
            border-bottom: 3px solid var(--nasa-blue);
        }

        .swagger-ui .info .title {
            font-size: 2.8em;
            font-weight: 700;
            color: var(--nasa-blue);
            margin-bottom: 0.5em;
        }

        .swagger-ui .info .description {
            font-size: 1.1em;
            line-height: 1.8;
            color: #34495e;
        }

        .swagger-ui .info .description h1 {
            color: var(--nasa-blue);
            margin-top: 2em;
            font-size: 2em;
            border-bottom: 2px solid var(--nasa-blue);
            padding-bottom: 0.5em;
        }

        .swagger-ui .info .description h2 {
            color: var(--space-blue);
            margin-top: 1.5em;
            font-size: 1.5em;
        }

        .swagger-ui .info .description code {
            background: #e8f0fe;
            padding: 3px 8px;
            border-radius: 4px;
            font-size: 0.9em;
            color: var(--nasa-blue);
            border: 1px solid #c6d9f1;
        }

        .swagger-ui .info .description ul li::before {
            content: "🚀";
            margin-right: 0.5em;
        }

        .swagger-ui .scheme-container {
            background: linear-gradient(135deg, #f8f9fa 0%, #e9ecef 100%);
            padding: 25px;
            border-radius: 12px;
            box-shadow: 0 2px 8px rgba(0,0,0,0.08);
            margin: 20px;
            border: 1px solid #dee2e6;
        }

        .swagger-ui .wrapper {
            padding: 0 20px 40px;
        }

        .swagger-ui select {
            font-size: 1em;
            padding: 10px 15px;
            border-radius: 6px;
            border: 2px solid #c6d9f1;
            background: white;
            transition: all 0.3s ease;
        }

        .swagger-ui select:focus {
            border-color: var(--nasa-blue);
            box-shadow: 0 0 0 3px rgba(11, 61, 145, 0.1);
        }

        .swagger-ui .btn {
            font-size: 1em;
            font-weight: 600;
            padding: 10px 24px;
            border-radius: 6px;
            box-shadow: 0 2px 5px rgba(0,0,0,0.15);
            transition: all 0.3s ease;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }

        .swagger-ui .btn.execute {
            background: var(--nasa-blue);
            color: white;
            border: none;
        }

        .swagger-ui .btn.execute:hover {
            background: #0a3270;
            transform: translateY(-2px);
            box-shadow: 0 4px 12px rgba(11, 61, 145, 0.3);
        }

        .swagger-ui .btn.authorize {
            background: #4CAF50;
            color: white;
            border: none;
        }

        .swagger-ui .btn.authorize:hover {
            background: #45a049;
            transform: translateY(-2px);
            box-shadow: 0 4px 12px rgba(76, 175, 80, 0.3);
        }

        /* Operation blocks styling */
        .swagger-ui .opblock.opblock-get .opblock-summary-method {
            background: #61affe;
            font-weight: 700;
            padding: 8px 15px;
            min-width: 80px;
            text-align: center;
            border-radius: 4px;
            box-shadow: 0 2px 4px rgba(97, 175, 254, 0.3);
        }

        .swagger-ui .opblock.opblock-get {
            border-color: #61affe;
            border-width: 2px;
            border-radius: 8px;
            overflow: hidden;
            margin-bottom: 20px;
            box-shadow: 0 2px 8px rgba(0,0,0,0.08);
            transition: all 0.3s ease;
        }

        .swagger-ui .opblock.opblock-get:hover {
            box-shadow: 0 4px 16px rgba(97, 175, 254, 0.2);
            transform: translateY(-2px);
        }

        .swagger-ui .opblock.opblock-get .opblock-summary {
            border-color: #61affe;
            background: linear-gradient(to right, #f8fbff 0%, #ffffff 100%);
        }

        .swagger-ui .opblock-tag {
            font-size: 1.6em;
            font-weight: 700;
            margin: 2em 0 1em 0;
            color: var(--nasa-blue);
            border-bottom: 3px solid var(--nasa-blue);
            padding-bottom: 0.5em;
            display: flex;
            align-items: center;
            gap: 1em;
        }

        .swagger-ui .opblock-tag:before {
            content: "🛸";
            font-size: 1.2em;
        }

        .swagger-ui .opblock-tag small {
            font-size: 0.6em;
            font-weight: 400;
            color: #6c757d;
            margin-left: auto;
            font-style: italic;
        }

        /* Response styling */
        .swagger-ui .responses-wrapper .responses-inner {
            padding: 20px;
            background: #f8f9fa;
            border-radius: 8px;
            margin-top: 20px;
        }

        .swagger-ui table tbody tr td {
            padding: 12px;
            border-bottom: 1px solid #e9ecef;
        }

        .swagger-ui table tbody tr:hover {
            background: #f8f9fa;
        }

        /* Model/Schema styling */
        .swagger-ui .model-box {
            background: white;
            border: 2px solid #e9ecef;
            border-radius: 8px;
            padding: 20px;
            margin: 10px 0;
        }

        .swagger-ui .model-title {
            font-size: 1.2em;
            font-weight: 700;
            color: var(--nasa-blue);
            margin-bottom: 15px;
        }

        /* Parameter styling */
        .swagger-ui .parameter__name {
            color: var(--nasa-blue);
            font-weight: 600;
        }

        .swagger-ui .parameter__type {
            color: var(--space-blue);
            font-family: 'Monaco', 'Menlo', monospace;
            font-size: 0.9em;
        }

        /* Footer */
        .api-footer {
            text-align: center;
            padding: 40px 20px;
            color: #6c757d;
            border-top: 1px solid #e9ecef;
            margin-top: 80px;
            background: #f8f9fa;
        }

        .api-footer a {
            color: var(--nasa-blue);
            text-decoration: none;
            font-weight: 600;
        }

        .api-footer a:hover {
            text-decoration: underline;
        }

        /* Loading animation */
        @keyframes pulse {
            0% { opacity: 1; }
            50% { opacity: 0.5; }
            100% { opacity: 1; }
        }

        .swagger-ui .loading-container {
            padding: 40px;
            text-align: center;
            animation: pulse 2s ease-in-out infinite;
        }

        /* Make it responsive */
        @media (max-width: 768px) {
            .header-banner h1 {
                font-size: 2em;
            }
            
            .header-banner p {
                font-size: 1em;
            }
            
            .nasa-badge {
                width: 60px;
                height: 60px;
                font-size: 2em;
            }
            
            .swagger-ui .info {
                padding: 30px 20px;
            }
            
            .swagger-ui .wrapper {
                padding: 0 10px 20px;
            }
        }
    </style>
</head>
<body>
    <div class="header-banner">
        <div class="nasa-badge">🚀</div>
        <h1>NASA API Proxy Service</h1>
        <p>Explore the cosmos through data 🌌</p>
    </div>
    <div id="swagger-ui"></div>
    <div class="api-footer">
        <p>Built with ❤️ using Rust and Cloudflare Workers</p>
        <p>
            <a href="https://github.com/marcusziade/nasa-rs">GitHub</a> · 
            <a href="/">Home</a> · 
            <a href="https://api.nasa.gov">NASA APIs</a>
        </p>
    </div>
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
                persistAuthorization: true,
                onComplete: function() {
                    console.log("🚀 NASA API Documentation loaded successfully");
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
        .map_err(|e| worker::Error::RustError(format!("Failed to parse OpenAPI spec: {e}")))?;
    
    let json_string = serde_json::to_string_pretty(&yaml_value)
        .map_err(|e| worker::Error::RustError(format!("Failed to convert to JSON: {e}")))?;
    
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