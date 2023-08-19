use crate::pages::CardUI;
use crate::pages::{GameState, OpponentHand, OurHand, Route};
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use indextree::NodeId;
use poker::{Game, Hand, State, DECK_OF_CARDS};

use super::history::HisHand;
use super::RemainHand;

pub fn PokerGame(cx: Scope) -> Element {
    let nav = use_navigator(cx);
    let our_hand = use_shared_state::<OurHand>(cx).unwrap();
    let opponent_hand = use_shared_state::<OpponentHand>(cx).unwrap();
    let remain_hand = use_shared_state::<RemainHand>(cx).unwrap();
    let game_state = use_shared_state::<GameState>(cx).unwrap();
    let his_hand = use_shared_state::<Vec<HisHand>>(cx).unwrap();

    let game = use_state::<Option<Game>>(cx, || None);
    let init_turn = use_state(cx, || 0_u8);
    let current_node_id = use_state::<Option<NodeId>>(cx, || None);
    let our_played_hand = use_state(cx, || Hand::default());
    let opponent_played_hands = use_state(cx, || Vec::<(NodeId, Hand)>::new());
    let init_hand = use_state(cx, || (Hand::default(), Hand::default(), 0_u8));
    let no_solution = use_state(cx, || false);

    // 根据游戏状态,控制手牌选择框的样式,和出牌区域的隐藏状态
    let (our_outline, opponent_outline, playing_hidden) = match *game_state.read() {
        GameState::OurHandEditing => ("outline-blue-600", "", "hidden"),
        GameState::OpponentHandEditing => ("", "outline-blue-600", "hidden"),
        GameState::Playing if *no_solution.get() => ("", "", "hidden"),
        _ => ("", "", ""),
    };
    let no_solution_hidden = no_solution.then(|| "").unwrap_or("hidden");

    // 我方手牌展示
    let our_cards = our_hand.read().0.map(|c| {
        let on_click = move |_| {
            our_hand.write().0.remove_suit_card(c);
            remain_hand.write().0.insert_suit_card(c);
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
            opponent_hand.write().0.remove_suit_card(c);
            remain_hand.write().0.insert_suit_card(c);
        };
        rsx!(CardUI {
            key: "{u64::from(c)}",
            suit_card: c,
            containing: true,
            on_click: on_click
        })
    });

    // 出牌
    let play_next = || {
        // log::debug!("begin play");
        if let Some(inner_game) = game.current().as_ref() {
            // log::debug!("play next");
            let (our_played_cards, opponent_choice_cards) =
                inner_game.action(*current_node_id.current());

            let mut our_played = Hand::default();
            let hand = &mut our_hand.write().0;
            for card in our_played_cards {
                if let Some(suit_card) = hand.play_card(card) {
                    our_played.insert_suit_card(suit_card);
                    remain_hand.write().0.insert_suit_card(suit_card);
                }
            }
            if !our_played.is_empty() {
                his_hand.write().push(HisHand::our(our_played));
            }
            our_played_hand.set(our_played);

            let mut opponent_played = Vec::new();
            for (node_id, opponent_played_cards) in opponent_choice_cards {
                let mut opponent_played_hand = Hand::default();
                let mut temp_hand = opponent_hand.read().0;
                for card in opponent_played_cards {
                    if let Some(suit_card) = temp_hand.play_card(card) {
                        opponent_played_hand.insert_suit_card(suit_card);
                    }
                }
                opponent_played.push((node_id, opponent_played_hand));
            }
            opponent_played_hands.set(opponent_played);
        }
    };

    // 开始/重开
    let start_game = move |_| {
        *his_hand.write() = Vec::new();
        let mut inner_game_state = game_state.write();
        match *inner_game_state {
            GameState::Playing => {
                // log::debug!("重开");
                //状态
                *inner_game_state = GameState::OpponentHandEditing;
                //路由
                nav.replace(Route::Cards {});

                // 恢复手牌
                let (old_our, old_opponent, _) = *init_hand.current();
                our_hand.write().0 = old_our;
                opponent_hand.write().0 = old_opponent;

                // 恢复剩余牌堆
                let mut remain = DECK_OF_CARDS;
                remain.remove_hand(old_our);
                remain.remove_hand(old_opponent);
                remain_hand.write().0 = remain;

                // 是否有解
                no_solution.set(false);

                //初始节点
                if let Some(inner_game) = game.current().as_ref() {
                    current_node_id.set(Some(inner_game.root));
                }
            }
            _ => {
                // log::debug!("开始");
                *inner_game_state = GameState::Playing;
                nav.replace(Route::History {});
                let game_pass;

                // 校验初始游戏状态是否发生实质性变化（无视花色的手牌和先手比较）
                let our_suit_hand = our_hand.read().0;
                let opponent_suit_hand = opponent_hand.read().0;
                let current_turn = *init_turn.current();
                let (old_our, old_opponent, old_turn) = *init_hand.get();

                let new_state =
                    match State::new(vec![our_suit_hand, opponent_suit_hand], current_turn) {
                        Ok(s) => s,
                        Err(e) => {
                            log::error!("创建游戏失败: {e}");
                            return;
                        }
                    };

                if new_state != State::new(vec![old_our, old_opponent], old_turn).unwrap() {
                    let new_game = Game::from_state(new_state);
                    current_node_id.set(Some(new_game.root));
                    game_pass = new_game.pass();
                    game.set(Some(new_game));
                } else {
                    game_pass = game.get().as_ref().unwrap().pass();
                    log::trace!("初始状态没有实质性变化，直接继续");
                }
                init_hand.set((our_suit_hand, opponent_suit_hand, current_turn));

                if game_pass {
                    //出牌
                    play_next();
                } else {
                    no_solution.set(true);
                }
            }
        };
    };

    // 对方出牌的展示
    let opponent_played_ui = opponent_played_hands
        .get()
        .iter()
        .map(|(node_id, played_hand)| {
            let on_click = move |_| {
                current_node_id.set(Some(*node_id));
                opponent_hand.write().0.remove_hand(*played_hand);
                remain_hand.write().0.insert_hand(*played_hand);
                if !played_hand.is_empty(){
                    his_hand.write().push(HisHand::opponent(*played_hand));
                }
                play_next();
            };

            let key = played_hand.value();
            if played_hand.is_empty() {
                rsx!(
                    div {
                        key: "{key}",
                        class: "flex flex-row flex-wrap shadow min-w-full h-14 pr-2 pb-2 justify-center rounded-xl items-center bg-blue-100",
                        style: "font-family: 楷体",
                        onclick: on_click,
                        "不要"
                    }
                )
            } else {
                rsx!(
                    div {
                        key: "{key}",
                        class: "flex flex-row flex-wrap shadow min-w-full pr-2 pb-2 justify-center rounded-xl bg-blue-100",
                        onclick: on_click,
                        for c in played_hand {
                            CardUI { key: "played_{u64::from(c)}", suit_card: c, containing: true }
                        }
                    }
                )
            }
        });

    cx.render(rsx! {
        div { class: "grow flex flex-col space-y-6 p-1",
            div { class: "flex flex-row space-x-2 min-h-16 items-center",
                label { class: "whitespace-nowrap", "对方手牌：" }
                div {
                    class: "flex flex-wrap shadow grow min-w-88 h-full pr-2 pb-2 justify-center rounded-xl outline-none hover:outline-blue-400 {opponent_outline} bg-blue-100",
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
                    class: "flex flex-wrap shadow grow min-w-88 h-full pr-2 pb-2 justify-center rounded-xl outline-none hover:outline-blue-400 {our_outline} bg-green-100",
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

            div { class: "flex justify-evenly space-x-8",
                button {
                    class: "w-32 py-2 px-4 bg-red-500 text-white font-semibold rounded-lg shadow-md hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-400 focus:ring-opacity-75",
                    onclick: |_| {
                        nav.replace(Route::Cards {});
                        opponent_hand.write().0 = Hand::default();
                        our_hand.write().0 = Hand::default();
                        remain_hand.write().0 = DECK_OF_CARDS;
                        *his_hand.write() = Vec::new();
                        *game_state.write() = GameState::OpponentHandEditing;
                        game.set(None);
                    },
                    "清空"
                }
                button {
                    class: "w-32 py-2 px-4 bg-blue-500 text-white font-semibold rounded-lg shadow-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-400 focus:ring-opacity-75",
                    disabled: (opponent_hand.read().0.is_empty() || our_hand.read().0.is_empty())
    && *game_state.read() != GameState::Playing,
                    onclick: start_game,
                    if *game_state.read() == GameState::Playing{
                        "重开"
                    }else{
                        "开始"
                    }
                }
            }

            div {
                class: "flex flex-row flex-wrap shadow min-w-full pr-2 pb-2 justify-center rounded-xl bg-red-100 items-center {no_solution_hidden}",
                style: "font-family: 楷体",
                h1 { class: "text-9xl text-center", "无解" }
            }

            div { class: "flex flex-col space-y-6 {playing_hidden}",
                // label { class: "whitespace-nowrap", "我方出牌：" }
                if our_played_hand.get().is_empty(){
                    rsx!( div { class: "flex flex-row flex-wrap shadow min-w-full h-14 pr-2 pb-2 justify-center rounded-xl bg-green-100 items-center",
                        style: "font-family: 楷体",
                        "不要"
                    })
                }else{
                    rsx!( div { class: "flex flex-row flex-wrap shadow min-w-full pr-2 pb-2 justify-center rounded-xl bg-green-100",
                        for c in our_played_hand.get() {
                            CardUI { key: "played_{u64::from(c)}", suit_card: c, containing: true }
                        }
                    })
                }

                opponent_played_ui
            }
        }
    })
}
