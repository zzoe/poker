use dioxus::prelude::*;

pub fn History(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "min-w-53 ml-8", label { "历史出牌:" } }
    })
}
