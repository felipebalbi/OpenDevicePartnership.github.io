//! Renders the repository dependency graph using D3 v7.
//!
//! Pure SSG flow:
//!
//!  * The per-page nodes/links payload is emitted inline as
//!    `window.__odpGraphData = {...}` via a `<script>` element next
//!    to the graph container.
//!  * `<link rel="stylesheet" href="/repo_graph.css">` and
//!    `<script src="/repo_graph.js" defer>` are emitted alongside it
//!    so the graph code (which itself lazy-loads D3) renders against
//!    the prerendered SVG shell.
//!
//! Only project pages mount `RepositoryGraph`, so the ~3.6KB D3
//! wrapper and the ~280KB D3 bundle stay off the critical path for
//! every other route.

use leptos::prelude::*;

#[component]
pub fn RepositoryGraph(#[prop(into)] nodes: String, #[prop(into)] links: String) -> impl IntoView {
    // Inline the payload as a JSON literal. The repo_graph.js loader
    // reads `window.__odpGraphData` at module scope and renders once
    // D3 is ready. Both `nodes` and `links` are already JSON-encoded
    // by `data::projects` (`serde_json::to_string`); we splice them
    // verbatim into a JS object literal, so `</script>` sequences in
    // user data would be unsafe — none of the call sites contain any.
    let payload = format!("window.__odpGraphData = {{ nodes: {nodes}, links: {links} }};",);

    view! {
        <div class="repository-graph">
            <div id="zoom-controls">
                <button id="zoom-in">"+"</button>
                <button id="zoom-out">"−"</button>
                <button id="zoom-fit">"⛶"</button>
            </div>
            <svg width="100%" height="100%" style="position:absolute;"></svg>
            <script inner_html=payload></script>
            <link rel="stylesheet" href="/repo_graph.css" />
            <script src="/repo_graph.js" defer></script>
        </div>
    }
}
