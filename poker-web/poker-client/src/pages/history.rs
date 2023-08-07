use dioxus::prelude::*;

pub fn History(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "contents", label { "历史出牌:" } }
    })
}
