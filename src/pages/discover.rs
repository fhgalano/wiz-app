use std::net::{Ipv4Addr, IpAddr};

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::FaPlus;
use wiz_bulb::bulb::Bulb;

use crate::components::{SideMenu, Footer};
use crate::server_interactions::{discover_bulbs, add_bulb};


#[component]
pub fn Discover() -> Element {
    let mut refresh = use_signal(|| true);
    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        div { id: "core",
            SideMenu {}
            div { id: "links",
                button {
                    onclick: move |_| *refresh.write() = true, "refresh"
                }
                h1 { "Discover" }
                UnknownBulbs { refresh: refresh }
            }
                Footer {}
        }
    }
}

#[component]
fn UnknownBulbs(refresh: Signal<bool>) -> Element {

    // NOTE: In the future this should be able to return information about the state of the bulb
    let mut cb = use_resource(move || async move {
        discover_bulbs().await
    });

    if *refresh.read() {
        cb.clear();
        cb.restart();
        *refresh.write() = false;
    }

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
    let mut name: Signal<String> = use_signal(|| "".to_string());
    let mut id = use_signal(|| 0);

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
            if *form_display.read() == "visible" {
                AddBulbForm { 
                    ip: ip, 
                    form_display: form_display,
                    name: name,
                    id: id,
                }
            }
        }
    }
}

#[component]
fn AddBulbForm(
        ip: String, 
        form_display: Signal<&'static str>,
        name: Signal<String>,
        id: Signal<u32>,
    ) -> Element {
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
                onsubmit: move |event| { 
                    println!("Submitted Event: {event:?}");
                    // check if the bulb was added successfully
                    // if bulb added successfully
                    let t_bulb = Bulb::new(
                        IpAddr::V4(ip.parse().expect("Unable to parse ip {ip}")),
                        (*name.read().clone()).to_string(),
                        *id.read(),
                    );
                    dbg!(&t_bulb);
                    spawn(async move {
                        match add_bulb(&t_bulb).await {
                            Ok(_) => println!("adding bulb worked"),
                            Err(e) => println!("some error occurred: {e}")
                        };
                    });
                    *form_display.write() = "hidden";
                },
                div {
                    h4 { "Add New Bulb {ip}" }

                    p { "name" }
                    input {
                        name: "name", r#type: "text", placeholder: "New Bulb", required: true,
                        oninput: move |event| name.set(event.value()) 
                    }
                    input {
                        name: "id", r#type: "number", placeholder: "Some Numeric ID", required: true,
                        oninput: move |event| id.set(event.value().parse().unwrap()) 
                    }
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

