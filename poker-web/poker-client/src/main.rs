#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use poker::{Hand, DECK_OF_CARDS};

use crate::pages::{Cards, History, PokerGame};

mod pages;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App)
}

fn App(cx: Scope) -> Element {
    render!( Router::<Route> {} )
}

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
pub(crate) enum Route {
    #[layout(Main)]
        #[route("/")]
        History {},
        #[route("/cards")]
        Cards {}
}

struct OurHand(Hand);
struct OpponentHand(Hand);
struct RemainHand(Hand);

fn Main(cx: Scope) -> Element {
    use_shared_state_provider(cx, || RemainHand(DECK_OF_CARDS));
    use_shared_state_provider(cx, || OurHand(Hand::default()));
    use_shared_state_provider(cx, || OpponentHand(Hand::default()));

    render!(
        main { class: "h-screen bg-cover bg-white dark:bg-gray-600",
            section { class: "flex h-full w-full justify-center container m-auto mt-6 gap-8",
                div { class: "grow", PokerGame {} }
                div { class: "grow-0 shrink", Outlet::<Route> {} }
            }
        }
    )
}
