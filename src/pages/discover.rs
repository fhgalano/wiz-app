use dioxus::prelude::*;

use crate::components::{SideMenu, Footer};
use crate::server_interactions::discover_bulbs;


#[component]
pub fn Discover() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        div { id: "core",
            SideMenu {}
            div { id: "links",
                h1 { "Discover" }
                UnknownBulb {}
            }
                Footer {}
        }
    }
}

#[component]
fn UnknownBulb() -> Element {

    let mut cb = use_resource(move || async move {
        discover_bulbs().await
    });

    let unknowns = match &*cb.read() {
        Some(Ok(v)) => {
            rsx! {
                {v.iter().map(|i| rsx! { p { "{i.to_string()}" } })}
            }
        },
        Some(Err(_)) => None,
        None => rsx! { p { "Loading..." } }
    }; unknowns
}
