#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::Route;
use dioxus_router::Router;
use dioxus_toast::{ToastFrame, ToastManager};
use fermi::{use_atom_ref, use_init_atom_root, AtomRef};

use hooks::mode::init_mode_info;
use pages::starter::{About, HelloDioxus, SayHi};

mod components;
mod hooks;
mod pages;

static TOAST_MANAGER: AtomRef<ToastManager> = |_| ToastManager::default();

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App)
}

fn App(cx: Scope) -> Element {
    // init mode information
    init_mode_info(cx);
    use_init_atom_root(cx);

    cx.render(rsx! {
        // dioxus toast manager init
        ToastFrame { manager: use_atom_ref(cx, TOAST_MANAGER) }
        // dioxus router info
        Router {
            Route { to: "/", HelloDioxus {} }
            Route { to: "/hi/:name", SayHi {} }
            Route { to: "/about", About {} }
            // 404 page
            Route { to: "", pages::_404::NotFound {} }
        }
    })
}
