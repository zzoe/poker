#![windows_subsystem = "windows"]
#![allow(non_snake_case)]

use dioxus::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use dioxus_desktop::{tao::dpi::LogicalPosition, LogicalSize, WindowBuilder};
use dioxus_router::prelude::Router;

mod pages;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    dioxus_desktop::launch_cfg(
        App,
        dioxus_desktop::Config::new()
            .with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string())
            .with_window(
                WindowBuilder::new()
                    .with_title("Poker")
                    .with_position(LogicalPosition::new(88, 88))
                    .with_inner_size(LogicalSize::new(880, 800)),
            ),
    );

    #[cfg(target_arch = "wasm32")]
    {
        wasm_logger::init(wasm_logger::Config::default());
        dioxus_web::launch(App)
    }
}

fn App(cx: Scope) -> Element {
    render!(Router::<pages::Route> {})
}
