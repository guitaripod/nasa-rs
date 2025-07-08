#!/bin/bash

# Simple script to serve the documentation landing page locally

PORT=${1:-8000}
DOCS_DIR="$(cd "$(dirname "$0")/../docs" && pwd)"

echo "🚀 NASA-RS Documentation Server"
echo "================================"
echo ""
echo "Serving documentation from: $DOCS_DIR"
echo "Access the landing page at: http://localhost:$PORT"
echo ""
echo "Press Ctrl+C to stop the server"
echo ""

# Check if Python 3 is available
if command -v python3 &> /dev/null; then
    cd "$DOCS_DIR" && python3 -m http.server $PORT
# Check if Python 2 is available
elif command -v python &> /dev/null; then
    cd "$DOCS_DIR" && python -m SimpleHTTPServer $PORT
# Check if Node.js http-server is available
elif command -v http-server &> /dev/null; then
    cd "$DOCS_DIR" && http-server -p $PORT
# Check if PHP is available
elif command -v php &> /dev/null; then
    cd "$DOCS_DIR" && php -S localhost:$PORT
else
    echo "❌ Error: No suitable web server found!"
    echo ""
    echo "Please install one of the following:"
    echo "  - Python 3: python3 -m http.server"
    echo "  - Python 2: python -m SimpleHTTPServer"
    echo "  - Node.js: npm install -g http-server"
    echo "  - PHP: php -S localhost:8000"
    exit 1
fi