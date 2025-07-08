# NASA-RS Scripts

This directory contains utility scripts for the NASA-RS project.

## serve

Simple Python script to serve the documentation locally.

### Usage

```bash
# From project root
./scripts/serve

# Opens browser automatically at http://localhost:8000
# Press Ctrl+C to stop
```

### Features
- Automatically finds available port (8000-8009)
- Opens browser automatically
- No-cache headers for development
- Minimal logging (only errors)
- Clean shutdown on Ctrl+C

### Requirements
- Python 3

## test_cli.sh

Script for testing CLI commands.