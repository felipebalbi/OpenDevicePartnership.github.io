use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::about::About;
use crate::pages::contact::Contact;
use crate::pages::news_and_blogs::NewsAndBlogs;
use crate::pages::boot_firmware::BootFirmware;
use crate::pages::embedded_controller::EmbeddedController;
use crate::pages::windows_ec_services::WindowsEcServices;
use crate::pages::mptf::Mptf;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />
        <Stylesheet id="leptos" href="/style/output.css" />

        // sets the document title
        <Title text="Open Device Partnership" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes fallback=|| view! { NotFound }>
                <Route path=path!("/") view=Home />
                <Route path=path!("/about") view=About />
                <Route path=path!("/contact") view=Contact />
                <Route path=path!("/news-and-blogs") view=NewsAndBlogs />
                <Route path=path!("/boot-firmware") view=BootFirmware />
                <Route path=path!("/embedded-controller") view=EmbeddedController />
                <Route path=path!("/windows-ec-services") view=WindowsEcServices />
                <Route path=path!("/mptf") view=Mptf />
            </Routes>
        </Router>
    }
}
