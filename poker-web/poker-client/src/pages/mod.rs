mod cards;
mod history;
mod starter;

use cards::{CardUI, Cards};
use history::{HisHand, History};
use starter::PokerGame;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use poker::{Hand, DECK_OF_CARDS};

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
pub(crate) enum Route {
    #[layout(Main)]
        #[route("/play")]
        History {},
        #[route("/")]
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
    use_shared_state_provider(cx, || GameState::OpponentHandEditing);
    use_shared_state_provider(cx, || RemainHand(DECK_OF_CARDS));
    use_shared_state_provider(cx, || OurHand(Hand::default()));
    use_shared_state_provider(cx, || OpponentHand(Hand::default()));
    use_shared_state_provider(cx, || Vec::<HisHand>::new());

    render!(
        main { class: "h-screen bg-cover bg-white dark:bg-gray-600 p-6",
            section { class: "flex h-full w-full justify-center m-0 space-x-4",
                PokerGame {}
                Outlet::<Route> {}
            }
        }
    )
}
