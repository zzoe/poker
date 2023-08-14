mod cards;
mod history;
mod starter;

pub use cards::*;
pub use history::History;
pub use starter::PokerGame;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use poker::{Hand, DECK_OF_CARDS};

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
pub(crate) enum Route {
    #[layout(Main)]
        #[route("/")]
        History {},
        #[route("/cards")]
        Cards {}
}

#[derive(PartialEq, Eq)]
enum GameState {
    OurHandEditing,
    OpponentHandEditing,
    Playing,
}

struct OurHand(Hand);
struct OpponentHand(Hand);
struct RemainHand(Hand);

fn Main(cx: Scope) -> Element {
    use_shared_state_provider(cx, || GameState::Playing);
    use_shared_state_provider(cx, || RemainHand(DECK_OF_CARDS));
    use_shared_state_provider(cx, || OurHand(Hand::default()));
    use_shared_state_provider(cx, || OpponentHand(Hand::default()));

    render!(
        main { class: "h-screen bg-cover bg-white dark:bg-gray-600",
            section { class: "flex h-full w-full justify-center container m-auto mt-6 gap-8",
                div { class: "grow container", PokerGame {} }
                // PokerGame{}
                div { class: "shrink", Outlet::<Route> {} }
            }
        }
    )
}
