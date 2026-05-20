//! Renders the repository dependency graph using D3 v7.
//!
//! All heavy lifting (force simulation, zoom controls, drag handlers,
//! styles) lives in two static assets that cargo-leptos copies next
//! to the wasm bundle:
//!
//!  * `public/repo_graph.js`  -- defines `window.__odpRenderGraph()`
//!    and self-loads D3 on demand.
//!  * `public/repo_graph.css` -- the graph styles.
//!
//! Both assets are loaded **lazily**, only when a `RepositoryGraph`
//! component first mounts (i.e. when the user navigates to a project
//! page). This keeps the ~280 KB D3 bundle and the graph stylesheet
//! off the critical path for every other page.
//!
//! ## Load order on first mount
//!
//! 1. The Effect publishes per-page payload as `window.__odpGraphData`.
//! 2. The Effect calls `request_render()`. If `__odpRenderGraph` is
//!    already defined (subsequent mounts), it runs immediately.
//! 3. The Effect calls `ensure_graph_assets()`, which injects
//!    `<link rel="stylesheet" href="/repo_graph.css">` and
//!    `<script src="/repo_graph.js">` exactly once. When the script
//!    finishes loading, it self-executes `render()` because
//!    `__odpGraphData` is already set (see `public/repo_graph.js`).
//! 4. `repo_graph.js` itself injects `<script src=".../d3.v7.min.js">`
//!    on first render, then resolves once D3 is ready.
//!
//! Subsequent route changes just publish fresh data + call render
//! synchronously -- no more script injection or downloads.
//!
//! Under SSR (prerender) the Effect never runs, so the component
//! renders just the empty `<svg>` shell. The hydrate-side Effect
//! takes over once wasm boots in the browser.

use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::{JsCast, JsValue};
#[cfg(feature = "hydrate")]
use web_sys::js_sys;

#[cfg(feature = "hydrate")]
const REPO_GRAPH_SCRIPT_ID: &str = "odp-repo-graph-script";
#[cfg(feature = "hydrate")]
const REPO_GRAPH_SCRIPT_SRC: &str = "/repo_graph.js";
#[cfg(feature = "hydrate")]
const REPO_GRAPH_STYLE_ID: &str = "odp-repo-graph-style";
#[cfg(feature = "hydrate")]
const REPO_GRAPH_STYLE_HREF: &str = "/repo_graph.css";

#[component]
pub fn RepositoryGraph(#[prop(into)] nodes: String, #[prop(into)] links: String) -> impl IntoView {
    // The Effect is a no-op under SSR (effects do not run); under
    // hydrate it publishes per-page data and lazy-loads the D3
    // pipeline on first mount.
    Effect::new({
        let nodes = nodes.clone();
        let links = links.clone();
        move |_| {
            run_graph_effect(&nodes, &links);
        }
    });
    // Silence "unused under ssr" lints without changing the API.
    let _ = (&nodes, &links);

    view! {
        <div class="repository-graph">
            <div id="zoom-controls">
                <button id="zoom-in">"+"</button>
                <button id="zoom-out">"−"</button>
                <button id="zoom-fit">"⛶"</button>
            </div>
            <svg width="100%" height="100%" style="position:absolute;"></svg>
        </div>
    }
}

#[cfg(feature = "hydrate")]
fn run_graph_effect(nodes_json: &str, links_json: &str) {
    publish_graph_data(nodes_json, links_json);
    request_render();
    ensure_graph_assets();
}

#[cfg(not(feature = "hydrate"))]
fn run_graph_effect(_nodes_json: &str, _links_json: &str) {}

/// Parses the per-page node/link JSON via the browser's native
/// `JSON.parse` and stores the result on `window.__odpGraphData`.
/// Silently no-ops if either string is not valid JSON.
#[cfg(feature = "hydrate")]
fn publish_graph_data(nodes_json: &str, links_json: &str) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let payload = js_sys::Object::new();
    if let Ok(nodes) = js_sys::JSON::parse(nodes_json) {
        let _ = js_sys::Reflect::set(&payload, &JsValue::from_str("nodes"), &nodes);
    }
    if let Ok(links) = js_sys::JSON::parse(links_json) {
        let _ = js_sys::Reflect::set(&payload, &JsValue::from_str("links"), &links);
    }
    let _ = js_sys::Reflect::set(&window, &JsValue::from_str("__odpGraphData"), &payload);
}

/// Calls `window.__odpRenderGraph()` if it is defined. On the very
/// first mount the script that defines it has not been injected yet,
/// in which case this is a no-op and `repo_graph.js` will self-render
/// once it loads (it checks for `__odpGraphData` at the end of its
/// IIFE).
#[cfg(feature = "hydrate")]
fn request_render() {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Ok(render) = js_sys::Reflect::get(&window, &JsValue::from_str("__odpRenderGraph")) else {
        return;
    };
    if let Some(func) = render.dyn_ref::<js_sys::Function>() {
        let _ = func.call0(&JsValue::UNDEFINED);
    }
}

/// Injects `<link rel="stylesheet">` and `<script src="/repo_graph.js">`
/// into `<head>` exactly once per session. Subsequent calls are cheap
/// no-ops. Both assets are kept off the critical path so the landing
/// page (and every non-project route) never pays for them.
#[cfg(feature = "hydrate")]
fn ensure_graph_assets() {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Some(head) = document.head() else {
        return;
    };

    if document.get_element_by_id(REPO_GRAPH_STYLE_ID).is_none() {
        if let Ok(link) = document.create_element("link") {
            let _ = link.set_attribute("id", REPO_GRAPH_STYLE_ID);
            let _ = link.set_attribute("rel", "stylesheet");
            let _ = link.set_attribute("href", REPO_GRAPH_STYLE_HREF);
            let _ = head.append_child(&link);
        }
    }

    if document.get_element_by_id(REPO_GRAPH_SCRIPT_ID).is_none() {
        if let Ok(script) = document.create_element("script") {
            let _ = script.set_attribute("id", REPO_GRAPH_SCRIPT_ID);
            let _ = script.set_attribute("src", REPO_GRAPH_SCRIPT_SRC);
            let _ = script.set_attribute("defer", "");
            let _ = head.append_child(&script);
        }
    }
}
