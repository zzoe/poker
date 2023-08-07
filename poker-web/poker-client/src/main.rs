#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::Route;
use dioxus_router::Router;
use dioxus_toast::{ToastFrame, ToastManager};
use fermi::{use_atom_ref, use_init_atom_root, AtomRef};
use pages::DeckOfCards;

use crate::components::Footer;
use crate::hooks::mode::init_mode_info;
use crate::pages::{Cards, PokerGame, History};

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
    use_shared_state_provider(cx, || DeckOfCards::default());

    cx.render(rsx! {
        // dioxus toast manager init
        ToastFrame { manager: use_atom_ref(cx, TOAST_MANAGER) }

        Router { 
            main { class: "h-screen bg-cover bg-white dark:bg-gray-600",
                section { class: "grid grid-cols-5 h-full w-full justify-center container m-auto mt-6 gap-8",
                    div { class: "container col-span-3", PokerGame {} }
                    div { class: "container col-span-2",
                        Route { to: "/cards", Cards {} }
                        Route { to: "/", History {} }
                    }

                    Footer {}
                }
            }
        }
    })
}
