#!/usr/bin/env python3
"""
Simple HTTP server for viewing the NASA-RS documentation locally.
Includes auto-reload functionality for development.
"""

import http.server
import socketserver
import os
import sys
import webbrowser
import time
from pathlib import Path

class NoCacheHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    """HTTP request handler with no caching headers"""
    
    def end_headers(self):
        self.send_header('Cache-Control', 'no-store, no-cache, must-revalidate')
        self.send_header('Expires', '0')
        super().end_headers()
    
    def do_GET(self):
        # Redirect root to index.html
        if self.path == '/':
            self.path = '/index.html'
        return super().do_GET()

def serve_docs(port=8000, open_browser=True):
    """Serve the documentation directory"""
    # Get the docs directory
    script_dir = Path(__file__).parent
    docs_dir = script_dir.parent / 'docs'
    
    if not docs_dir.exists():
        print(f"❌ Error: Documentation directory not found at {docs_dir}")
        sys.exit(1)
    
    # Change to docs directory
    os.chdir(docs_dir)
    
    # Set up the server
    handler = NoCacheHTTPRequestHandler
    
    try:
        with socketserver.TCPServer(("", port), handler) as httpd:
            print("🚀 NASA-RS Documentation Server")
            print("=" * 40)
            print(f"📁 Serving from: {docs_dir}")
            print(f"🌐 URL: http://localhost:{port}")
            print(f"📄 Landing page: http://localhost:{port}/index.html")
            print("=" * 40)
            print("Press Ctrl+C to stop the server\n")
            
            # Open browser if requested
            if open_browser:
                time.sleep(0.5)  # Small delay to ensure server is ready
                webbrowser.open(f'http://localhost:{port}')
            
            # Start serving
            httpd.serve_forever()
            
    except KeyboardInterrupt:
        print("\n\n✅ Server stopped")
    except OSError as e:
        if e.errno == 48:  # Address already in use
            print(f"❌ Error: Port {port} is already in use")
            print(f"Try a different port: python3 {__file__} --port 8001")
        else:
            print(f"❌ Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser(description='Serve NASA-RS documentation locally')
    parser.add_argument('--port', '-p', type=int, default=8000,
                        help='Port to serve on (default: 8000)')
    parser.add_argument('--no-browser', '-n', action='store_true',
                        help="Don't open browser automatically")
    
    args = parser.parse_args()
    
    serve_docs(port=args.port, open_browser=not args.no_browser)