use crate::pages::Card;
use dioxus::prelude::*;
use dioxus_router::use_router;


pub fn PokerGame(cx: Scope) -> Element {
    let router = use_router(cx);

    cx.render(rsx! {
        div { class: "flex flex-col space-y-6",
            div { class: "flex flex-row space-x-1 h-16 items-center",
                label { "对方手牌：" }
                div {
                    class: "flex shadow grow h-full items-center px-1",
                    id: "hands_of_player1",
                    onclick: |_| {
                        router.navigate_to("/cards");
                        log::info!("sdf");
                    },
                    Card { value: 1 }
                    Card { value: 2 }
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
                    class: "flex shadow grow h-full items-center px-1",
                    id: "hands_of_player2",
                    onclick: |_| {
                        router.navigate_to("/cards");
                    },
                    Card { value: 2 }
                    Card { value: 2 }
                    Card { value: 2 }
                    Card { value: 2 }
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
                button { class: "w-32 py-2 px-4 bg-blue-500 text-white font-semibold rounded-lg shadow-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-400 focus:ring-opacity-75",
                    "开始/重开"
                }
                button { class: "w-32 py-2 px-4 bg-gray-500 text-white font-semibold rounded-lg shadow-md hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-opacity-75",
                    "悔一步"
                }
            }
        }
    })
}
