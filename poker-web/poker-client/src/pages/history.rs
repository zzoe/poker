use crate::pages::CardUI;
use dioxus::prelude::*;
use poker::Hand;

#[derive(Clone, Copy)]
pub struct HisHand {
    turn: bool,
    hand: Hand,
}

impl HisHand {
    pub fn our(hand: Hand) -> Self {
        HisHand { turn: false, hand }
    }

    pub fn opponent(hand: Hand) -> Self {
        HisHand { turn: true, hand }
    }
}

pub fn History(cx: Scope) -> Element {
    let his_hand = use_shared_state::<Vec<HisHand>>(cx).unwrap();

    let exact_his_hands = his_hand.read().clone();
    let hands_ui = exact_his_hands.iter().map(|his_hand| {
        let bg_color = if his_hand.turn {
            "bg-blue-100"
        } else {
            "bg-green-100"
        };

        rsx!(
            div { class: "flex flex-row flex-wrap shadow w-full pr-2 pb-2 justify-center rounded-xl {bg_color}",
                for c in his_hand.hand {
                    CardUI { key: "his_{u64::from(c)}", suit_card: c, containing: true }
                }
            }
        )
    });

    cx.render(rsx!(div {
        class: "flex flex-col",
        hands_ui
    }))
}
