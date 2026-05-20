//! ODP website SSR/dev server binary.
//!
//! This binary plays two roles depending on how it is invoked:
//!
//! * `cargo leptos serve` runs it as a normal axum SSR server with
//!   live-reload — useful during development to render + hydrate
//!   the site against the cargo-leptos asset pipeline.
//! * Invoked with the `--prerender` flag (added in a later commit),
//!   it walks the route table and writes a fully-static
//!   `target/site/` tree that ships to Cloudflare Pages.
//!
//! The production deploy never runs this binary as a server. CF
//! Pages only serves the static output.

use axum::Router;
use leptos::config::get_configuration;
use leptos_axum::{generate_route_list, LeptosRoutes};
use odp::{App, Shell, ShellProps};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let conf = get_configuration(None).expect("failed to read leptos configuration");
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let site_root = leptos_options.site_root.clone();
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || {
                Shell(ShellProps {
                    options: leptos_options.clone(),
                })
            }
        })
        .fallback_service(ServeDir::new(site_root.to_string()))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind site_addr");
    println!("listening on http://{addr}");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("server failed");
}
