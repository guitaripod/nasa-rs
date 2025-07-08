# NASA-RS Scripts

This directory contains utility scripts for the NASA-RS project.

## serve-docs.sh

A simple shell script to serve the documentation landing page locally using any available web server.

### Usage

```bash
# Serve on default port 8000
./scripts/serve-docs.sh

# Serve on custom port
./scripts/serve-docs.sh 3000
```

The script will automatically detect and use one of the following servers:
- Python 3 (`http.server`)
- Python 2 (`SimpleHTTPServer`)
- Node.js (`http-server`)
- PHP built-in server

## serve-docs.py

A more feature-rich Python script for serving documentation with additional features:
- No-cache headers for development
- Automatic browser opening
- Better error handling
- Command-line options

### Usage

```bash
# Serve on default port 8000 and open browser
./scripts/serve-docs.py

# Serve on custom port
./scripts/serve-docs.py --port 3000

# Don't open browser automatically
./scripts/serve-docs.py --no-browser

# Short options
./scripts/serve-docs.py -p 3000 -n
```

### Requirements

- Python 3.6 or higher

## test_cli.sh

Existing script for testing CLI commands (already present in the repository).