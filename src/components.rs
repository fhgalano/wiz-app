use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::{FaLightbulb, FaObjectGroup, FaPlus};

use crate::Route;
use crate::server_interactions::{get_bulb, bulb_on, bulb_off};


#[component]
pub fn SideMenu() -> Element {
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
                route: Route::Discover {}
            }
        }
    }
}

#[component]
pub fn BulbCard() -> Element {
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
pub fn SideMenuButton(icon: Element, tooltip: String, route: Route) -> Element {
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
pub fn Footer() -> Element {
    rsx! {
        div {
            id: "footer",
            a { "Info" }
            a { "Contact" }
            span { "Copyright GimmeVeggies 2024" }
        }
    }
}
