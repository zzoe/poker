use crate::pages::CardUI;
use crate::pages::{GameState, OpponentHand, OurHand, Route};
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use poker::{Game, Hand, DECK_OF_CARDS};

use super::RemainHand;

pub fn PokerGame(cx: Scope) -> Element {
    let nav = use_navigator(cx);
    let our_hand = use_shared_state::<OurHand>(cx).unwrap();
    let opponent_hand = use_shared_state::<OpponentHand>(cx).unwrap();
    let remain_hand = use_shared_state::<RemainHand>(cx).unwrap();
    let game_state = use_shared_state::<GameState>(cx).unwrap();
    let game = use_shared_state::<Option<Game>>(cx).unwrap();
    let init_turn = use_state(cx, || 0_u8);
    let has_no_solution = use_state(cx, || false);

    // 根据状态控制手牌选择框的样式
    let (our_outline, opponent_outline) = match *game_state.read() {
        GameState::OurHandEditing => ("outline-blue-600", ""),
        GameState::OpponentHandEditing => ("", "outline-blue-600"),
        _ => ("", ""),
    };

    // 我方手牌展示
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
    // 对方手牌展示
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
        div { class: "grow flex flex-col space-y-6",
            // TODO 这两个可以合到一起
            div { class: "flex flex-row space-x-2 min-h-16 items-center",
                label { class: "whitespace-nowrap", "对方手牌：" }
                div {
                    class: "flex flex-wrap shadow grow h-full pr-2 pb-2 rounded-xl outline-none hover:outline-blue-400 {opponent_outline}",
                    id: "hands_of_player1",
                    onclick: |_| {
                        nav.replace(Route::Cards {});
                        *game_state.write() = GameState::OpponentHandEditing;
                    },
                    opponent_cards
                }
                input {
                    class: "peer/player1",
                    id: "turn_of_player1",
                    r#type: "radio",
                    name: "turn",
                    checked: *init_turn == 1,
                    onclick: |_| init_turn.set(1)
                }
                label { class: "whitespace-nowrap", r#for: "turn_of_player1", "对方先手" }
            }

            div { class: "flex flex-row space-x-2 min-h-16 items-center",
                label { class: "whitespace-nowrap", "我方手牌：" }
                div {
                    class: "flex flex-wrap shadow grow h-full pr-2 pb-2 rounded-xl outline-none hover:outline-blue-400 {our_outline}",
                    id: "hands_of_player2",
                    onclick: |_| {
                        nav.replace(Route::Cards {});
                        *game_state.write() = GameState::OurHandEditing;
                    },
                    our_cards
                }
                input {
                    class: "peer/player2",
                    id: "turn_of_player2",
                    r#type: "radio",
                    name: "turn",
                    checked: *init_turn == 0,
                    onclick: |_| init_turn.set(0)
                }
                label { class: "whitespace-nowrap", r#for: "turn_of_player2", "我方先手" }
            }

            div { class: "flex justify-center space-x-8",
                button {
                    class: "w-32 py-2 px-4 bg-red-500 text-white font-semibold rounded-lg shadow-md hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-400 focus:ring-opacity-75",
                    onclick: |_| {
                        nav.replace(Route::Cards {});
                        opponent_hand.write().0 = Hand::default();
                        our_hand.write().0 = Hand::default();
                        remain_hand.write().0 = DECK_OF_CARDS;
                        *game_state.write() = GameState::OpponentHandEditing;
                        *game.write() = None;
                    },
                    "清空"
                }
                button {
                    class: "w-32 py-2 px-4 bg-blue-500 text-white font-semibold rounded-lg shadow-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-400 focus:ring-opacity-75",
                    onclick: |_| {
                        match Game::new(vec![our_hand.read().0, opponent_hand.read().0], *init_turn.get()) {
                            Ok(mut new_game) => {
                                nav.replace(Route::History {});
                                *game_state.write() = GameState::Playing;
                                new_game.play();
                                log::debug!("pass: {}", new_game.pass());
                                new_game.print();
                                has_no_solution.set(!new_game.pass());
                                *game.write() = Some(new_game);
                            }
                            Err(e) => log::error!("创建游戏失败: {e}"),
                        }
                    },
                    "开始/重开"
                }
                button { class: "w-32 py-2 px-4 bg-gray-500 text-white font-semibold rounded-lg shadow-md hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-opacity-75",
                    "悔一步"
                }
            }
            div { class: "flex justify-center space-x-8",
                if *has_no_solution.get() { "无解" }else{""}
            }
        }
    })
}
