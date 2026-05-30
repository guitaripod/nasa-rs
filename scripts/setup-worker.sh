#!/bin/bash
#
# Bootstraps a self-hosted nasa-rs Cloudflare Worker:
#   - copies wrangler.toml.example -> wrangler.toml (gitignored)
#   - creates the NASA_CACHE and RATE_LIMIT KV namespaces
#   - writes their IDs into wrangler.toml
#   - prompts for the NASA_API_KEY secret
#
# After it finishes, run `wrangler deploy`.

set -euo pipefail

cd "$(dirname "$0")/.."

echo "nasa-rs worker setup"
echo "===================="

if ! command -v wrangler >/dev/null 2>&1; then
    echo "Error: wrangler is not installed. Install it with: npm install -g wrangler" >&2
    exit 1
fi

if [ ! -f wrangler.toml.example ]; then
    echo "Error: wrangler.toml.example not found. Run this from the repository root." >&2
    exit 1
fi

if [ -f wrangler.toml ]; then
    read -r -p "wrangler.toml already exists. Overwrite it from the template? [y/N] " answer
    case "$answer" in
        y | Y) ;;
        *)
            echo "Keeping the existing wrangler.toml; aborting setup."
            exit 0
            ;;
    esac
fi

cp wrangler.toml.example wrangler.toml
echo "Created wrangler.toml from the template."
echo ""

replace_token() {
    local token="$1" value="$2"
    if [ -z "$value" ]; then
        echo "  could not detect the id automatically — edit wrangler.toml and replace $token by hand." >&2
        return
    fi
    sed -i.bak "s/$token/$value/" wrangler.toml && rm -f wrangler.toml.bak
    echo "  $token set"
}

create_namespace() {
    local binding="$1" token="$2"
    echo "Creating KV namespace: $binding"
    local output id
    if ! output="$(wrangler kv namespace create "$binding" 2>&1)"; then
        echo "$output" >&2
        echo "  namespace creation failed — set $token in wrangler.toml manually." >&2
        return
    fi
    id="$(printf '%s' "$output" | grep -oiE '[0-9a-f]{32}' | head -1)"
    replace_token "$token" "$id"
}

create_namespace NASA_CACHE __NASA_CACHE_ID__
create_namespace RATE_LIMIT __RATE_LIMIT_ID__

echo ""
echo "Set your NASA API key (get a free one at https://api.nasa.gov; DEMO_KEY works for low volume):"
wrangler secret put NASA_API_KEY || echo "  skipped/failed: NASA_API_KEY"

echo ""
echo "Setup complete. Review wrangler.toml, then deploy with:"
echo "  wrangler deploy"
echo ""
echo "Finally, point the CLI at your worker:"
echo "  nasa config set api_endpoint https://<your-worker>.workers.dev"
