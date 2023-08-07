use dioxus::core::ScopeState;
use dioxus_use_storage::use_local_storage;

pub fn is_dark(cx: &ScopeState) -> bool {
    let storage = use_local_storage(cx);
    let current_mode = storage.get("mode");
    matches!(current_mode, Some(m) if m.to_lowercase() == "dark")
}

pub fn mode(cx: &ScopeState, dark: bool) {
    let storage = use_local_storage(cx);
    storage.insert("mode", if dark { "dark" } else { "light" });
    set_mode(dark);
}

pub fn init_mode_info(cx: &ScopeState) {
    set_mode(is_dark(cx));
}

fn set_mode(dark: bool) {
    if dark {
        let _ = js_sys::eval("document.documentElement.classList.add('dark');");
    } else {
        let _ = js_sys::eval("document.documentElement.classList.remove('dark');");
    }
}
