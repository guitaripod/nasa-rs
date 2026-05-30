#!/bin/bash
#
# Builds the Cloudflare Worker to WebAssembly.
#
# This repo's Cargo.lock pins wasm-bindgen 0.2.100, but the pinned worker-build
# downloads the 0.2.105 CLI, whose bindgen schema is incompatible. We seed
# worker-build's cache with a matching 0.2.100 CLI so the build is reproducible
# across local, CI and self-host environments.
# Remove this workaround once `worker` is upgraded to >= 0.8 (works with current
# worker-build out of the box).

set -euo pipefail

WORKER_BUILD_VERSION="0.1.14"
WASM_BINDGEN_VERSION="0.2.100"
WORKER_BUILD_EXPECTS="0.2.105"

case "$(uname -s)-$(uname -m)" in
    Linux-x86_64) TARGET="x86_64-unknown-linux-musl" ;;
    Darwin-arm64) TARGET="aarch64-apple-darwin" ;;
    Darwin-x86_64) TARGET="x86_64-apple-darwin" ;;
    *)
        echo "build-worker: unrecognized platform $(uname -s)-$(uname -m); building without the wasm-bindgen pin." >&2
        TARGET=""
        ;;
esac

cargo install -q "worker-build@${WORKER_BUILD_VERSION}"

if [ -n "$TARGET" ]; then
    cache="$HOME/.cache/worker-build/wasm-bindgen-${TARGET}-${WORKER_BUILD_EXPECTS}"
    if [ "$("${cache}/wasm-bindgen" --version 2>/dev/null)" != "wasm-bindgen ${WASM_BINDGEN_VERSION}" ]; then
        pkg="wasm-bindgen-${WASM_BINDGEN_VERSION}-${TARGET}"
        url="https://github.com/wasm-bindgen/wasm-bindgen/releases/download/${WASM_BINDGEN_VERSION}/${pkg}.tar.gz"
        tmp="$(mktemp -d)"
        curl -fsSL "$url" | tar xz -C "$tmp"
        mkdir -p "$cache"
        cp "${tmp}/${pkg}/wasm-bindgen" "${cache}/wasm-bindgen"
        chmod +x "${cache}/wasm-bindgen"
        rm -rf "$tmp"
    fi
fi

exec "$HOME/.cargo/bin/worker-build" --release --no-opt
