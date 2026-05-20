#!/usr/bin/env bash
# Production build pipeline for the ODP website (pure SSG, no wasm).
#
# Output: target/site/  (deploy this directory to Cloudflare Pages)
#
# Steps:
#   1. npm ci                 install JS toolchain
#   2. npm run build:css      UnoCSS scan + concat into style/main.css
#   3. cargo build --release  build the prerender binary
#   4. npm run build:assets   clean + copy public/* + minify css/js
#   5. ./target/release/odp --prerender   write per-route index.html
set -euo pipefail
cd "$(dirname "$0")/.."

echo "==> 1/5  npm ci"
npm ci

echo "==> 2/5  npm run build:css"
npm run build:css

echo "==> 3/5  cargo build --release --bin odp"
cargo build --release --bin odp

echo "==> 4/5  npm run build:assets"
npm run build:assets

echo "==> 5/5  prerender all routes"
./target/release/odp --prerender

echo
echo "build: success -- deploy contents of target/site/"
