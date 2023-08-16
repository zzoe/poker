use crate::pages::CardUI;
use crate::pages::{GameState, OpponentHand, OurHand, Route};
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use indextree::NodeId;
use poker::{Game, Hand, DECK_OF_CARDS};

use super::RemainHand;

pub fn PokerGame(cx: Scope) -> Element {
    let nav = use_navigator(cx);
    let our_hand = use_shared_state::<OurHand>(cx).unwrap();
    let opponent_hand = use_shared_state::<OpponentHand>(cx).unwrap();
    let remain_hand = use_shared_state::<RemainHand>(cx).unwrap();
    let game_state = use_shared_state::<GameState>(cx).unwrap();
    let game = use_state::<Option<Game>>(cx, || None);
    let init_turn = use_state(cx, || 0_u8);
    let node_id = use_state::<Option<NodeId>>(cx, || None);

    // 根据状态控制手牌选择框的样式和出牌区域的隐藏状态
    let (our_outline, opponent_outline, playing_hidden) = match *game_state.read() {
        GameState::OurHandEditing => ("outline-blue-600", "", "hidden"),
        GameState::OpponentHandEditing => ("", "outline-blue-600", "hidden"),
        _ => ("", "", ""),
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

    //"无解提示"是否隐藏
    let mut no_solution_hidden = "hidden";
    let mut our_played_hand = Hand::default();
    let mut opponent_choice = Vec::new();
    if let Some(exact_game) = game.get().as_ref() {
        if !exact_game.pass() {
            //无解
            no_solution_hidden = "";
        } else {
            //有解，解析展示数据
            let mut exact_node_id = node_id.get().unwrap_or(exact_game.root);
            if let Some(state) = exact_game.arena.get(exact_node_id) {
                //我方出牌的状态
                if state.get().turn() == 0 {
                    // 从下一个节点取我方的action
                    if let Some(n) = exact_node_id.children(&exact_game.arena).next() {
                        exact_node_id = n;
                    }
                    if let Some(next_state) = exact_game.arena.get(exact_node_id) {
                        let our_hand = &mut our_hand.write().0;
                        for card in next_state.get().action_cards() {
                            if let Some(suit_card) = our_hand.play_card(card) {
                                our_played_hand.insert(suit_card);
                            }
                        }
                    }
                }

                // 对方出牌的状态
                for n in exact_node_id.children(&exact_game.arena) {
                    log::debug!("node: {n}");
                    if let Some(next_state) = exact_game.arena.get(n) {
                        let mut opponent_hand = opponent_hand.read().0;
                        let mut opponent_played = Hand::default();
                        for card in next_state.get().action_cards() {
                            if let Some(suit_card) = opponent_hand.play_card(card) {
                                opponent_played.insert(suit_card);
                            }
                        }
                        opponent_choice.push((n, opponent_played));
                    }
                }
            }
        }
    }

    // 我方打出的手牌
    let our_played_cards = our_played_hand.map(|c| {
        rsx!(CardUI {
            key: "played_{u64::from(c)}",
            suit_card: c,
            containing: true
        })
    });

    // // 对方可以选择打出的所有手牌
    // let opponent_played_hands =
    //     opponent_choice
    //         .iter()
    //         .map(|(played_node_id, opponent_played_hand)| {
    //             let opponent_played_cards = opponent_played_hand.map(|c| {
    //                 rsx!(CardUI {
    //                     key: "{u64::from(c)}",
    //                     suit_card: c,
    //                     containing: true
    //                 })
    //             });

    //             let on_click = |_| node_id.set(Some(*played_node_id));

    //             rsx!(div {
    //                 class: "flex flex-row flex-wrap shadow grow pr-2 pb-2 rounded-xl",
    //                 onclick: on_click,
    //                 opponent_played_cards
    //             })
    //         });

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
                        game.set(None);
                    },
                    "清空"
                }
                button {
                    class: "w-32 py-2 px-4 bg-blue-500 text-white font-semibold rounded-lg shadow-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-400 focus:ring-opacity-75",
                    disabled: opponent_hand.read().0.is_empty() || our_hand.read().0.is_empty(),
                    onclick: |_| {
                        match Game::new(vec![our_hand.read().0, opponent_hand.read().0], *init_turn.get()) {
                            Ok(mut new_game) => {
                                nav.replace(Route::History {});
                                *game_state.write() = GameState::Playing;
                                new_game.play();
                                node_id.set(Some(new_game.root));
                                game.set(Some(new_game));
                            }
                            Err(e) => log::error!("创建游戏失败: {e}"),
                        }
                    },
                    "开始/重开"
                }
                button {
                    class: "w-32 py-2 px-4 bg-gray-500 text-white font-semibold rounded-lg shadow-md hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-opacity-75",
                    disabled: *game_state.read() != GameState::Playing,
                    "悔一步"
                }
            }

            h1 { class: "text-9xl text-center {no_solution_hidden}", "无解" }

            div { class: "flex flex-row space-x-2 min-h-16 items-center {playing_hidden}",
                label { class: "whitespace-nowrap", "我方出牌：" }
                div { class: "flex flex-row flex-wrap shadow grow pr-2 pb-2 rounded-xl",
                    our_played_cards
                }
                for (played_node_id , played_hand) in opponent_choice {

                    rsx!(div {
                    class: "flex flex-row flex-wrap shadow grow pr-2 pb-2 rounded-xl",
                    onclick: |_| node_id.set(Some(played_node_id.clone())),
                    for c in played_hand{
                    rsx!(CardUI {
                        key: "{u64::from(c)}",
                        suit_card: c,
                        containing: true
                    })
                    }
                })
                }
            }
        }
    })
}
