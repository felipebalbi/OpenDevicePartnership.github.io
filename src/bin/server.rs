//! ODP website prerender binary.
//!
//! Pure SSG: this binary has no networking role in production. It
//! walks [`odp::static_routes`], drives an in-process axum router
//! via [`tower::ServiceExt::oneshot`] (no socket), and writes each
//! response as a static `target/site/<route>/index.html` file plus
//! a top-level `404.html` for Cloudflare Pages' built-in not-found
//! handling.
//!
//! A `serve` mode is kept for local browser previewing of the
//! prerendered output (`./odp serve`) — it just spins up a static
//! file server pointed at `target/site/`.

use axum::body::{to_bytes, Body};
use axum::extract::Request;
use axum::Router;
use leptos::config::LeptosOptions;
use leptos_axum::{generate_route_list, LeptosRoutes};
use odp::{static_routes, App, Shell};
use std::path::{Path, PathBuf};
use tokio::fs;
use tower::ServiceExt;
use tower_http::services::ServeDir;

const SITE_ROOT: &str = "target/site";
const SITE_ADDR: &str = "127.0.0.1:3000";

fn leptos_options() -> LeptosOptions {
    // No cargo-leptos in this branch, so there is no
    // `[package.metadata.leptos]` table to read. The few fields the
    // SSG path needs are filled in by hand; everything else falls
    // back to the type's defaults.
    let mut opts = LeptosOptions::builder().output_name("odp").site_root(SITE_ROOT).build();
    opts.site_addr = SITE_ADDR.parse().expect("parse site_addr");
    opts
}

fn build_router(opts: LeptosOptions) -> Router {
    let site_root = opts.site_root.to_string();
    let routes = generate_route_list(App);

    Router::new()
        .leptos_routes(&opts, routes, Shell)
        .fallback_service(ServeDir::new(site_root))
        .with_state(opts)
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = if args.iter().any(|a| a == "--prerender") {
        Mode::Prerender
    } else {
        Mode::Serve
    };

    let opts = leptos_options();
    let site_root = PathBuf::from(opts.site_root.to_string());
    let addr = opts.site_addr;
    let app = build_router(opts);

    if matches!(mode, Mode::Prerender) {
        let listing = generate_route_list(App);
        eprintln!("prerender: generate_route_list returned {} route(s):", listing.len());
        for r in &listing {
            eprintln!("  - {}", r.path());
        }
    }

    match mode {
        Mode::Serve => {
            let listener = tokio::net::TcpListener::bind(&addr)
                .await
                .expect("failed to bind site_addr");
            println!("listening on http://{addr}");
            axum::serve(listener, app.into_make_service())
                .await
                .expect("server failed");
        }
        Mode::Prerender => {
            prerender(app, &site_root).await;
        }
    }
}

enum Mode {
    Serve,
    Prerender,
}

async fn prerender(app: Router, site_root: &Path) {
    let routes = static_routes();
    println!(
        "prerender: writing {} routes into {}",
        routes.len(),
        site_root.display()
    );

    for url in &routes {
        let html = render_one(&app, url).await;
        let out = output_path(site_root, url);
        if let Some(parent) = out.parent() {
            fs::create_dir_all(parent).await.expect("create_dir_all");
        }
        fs::write(&out, html).await.expect("write html");
        println!("  {} -> {}", url, out.display());
    }

    // 404.html for Cloudflare Pages.
    let html = render_one(&app, "/_404").await;
    let out = site_root.join("404.html");
    fs::write(&out, html).await.expect("write 404.html");
    println!("  (fallback) -> {}", out.display());
}

async fn render_one(app: &Router, url: &str) -> String {
    let req = Request::builder()
        .uri(url)
        .header("accept", "text/html")
        .body(Body::empty())
        .expect("build request");
    let resp = app
        .clone()
        .oneshot(req)
        .await
        .unwrap_or_else(|e| panic!("oneshot {url}: {e}"));

    let status = resp.status();
    let body = resp.into_body();
    let bytes = to_bytes(body, 8 * 1024 * 1024)
        .await
        .unwrap_or_else(|e| panic!("read body {url}: {e}"));
    let html = String::from_utf8(bytes.to_vec()).unwrap_or_else(|e| panic!("utf8 {url}: {e}"));

    if !status.is_success() && url != "/_404" {
        panic!("non-success status {status} for {url}");
    }
    html
}

/// `/` -> `<root>/index.html`, `/foo` -> `<root>/foo/index.html`,
/// `/foo/bar` -> `<root>/foo/bar/index.html`.
fn output_path(site_root: &Path, url: &str) -> PathBuf {
    let trimmed = url.trim_start_matches('/');
    if trimmed.is_empty() {
        site_root.join("index.html")
    } else {
        site_root.join(trimmed).join("index.html")
    }
}
