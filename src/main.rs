#![allow(non_snake_case)]

mod components;
mod pages;
mod server_interactions;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Serialize, Deserialize};
use log::LevelFilter;

use wiz_bulb::bulb::Bulb;

use pages::{Home, PageNotFound, Discover};

pub static BASE_URL: &str = "http://localhost:3000/";


#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/discover")]
    Discover {},
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️
    rsx! { Router::<Route> {} }
}

