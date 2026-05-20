//! ODP website root.
//!
//! Pure SSG: the [`Shell`] component renders the full `<!DOCTYPE
//! html>` + `<html>` shell once per route via the prerender pass in
//! `src/bin/server.rs`, producing static HTML files under
//! `target/site/`. There is no client-side Leptos runtime — a small
//! hand-written `public/interactive.js` handles the few interactive
//! widgets (theme toggle, mobile nav, video facade) directly against
//! the prerendered DOM.

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

/// Inline `<head>` script that resolves the active theme before any
/// CSS loads. Eliminates the first-paint flash that would otherwise
/// occur between the prerendered (default-light) HTML and the user's
/// stored / preferred theme.
///
/// Resolution order: `localStorage["odp-theme"]` > `prefers-color-
/// scheme: dark` > `light`.
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

/// Top-level HTML shell rendered once per prerendered route.
#[component]
pub fn Shell() -> impl IntoView {
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
                <link rel="stylesheet" href="/odp.css" />
                <MetaTags />
                <script>{THEME_FLASH_SCRIPT}</script>
            </head>
            <body>
                <App />
                <script src="/interactive.js" defer></script>
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

/// Every URL that the prerender pass should emit as a static HTML
/// file. Includes the 11 fixed routes plus one entry per
/// announcement slug so `/announcements/<slug>` permalinks render
/// without a runtime server.
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
