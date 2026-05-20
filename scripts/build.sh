#!/usr/bin/env bash
# Production build pipeline for the ODP website.
#
# Steps (in order; each must succeed):
#   1. npm ci                       -- pin Node deps for the CSS pipeline
#   2. npm run build:css            -- UnoCSS scan + concat into style/main.css
#   3. cargo leptos build --release -- hydrate wasm + SSR axum bin + asset hash
#   4. npm run postbuild            -- minify lazy JS, strip .d.ts from /pkg/
#   5. ./target/release/odp --prerender
#                                   -- walk every route, write static HTML
#
# Output: a fully-static, deploy-ready site tree at target/site/.
# Intended consumers: scripts/release.sh, the Cloudflare Pages deploy
# workflow, and contributors who want a one-shot local rebuild.

set -euo pipefail

cd "$(dirname "$0")/.."

echo "==> 1/5  npm ci"
npm ci

echo "==> 2/5  npm run build:css"
npm run build:css

echo "==> 3/5  cargo leptos build --release"
cargo leptos build --release

echo "==> 4/5  npm run postbuild"
npm run postbuild

echo "==> 5/5  prerender all routes"
./target/release/odp --prerender

echo
echo "build: success -- deploy contents of target/site/"
