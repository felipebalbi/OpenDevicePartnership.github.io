//! ODP website root.
//!
//! Built as a dual-target crate by cargo-leptos:
//!
//!   * The `hydrate` feature compiles to wasm and ships in
//!     `target/site/pkg/odp.{js,wasm}`. The `hydrate` entry below
//!     attaches Leptos to a pre-rendered DOM.
//!   * The `ssr` feature compiles to a native binary
//!     (`src/bin/server.rs`) used for `cargo leptos serve` in dev
//!     and for the static prerender pass that produces the files
//!     deployed to Cloudflare Pages.
//!
//! The [`shell`] component renders the full `<!DOCTYPE html>` +
//! `<html>` shell once per request / per prerendered file, then
//! mounts [`App`] inside `<body>`. Page chrome (sticky `NavBar`,
//! `Footer`, theme provider) is rendered once around the route
//! tree by `App`; individual pages render only their content.

use crate::components::nav::{Footer, NavBar};
use crate::components::theme::ThemeProvider;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::path;

pub mod components;
pub mod data;
mod pages;

use crate::pages::announcements::{AnnouncementDetailPage, AnnouncementsPage};
use crate::pages::boot_firmware::BootFirmware;
use crate::pages::community::Community;
use crate::pages::embedded_controller::EmbeddedController;
use crate::pages::getting_started::GettingStarted;
use crate::pages::home::Home;
use crate::pages::not_found::NotFoundPage;
use crate::pages::projects::Projects;
use crate::pages::team_ec::TeamEC;
use crate::pages::team_ec_services::TeamECServices;
use crate::pages::team_patina::TeamPatina;
use crate::pages::unified_ec_services::WindowsEcServices;

/// Inline `<head>` script that resolves the active theme before
/// any CSS loads. Eliminates the first-paint flash that would
/// otherwise occur while the hydrate wasm bundle boots and runs
/// `ThemeProvider`'s effect.
///
/// Resolution order matches `components::theme::read_initial_theme`:
/// localStorage > prefers-color-scheme > "light".
#[cfg(feature = "ssr")]
const THEME_FLASH_SCRIPT: &str = r#"
(function () {
  try {
    var key = 'odp-theme';
    var stored = window.localStorage && window.localStorage.getItem(key);
    var t = stored === 'dark' || stored === 'light'
      ? stored
      : (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches
          ? 'dark'
          : 'light');
    document.documentElement.setAttribute('data-theme', t);
  } catch (_) {
    document.documentElement.setAttribute('data-theme', 'light');
  }
})();
"#;

/// SSR-side helper: replicate the hash-resolution logic that
/// `leptos::HydrationScripts` and `leptos_meta::HashedStylesheet`
/// use, returning the fully hashed (or unhashed, in dev) base
/// filenames for the JS shim and the wasm bundle.
#[cfg(feature = "ssr")]
fn hashed_pkg_basenames(options: &LeptosOptions) -> (String, String) {
    let mut js = options.output_name.to_string();
    let mut wasm = options.output_name.to_string();
    if options.hash_files {
        if let Some(hash_path) = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .map(|p| p.join(options.hash_file.as_ref()))
            .filter(|p| p.exists())
        {
            if let Ok(hashes) = std::fs::read_to_string(&hash_path) {
                for line in hashes.lines() {
                    if let Some((file, hash)) = line.trim().split_once(':') {
                        match file {
                            "js" => js.push_str(&format!(".{}", hash.trim())),
                            "wasm" => wasm.push_str(&format!(".{}", hash.trim())),
                            _ => {}
                        }
                    }
                }
            }
        }
    } else if std::option_env!("LEPTOS_OUTPUT_NAME").is_none() {
        wasm.push_str("_bg");
    }
    (js, wasm)
}

/// Stand-in for `leptos::HydrationScripts` that does NOT emit
/// `<link rel=modulepreload>` or `<link rel="preload" as=fetch>`
/// for the wasm/JS bundle. Those preloads pull ~600KB into the
/// browser's critical bandwidth window and starve LCP-critical
/// resources (CSS, fonts, hero text paint) on cold loads.
///
/// Instead we emit a single `<script type=module>` at the end of
/// `<body>` that defers the dynamic `import()` until after the
/// `load` event has fired -- i.e. after the first meaningful
/// paint has happened and the user is already looking at the
/// fully-rendered static HTML. Hydration then attaches event
/// handlers in the background; the page is interactive a tick
/// later, but the user perceives a snappy first paint.
///
/// Mirrors `leptos`'s built-in hydration script
/// (`hydration_script.js`) byte-for-byte for the boot sequence
/// itself; we only change *when* it runs and skip the preload
/// hints.
#[cfg(feature = "ssr")]
#[component]
fn DeferredHydration(options: LeptosOptions) -> impl IntoView {
    let (js, wasm) = hashed_pkg_basenames(&options);
    let pkg_path = options.site_pkg_dir.to_string();
    let script = format!(
        "addEventListener('load',function(){{\
            import('/{pkg_path}/{js}.js').then(function(m){{\
                m.default({{module_or_path:'/{pkg_path}/{wasm}.wasm'}}).then(function(){{m.hydrate();}});\
            }});\
        }});"
    );
    view! { <script type="module">{script}</script> }
}

/// Top-level HTML shell rendered by cargo-leptos's SSR pipeline
/// (and by our prerender bin).
#[cfg(feature = "ssr")]
#[component]
pub fn Shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" dir="ltr">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <link rel="icon" href="/images/odpicon.ico" type="image/x-icon" />
                <link
                    rel="preload"
                    href="/fonts/geist-latin.woff2"
                    r#as="font"
                    r#type="font/woff2"
                    crossorigin="anonymous"
                />
                <AutoReload options=options.clone() />
                <HashedStylesheet options=options.clone() id="leptos" />
                <MetaTags />
                <script>{THEME_FLASH_SCRIPT}</script>
            </head>
            <body>
                <App />
                <DeferredHydration options />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Open Device Partnership" />
        <Meta
            name="description"
            content="Open Device Partnership: an open collaboration for secure, modern device firmware. Built in the open, by the people who maintain it."
        />

        <ThemeProvider>
            <Router>
                <div class="flex flex-col min-h-screen w-full bg-surface-page text-ink-primary">
                    <NavBar />
                    <main class="flex-1 w-full">
                        <Routes fallback=NotFoundPage>
                            <Route path=path!("/") view=Home />
                            <Route path=path!("/projects") view=Projects />
                            <Route path=path!("/getting-started") view=GettingStarted />
                            <Route path=path!("/community") view=Community />
                            <Route path=path!("/announcements") view=AnnouncementsPage />
                            <Route path=path!("/announcements/:slug") view=AnnouncementDetailPage />
                            <Route path=path!("/boot-firmware") view=BootFirmware />
                            <Route path=path!("/embedded-controller") view=EmbeddedController />
                            <Route path=path!("/windows-ec-services") view=WindowsEcServices />
                            <Route path=path!("/team-patina") view=TeamPatina />
                            <Route path=path!("/team-ec") view=TeamEC />
                            <Route path=path!("/team-ec-services") view=TeamECServices />
                            // Synthetic route that exposes the `<Routes fallback>`
                            // page so the prerender pass can write it to
                            // `target/site/404.html` for Cloudflare Pages' built-in
                            // not-found handling. The wildcard fallback isn't part
                            // of `generate_route_list()`, so leptos_axum has nothing
                            // to dispatch arbitrary URLs to; this gives us one.
                            // Kept out of `static_routes()` so no `/_404/index.html`
                            // is generated.
                            <Route path=path!("/_404") view=NotFoundPage />
                        </Routes>
                    </main>
                    <Footer />
                </div>
            </Router>
        </ThemeProvider>
    }
}

/// Hydrate entry point. Called automatically when the wasm module
/// loads in the browser (via `#[wasm_bindgen(start)]`).
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

/// Every URL that the prerender pass should emit as a static HTML
/// file. Includes the 11 fixed routes plus one entry per
/// announcement slug so `/announcements/<slug>` permalinks render
/// without a runtime server.
///
/// Kept in this crate (instead of the prerender bin) so the source
/// of truth lives next to the route table in [`App`] and the
/// announcement list in [`data::announcements`].
#[cfg(feature = "ssr")]
pub fn static_routes() -> Vec<String> {
    let mut routes = vec![
        "/".to_string(),
        "/projects".to_string(),
        "/getting-started".to_string(),
        "/community".to_string(),
        "/announcements".to_string(),
        "/boot-firmware".to_string(),
        "/embedded-controller".to_string(),
        "/windows-ec-services".to_string(),
        "/team-patina".to_string(),
        "/team-ec".to_string(),
        "/team-ec-services".to_string(),
    ];
    for a in data::announcements::ANNOUNCEMENTS {
        routes.push(format!("/announcements/{}", a.slug));
    }
    routes
}
