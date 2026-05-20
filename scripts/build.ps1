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
$ErrorActionPreference = "Stop"
Set-Location (Join-Path $PSScriptRoot "..")

Write-Host "==> 1/5  npm ci"
npm ci
if ($LASTEXITCODE -ne 0) { throw "npm ci failed" }

Write-Host "==> 2/5  npm run build:css"
npm run build:css
if ($LASTEXITCODE -ne 0) { throw "build:css failed" }

Write-Host "==> 3/5  cargo build --release --bin odp"
cargo build --release --bin odp
if ($LASTEXITCODE -ne 0) { throw "cargo build failed" }

Write-Host "==> 4/5  npm run build:assets"
npm run build:assets
if ($LASTEXITCODE -ne 0) { throw "build:assets failed" }

Write-Host "==> 5/5  prerender all routes"
& "./target/release/odp.exe" --prerender
if ($LASTEXITCODE -ne 0) { throw "prerender failed" }

Write-Host ""
Write-Host "build: success -- deploy contents of target/site/"
