use dioxus::prelude::*;

use crate::components::SideMenu;


#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        SideMenu {}
        div {
            id: "links",
            h1 { color: "var(--gray-1)", "Page not found" }
            p { color: "var(--gray-1)", "We are terribly sorry, but the page you requested doesn't exist." }
            pre { color: "var(--accent-1)", "log:\nattemped to navigate to: {route:?}" }
        }
    }
}