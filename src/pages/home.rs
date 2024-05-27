use dioxus::prelude::*;

use crate::components::{SideMenu, BulbCard, Footer};

#[component]
pub fn Home() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        div { id: "core",
            SideMenu {}
            div { id: "links",
                h1 { "WizLights" }
                a { href: "https://dioxuslabs.com/learn/0.5/", "📚 Learn Dioxus" }
                a { href: "https://github.com/DioxusLabs/dioxus-std", "⚙️ Dioxus Standard Library" }
                BulbCard { }
            }
                Footer {}
        }
    }
}

