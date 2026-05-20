//! Theme: static toggle button for vanilla-JS interactivity.
//!
//! Two themes are supported: `light` (default) and `dark`. The
//! resolved theme is set as the `data-theme` attribute on the
//! `<html>` element by the inline `THEME_FLASH_SCRIPT` (see
//! `crate::THEME_FLASH_SCRIPT`) before any CSS loads. The
//! `public/interactive.js` script then attaches a click handler to
//! the button rendered below to flip the attribute and persist the
//! choice in `localStorage["odp-theme"]`.
//!
//! Both icons (moon and sun) are rendered as siblings; `style/base.css`
//! shows/hides them based on the active `[data-theme]` attribute so
//! that the visible icon flips instantly with the theme.

use leptos::prelude::*;
use unocss_classes::uno;

/// No-op kept as a context provider so existing component code can
/// continue to wrap its tree in `<ThemeProvider>`. The actual theme
/// state lives entirely in the DOM (`<html data-theme>`) and is
/// managed by `public/interactive.js`.
#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    children()
}

/// Sun/moon icon button that toggles the theme.
///
/// Rendered as a plain static button with `data-theme-toggle`;
/// `public/interactive.js` flips `<html data-theme>` and
/// `localStorage["odp-theme"]` on click. Both icons live in the
/// markup; `style/base.css` hides the one that doesn't match the
/// current theme.
#[component]
pub fn ThemeToggle() -> impl IntoView {
    view! {
        <button
            type="button"
            data-theme-toggle
            class=uno![
                "inline-flex items-center justify-center w-10 h-10 rounded-md",
                "text-ink-secondary hover:(text-ink-primary bg-surface-sunken)",
                "transition-colors duration-200"
            ]
            aria-label="Toggle theme"
        >
            <span class=uno!("i-lucide-moon w-5 h-5 theme-icon-light") aria-hidden="true"></span>
            <span class=uno!("i-lucide-sun w-5 h-5 theme-icon-dark") aria-hidden="true"></span>
        </button>
    }
}
