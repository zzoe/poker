use dioxus::prelude::*;
use poker::{Card, SuitCard};

use crate::pages::{GameState, OpponentHand, OurHand, RemainHand};

pub fn Cards(cx: Scope) -> Element {
    let remain_hand = use_shared_state::<RemainHand>(cx).unwrap();
    let our_hand = use_shared_state::<OurHand>(cx).unwrap();
    let opponent_hand = use_shared_state::<OpponentHand>(cx).unwrap();
    let game_state = use_shared_state::<GameState>(cx).unwrap();

    let card_click = |suit_card: SuitCard| {
        move |_| {
            match *game_state.read() {
                GameState::OurHandEditing => our_hand.write().0.insert(suit_card),
                GameState::OpponentHandEditing => opponent_hand.write().0.insert(suit_card),
                _ => return,
            }
            remain_hand.write().0.remove(suit_card);
        }
    };

    let cards_except_joker = (0..52)
        .map(|u| {
            (
                u,
                SuitCard::new(Card::from_u16(1 << (u / 4)).unwrap(), (3 - u % 4) as u8),
            )
        })
        .map(|(key, suit_card)| {
            // 回调是作为参数传递给CardUI的，如果直接写在rsx!里面，会导致suit_card值异常
            // let on_click = move |_| {
            //     match *game_state.read() {
            //         GameState::OurHandEditing => our_hand.write().0.insert(suit_card),
            //         GameState::OpponentHandEditing => opponent_hand.write().0.insert(suit_card),
            //         _ => return,
            //     }
            //     remain_hand.write().0.remove(suit_card);
            // };

            rsx!(CardUI {
                key: "{key}",
                suit_card: suit_card,
                containing: remain_hand.read().0.contains(suit_card),
                on_click: card_click(suit_card)
            })
        });

    let black_joker = SuitCard::new(Card::BlackJoker, 0);
    let red_joker = SuitCard::new(Card::RedJoker, 0);

    cx.render(rsx! {
        div { class: "flex-shrink-0 flex flex-wrap justify-evenly ml-8 w-44",
            cards_except_joker,
            div { class: "w-9" }
            CardUI {
                key: "53",
                suit_card: black_joker,
                containing: remain_hand.read().0.contains(black_joker),
                on_click: card_click(SuitCard::new(Card::BlackJoker, 0))
            }
            CardUI {
                key: "54",
                suit_card: red_joker,
                containing: remain_hand.read().0.contains(red_joker),
                on_click: card_click(SuitCard::new(Card::RedJoker, 0))
            }
            div { class: "w-9" }
        }
    })
}

#[derive(Props)]
pub struct CardProps<'a> {
    suit_card: SuitCard,
    #[props(default = true)]
    containing: bool,
    on_click: Option<EventHandler<'a, MouseEvent>>,
}

pub fn CardUI<'a>(cx: Scope<'a, CardProps<'a>>) -> Element {
    let show = |card: Card| match card {
        Card::Ten => "10".to_owned(),
        Card::BlackJoker => "王".to_owned(),
        Card::RedJoker => "王".to_owned(),
        c => c.to_string(),
    };

    // ♠♥♣♦
    let (suit, color, card, card_font) = match cx.props.suit_card {
        SuitCard::Spades(c) => ("♠", "text-black", show(c), "Consolas"),
        SuitCard::Hearts(c) => ("♥", "text-red-500", show(c), "Consolas"),
        SuitCard::Clubs(c) => ("♣", "text-black", show(c), "Consolas"),
        SuitCard::Diamonds(c) if c == Card::RedJoker => ("", "text-red-500", show(c), "楷体"),
        SuitCard::Diamonds(c) if c == Card::BlackJoker => ("", "text-black", show(c), "楷体"),
        SuitCard::Diamonds(c) => ("♦", "text-red-500", show(c), "Consolas"),
    };

    let bg = (!cx.props.containing)
        .then(|| "bg-stone-300")
        .unwrap_or_default();

    cx.render(rsx! {
        div {
            class: "flex relative shadow justify-center items-center ml-2 mt-2 w-9 h-11 text-2xl cursor-default outline outline-amber-200 hover:bg-sky-100 {color} {bg}",
            style: " font-family: {card_font}",
            onclick: move |event| {
                if let Some(on_click) = cx.props.on_click.as_ref() {
                    if cx.props.containing {
                        on_click.call(event)
                    }
                }
            },
            div { class: "absolute top-0 left-0 text-sm {color}", "{suit}" }
            div { class: "absolute bottom-0 right-0 text-sm {color}", "{suit}" }
            "{card}"
        }
    })
}
