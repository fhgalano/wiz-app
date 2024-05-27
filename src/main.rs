#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::{FaLightbulb, FaObjectGroup, FaPlus};
use dioxus_router::prelude::*;
use serde::{Serialize, Deserialize};
use log::LevelFilter;

use wiz_bulb::bulb::Bulb;

pub static BASE_URL: &str = "http://localhost:3000/";


#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    launch(App);
}

async fn get_bulb(name: String) -> Result<Bulb, reqwest::Error> {
    let url = format!("{}{}{}", BASE_URL, "bulb/", name);
    reqwest::get(url).await?.json().await
}

async fn bulb_on(id: i32) -> Result<(), reqwest::Error> {
    let url = format!("{}{}{}", BASE_URL, "bulb/on/", id);
    reqwest::get(url).await?.text().await.unwrap();
    Ok(())
}

async fn bulb_off(id: i32) -> Result<(), reqwest::Error> {
    let url = format!("{}{}{}", BASE_URL, "bulb/off/", id);
    reqwest::get(url).await?.text().await.unwrap();
    Ok(())
}

#[component]
fn App() -> Element {
    // Build cool things ✌️
    rsx! { Router::<Route> {} }
}

#[component]
fn Home() -> Element {
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

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
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
// #[component]
// fn BulbButton() -> Element {}

#[component]
fn SideMenu() -> Element {
    rsx! {
        div {
            id: "sidebar-box",
            SideMenuButton {
                icon: rsx! {
                    Icon {
                        width: 30,
                        height: 30,
                        icon: FaLightbulb,
                    }
                },
                tooltip: "Main Page️".to_string(),
                route: Route::Home {}
            }
            SideMenuButton {
                icon: rsx! {
                    Icon {
                        width: 30,
                        height: 30,
                        icon: FaObjectGroup,
                    }
                },
                tooltip: "tooltip ✌️".to_string(),
                route: Route::PageNotFound{ route: vec![] }
            }
            SideMenuButton {
                icon: rsx! {
                    Icon {
                        width: 30,
                        height: 30,
                        icon: FaPlus,
                    }
                },
                tooltip: "Bulb Discovery️".to_string(),
                route: Route::PageNotFound{ route: vec![] }
            }
        }
    }
}

#[component]
fn BulbCard() -> Element {
    let mut a = use_signal(move || 0);
    let mut cb = use_resource(move || async move {
        &a();
        get_bulb("test_bulb_1".to_string()).await
    });

    let element = match &*cb.read() {
        Some(Ok(bulb)) => {

            dbg!(bulb.clone());
            rsx! {
                div {
                    class: "bulb-card",
                    p { { bulb.name.to_owned() } },
                    match bulb.state {
                        true => {
                            rsx! {
                                button {
                                    class: "bulb-button on",
                                    onclick: move |_| {
                                        spawn(async move {
                                            bulb_off(1).await.unwrap();
                                            a += 1;
                                        });
                                    },
                                    p { "off" }
                                }
                            }
                        },
                        false => {
                            rsx! {
                                button {
                                    class: "bulb-button",
                                    onclick: move |_| {
                                        spawn(async move {
                                            bulb_on(1).await.unwrap();
                                            println!("before");
                                            a += 1;
                                        });
                                    },
                                    p { "on " }
                                }
                            }
                        },
                    }
                }
            }
        },
        Some(Err(e)) => rsx! { "an error occurred while loading bulb, {e}" },
        None => rsx! { p { "loading..." } },
    }; element
}

#[component]
fn SideMenuButton(icon: Element, tooltip: String, route: Route) -> Element {
    let nav = navigator();

    rsx! {
        div {
            class: "sidebar-icon",
            button {
                class: "sidebar-button",
                onclick: move |_| {
                    nav.push(route.clone());
                },
                {icon}
            }
            div { class: "tooltip", "{tooltip}" }
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        div {
            id: "footer",
            a { "Info" }
            a { "Contact" }
            span { "Copyright GimmeVeggies 2024" }
        }
    }
}
