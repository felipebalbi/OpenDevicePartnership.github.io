# Production build pipeline for the ODP website (Windows / PowerShell).
#
# Mirrors scripts/build.sh step-for-step so local Windows contributors
# can produce the same deploy-ready target/site/ tree the CI workflow
# uploads to Cloudflare Pages. See scripts/build.sh for full notes.

$ErrorActionPreference = 'Stop'

Set-Location (Join-Path $PSScriptRoot '..')

Write-Host '==> 1/5  npm ci'
npm ci
if ($LASTEXITCODE -ne 0) { throw 'npm ci failed' }

Write-Host '==> 2/5  npm run build:css'
npm run build:css
if ($LASTEXITCODE -ne 0) { throw 'npm run build:css failed' }

Write-Host '==> 3/5  cargo leptos build --release'
cargo leptos build --release
if ($LASTEXITCODE -ne 0) { throw 'cargo leptos build failed' }

Write-Host '==> 4/5  npm run postbuild'
npm run postbuild
if ($LASTEXITCODE -ne 0) { throw 'npm run postbuild failed' }

Write-Host '==> 5/5  prerender all routes'
& .\target\release\odp.exe --prerender
if ($LASTEXITCODE -ne 0) { throw 'prerender failed' }

Write-Host ''
Write-Host 'build: success -- deploy contents of target/site/'
