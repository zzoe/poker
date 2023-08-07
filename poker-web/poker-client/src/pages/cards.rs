use dioxus::prelude::*;

/// 用u64表示一副牌，每16位代表一个花色，分别是桃仙梅方；用后15位分别表示大王、小王、2、A、K、Q、J、10、9、8、7、6、5、4、3
pub struct DeckOfCards(u64);

impl Default for DeckOfCards{
    fn default() -> Self {
        DeckOfCards(0b0001111111111111000111111111111100011111111111110111111111111111)
    }
}

pub fn Cards(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "grid grid-col-4 h-full w-full items-center justify-center container mx-auto px-8",
            
        }
    })
}

#[derive(PartialEq, Props)]
pub struct CardProps {
    value: u64,
}

pub fn Card(cx: Scope<CardProps>) -> Element {
    let card = cx.props.value.to_string();

    cx.render(rsx! { div { class: "flex shadow mx-1 py-1 items-center", 
    div{ class: "p-1",
        "{card}♠" }} })
}
