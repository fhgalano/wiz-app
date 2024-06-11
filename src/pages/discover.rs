use std::net::Ipv4Addr;

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::FaPlus;

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
                UnknownBulbs {}
                MockUnknownBulbs {}
            }
                Footer {}
        }
    }
}

#[component]
fn UnknownBulbs() -> Element {

    // NOTE: In the future this should be able to return information about the state of the bulb
    let mut cb = use_resource(move || async move {
        discover_bulbs().await
    });

    let unknowns = match &*cb.read() {
        Some(Ok(v)) => {
            rsx! {
                {v.iter().map(|i| rsx! { UnknownBulb { ip: i.to_string() } })}
            }
        },
        Some(Err(_)) => None,
        None => rsx! { p { "Loading..." } }
    }; unknowns
}

#[component]
fn UnknownBulb(ip: String) -> Element {
    let mut form_display = use_signal(|| "hidden");

    rsx! {
        div {
            class: "add-bulb",
            span { "{ip}" },
            button {
                class: "add-bulb-button",
                onclick: move |_| *form_display.write() = "visible",
                Icon {
                    width: 30,
                    height: 30,
                    icon: FaPlus,
                }
            }
            AddBulbForm { ip: ip, form_display: form_display }
        }
    }
}

#[component]
fn MockUnknownBulbs() -> Element {

    let unknown_ips = mock_bulb_ip_vec();

    rsx! {
        {unknown_ips.iter().map(|i| rsx! { MockUnknownBulb { ip: i.to_string() } })}
    }
}

#[component]
fn MockUnknownBulb(ip: String) -> Element {
    let mut form_display = use_signal(|| "hidden");

    rsx! {
        div {
            class: "add-bulb",
            span { "{ip}" },
            button {
                class: "add-bulb-button",
                onclick: move |_| *form_display.write() = "visible",
                Icon {
                    width: 30,
                    height: 30,
                    icon: FaPlus,
                }
            }
            AddBulbForm { ip: ip, form_display: form_display }
        }
    }
}

fn mock_bulb_ip_vec() -> Vec<Ipv4Addr> {
    vec![
        Ipv4Addr::new(127, 0, 0, 4),
        Ipv4Addr::new(127, 0, 0, 5),
        Ipv4Addr::new(127, 0, 0, 6),
        Ipv4Addr::new(127, 0, 0, 7),
    ]
}

#[component]
fn AddBulbForm(ip: String, form_display: Signal<&'static str>) -> Element {
    rsx! {
        div {
            class: "add-bulb-popup",
            position: "fixed",
            visibility: form_display(),
            opacity: {
                if form_display() == "hidden" {
                    0
                }
                else {
                    1
                }
            },
            transition: "visibility 0.8s ease-out, opacity 0.4s 0.4s linear",
            z_index: 4,
            form {
                onsubmit: move |event| { println!("Submitted Event: {event:?}") },
                div {
                    h4 { "Add New Bulb {ip}" }

                    p { "name" }
                    input { name: "name", r#type: "text", placeholder: "New Bulb", required: true }

                    button {
                        r#type: "submit",
                        "submit"
                    }

                    button {
                        onclick: move |_| { *form_display.write() = "hidden"; },
                        "cancel"
                    }
                }
            }
        }
        div {
            position: "fixed",
            width: "100%",
            height: "100%",
            top: 0,
            left: 0,
            right: 0,
            bottom: 0,
            z_index: 1,
            background_color: "rgba(0,0,0,0.5)",
            visibility: form_display(),
        }
    }
}

