use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons;
use dioxus_free_icons::Icon;

use crate::hooks::mode::{is_dark, mode};

pub fn Footer(cx: Scope) -> Element {
    log::info!("dark mode: {:?}", is_dark(cx));

    cx.render(rsx! {
        div { class: "mt-6 flex space-x-4 absolute bottom-0 left-0",
            a {
                class: "text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                href: "javascript:;",
                onclick: move |_| {
                    mode(cx, !is_dark(cx));
                    cx.needs_update();
                },

                if is_dark(cx) {
                    cx.render(rsx!{
                        Icon { width: 26, icon: fa_solid_icons::FaSun }
                    })
                }else{
                    cx.render(rsx!{
                        Icon { width: 26, icon: fa_solid_icons::FaMoon }
                    })
                }
            }
        }
    })
}
