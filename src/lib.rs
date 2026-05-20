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
mod data;
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

/// Top-level HTML shell rendered by cargo-leptos's SSR pipeline
/// (and by our prerender bin).
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
                    as_="font"
                    type_="font/woff2"
                    crossorigin="anonymous"
                />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <link rel="stylesheet" id="leptos" href="/pkg/odp.css" />
                <MetaTags />
                <script>{THEME_FLASH_SCRIPT}</script>
            </head>
            <body>
                <App />
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
            <Router base="/">
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
