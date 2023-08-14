use crate::pages::CardUI;
use crate::pages::{GameState, OpponentHand, OurHand, Route};
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;

use super::RemainHand;

pub fn PokerGame(cx: Scope) -> Element {
    let nav = use_navigator(cx);
    let our_hand = use_shared_state::<OurHand>(cx).unwrap();
    let opponent_hand = use_shared_state::<OpponentHand>(cx).unwrap();
    let remain_hand = use_shared_state::<RemainHand>(cx).unwrap();
    let game_state = use_shared_state::<GameState>(cx).unwrap();
    let (our_border, opponent_border) = match *game_state.read() {
        GameState::OurHandEditing => ("border-4 border-blue-200", ""),
        GameState::OpponentHandEditing => ("", "border-4 border-blue-200"),
        _ => ("", ""),
    };
    let our_cards = our_hand.read().0.map(|c| {
        let on_click = move |_| {
            our_hand.write().0.remove(c);
            remain_hand.write().0.insert(c);
        };
        rsx!(CardUI {
            key: "{u64::from(c)}",
            suit_card: c,
            containing: true,
            on_click: on_click
        })
    });
    let opponent_cards = opponent_hand.read().0.map(|c| {
        let on_click = move |_| {
            opponent_hand.write().0.remove(c);
            remain_hand.write().0.insert(c);
        };
        rsx!(CardUI {
            key: "{u64::from(c)}",
            suit_card: c,
            containing: true,
            on_click: on_click
        })
    });

    cx.render(rsx! {
        div { class: "flex flex-col space-y-6",
            div { class: "flex flex-row space-x-1 h-16 items-center",
                label { "对方手牌：" }
                div {
                    class: "flex shadow grow h-full items-center px-1 {our_border}",
                    id: "hands_of_player1",
                    onclick: |_| {
                        nav.push("/cards");
                        *game_state.write() = GameState::OurHandEditing;
                    },
                    our_cards
                }
                input {
                    class: "peer/player1",
                    id: "turn_of_player1",
                    r#type: "radio",
                    name: "turn"
                }
                label { r#for: "turn_of_player1", "对方先手" }
            }

            div { class: "flex flex-row space-x-1 h-16 items-center",
                label { "我方手牌：" }
                div {
                    class: "flex shadow grow h-full items-center px-1 {opponent_border}",
                    id: "hands_of_player2",
                    onclick: |_| {
                        nav.push("/cards");
                        *game_state.write() = GameState::OpponentHandEditing;
                    },
                    opponent_cards
                }
                input {
                    class: "peer/player2",
                    id: "turn_of_player2",
                    r#type: "radio",
                    name: "turn",
                    checked: true
                }
                label { r#for: "turn_of_player2", "我方先手" }
            }

            div { class: "flex justify-center space-x-8",
                button { class: "w-32 py-2 px-4 bg-red-500 text-white font-semibold rounded-lg shadow-md hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-400 focus:ring-opacity-75",
                    "清空"
                }
                button {
                    class: "w-32 py-2 px-4 bg-blue-500 text-white font-semibold rounded-lg shadow-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-400 focus:ring-opacity-75",
                    onclick: |_| {
                        nav.push(Route::History {});
                    },
                    "开始/重开"
                }
                button { class: "w-32 py-2 px-4 bg-gray-500 text-white font-semibold rounded-lg shadow-md hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-opacity-75",
                    "悔一步"
                }
            }
        }
    })
}
