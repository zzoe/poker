use dioxus::prelude::*;
use dioxus_free_icons::icons::{fa_brands_icons, fa_solid_icons};
use dioxus_free_icons::Icon;
use dioxus_router::Link;

use crate::hooks::mode::{is_dark, mode};

pub fn Footer(cx: Scope) -> Element {
    log::info!("dark mode: {:?}", is_dark(cx));

    cx.render(rsx! {
        div { class: "mt-6 flex space-x-4 justify-center",
            Link {
                class: "text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                to: "/",
                Icon { width: 26, icon: fa_solid_icons::FaHouse }
            }
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
            Link {
                class: "text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                to: "/about",
                Icon { width: 26, icon: fa_solid_icons::FaBook }
            }
            label { class: "text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                // href: "https://github.com/mrxiaozhuox/dioxus-starter",
                Icon { width: 26, icon: fa_brands_icons::FaGithub }
            }
        }
    })
}
