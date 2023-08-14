#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::Router;

mod pages;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App)
}

fn App(cx: Scope) -> Element {
    render!( Router::<pages::Route> {} )
}
